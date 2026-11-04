"""Synthetic screenshots only: no game pixels, discs or viewer required."""
import copy
import hashlib
import json
from pathlib import Path
import struct
import tempfile
import unittest
import zlib

from comparisons import png_dimensions, ppm_pixels, ppm_preview, prepare, validate_comparisons
from dashboard import blog
from check_public_tree import problems


def png(red=0):
    def chunk(tag, payload):
        return struct.pack('>I', len(payload)) + tag + payload + struct.pack('>I', zlib.crc32(tag + payload))
    return (b'\x89PNG\r\n\x1a\n' + chunk(b'IHDR', struct.pack('>IIBBBBB', 2, 1, 8, 6, 0, 0, 0)) +
            chunk(b'IDAT', zlib.compress(bytes([0, red, 0, 0, 255, 0, 0, 0, 255]))) + chunk(b'IEND', b''))


def fixture():
    images, pair = {}, dict(id='synthetic-pair', date='2026-11-04', title='Synthetic comparison',
                            feature='R03', scenario='Original synthetic pixels, not a game',
                            alignment='unmatched', reviewed=True,
                            observations=['Different original synthetic pixels.'], limits=['No fidelity claim.'])
    for side, red in [('reference', 1), ('bevy', 2)]:
        data = png(red)
        digest = hashlib.sha256(data).hexdigest()
        path = f'media/{digest}.png'
        pair[side] = dict(path=path, sha256=digest, width=2, height=1, caption=side,
                          profile='retail-usa', runtime='synthetic fixture', revision='unknown',
                          captured_at='2026-11-04T20:00:00+01:00')
        images['docs/dashboard/' + path] = data
    return pair, images


class ComparisonTests(unittest.TestCase):
    def test_valid_pair_and_original_links(self):
        pair, images = fixture()
        self.assertEqual(validate_comparisons([pair], images.__getitem__), set(images))
        page = blog(dict(updates=[], captures=[], comparisons=[pair], repo='https://example.org/repo'))
        for item in (pair['reference'], pair['bevy']):
            self.assertIn(f'href="{item["path"]}" download', page)
        self.assertIn('Reference / Bevy', page)
        self.assertIn('Alignment: unmatched', page)

    def test_no_fabricated_gallery_when_empty(self):
        page = blog(dict(updates=[], captures=[], comparisons=[], repo='https://example.org/repo'))
        self.assertNotIn('id="comparisons"', page)

    def test_html_text_escaped(self):
        pair, _ = fixture()
        pair['reference']['caption'] = '<script>"unsafe"</script>'
        pair['title'] = '<unsafe>'
        page = blog(dict(updates=[], captures=[], comparisons=[pair], repo='https://example.org/repo'))
        self.assertNotIn('<script>', page)
        self.assertIn('&lt;unsafe&gt;', page)

    def test_hash_and_dimensions_enforced(self):
        for field, value in [('sha256', '0' * 64), ('width', 3), ('height', 2), ('path', '../private.png')]:
            pair, images = fixture()
            pair['reference'][field] = value
            with self.subTest(field=field), self.assertRaises(ValueError):
                validate_comparisons([pair], images.__getitem__)
        pair, images = fixture()
        images['docs/dashboard/' + pair['reference']['path']] = png(3)
        with self.assertRaises(ValueError):
            validate_comparisons([pair], images.__getitem__)

    def test_png_crc_truncation_and_trailing_data(self):
        valid = png()
        self.assertEqual(png_dimensions(valid), (2, 1))
        for data in [b'not PNG', valid[:-1], valid + b'extra', valid[:33] + b'bad',
                     valid[:20] + bytes([valid[20] ^ 1]) + valid[21:]]:
            with self.subTest(length=len(data)), self.assertRaises(ValueError):
                png_dimensions(data)

    def test_metadata_review_and_exact_revision_required(self):
        for side, field, value in [('reference', 'captured_at', '2026-11-04T20:00:00'),
                                   ('bevy', 'revision', 'main'), ('reference', 'profile', 'unspecified')]:
            pair, images = fixture()
            pair[side][field] = value
            with self.subTest(field=field), self.assertRaises(ValueError):
                validate_comparisons([pair], images.__getitem__)
        pair, images = fixture()
        pair['reviewed'] = False
        with self.assertRaises(ValueError):
            validate_comparisons([pair], images.__getitem__)

    def test_duplicate_ids_profiles_and_reused_image_refused(self):
        pair, images = fixture()
        with self.assertRaises(ValueError):
            validate_comparisons([pair, pair], images.__getitem__)
        pair['bevy']['profile'] = 'prototype-june-2006'
        with self.assertRaises(ValueError):
            validate_comparisons([pair], images.__getitem__)
        pair['bevy'] = copy.deepcopy(pair['reference'])
        with self.assertRaises(ValueError):
            validate_comparisons([pair], images.__getitem__)

    def test_unexpected_private_fields_refused(self):
        pair, images = fixture()
        pair['reference']['source'] = 'private-source.png'
        with self.assertRaises(ValueError):
            validate_comparisons([pair], images.__getitem__)

    def test_preparation_preserves_sources_and_original_bytes(self):
        pair, images = fixture()
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / 'recomp/experiments').mkdir(parents=True)
            (root / 'docs/dashboard').mkdir(parents=True)
            manifest = root / 'docs/dashboard/data.json'
            manifest.write_text('{"comparisons": []}', encoding='utf-8')
            for side in ('reference', 'bevy'):
                source = root / 'recomp/experiments' / (side + '.png')
                payload = images['docs/dashboard/' + pair[side]['path']]
                source.write_bytes(payload)
                for field in ('path', 'width', 'height', 'sha256'):
                    pair[side].pop(field)
                pair[side]['source'] = str(source)
            original = copy.deepcopy(pair)
            self.assertEqual(prepare(root, pair), 'synthetic-pair')
            self.assertEqual(pair, original)
            result = json.loads(manifest.read_text())['comparisons'][0]
            for side in ('reference', 'bevy'):
                source = Path(original[side]['source'])
                target = root / 'docs/dashboard' / result[side]['path']
                self.assertEqual(target.read_bytes(), source.read_bytes())
                self.assertNotIn('source', result[side])
                self.assertEqual(problems(str(target.relative_to(root)).replace('\\', '/'),
                                          target.read_bytes(), set(images)), [])
            with self.assertRaises(ValueError):
                prepare(root, pair)

    def test_unregistered_media_stays_forbidden(self):
        pair, images = fixture()
        for path, payload in images.items():
            self.assertTrue(problems(path, payload, set()))

    def test_ppm_preview_preserves_exact_original_pixels(self):
        raw = b'P6\n2 1\n255\n' + bytes([1, 2, 3, 4, 5, 6])
        self.assertEqual(ppm_pixels(raw), (2, 1, bytes([1, 2, 3, 4, 5, 6])))
        preview = ppm_preview(raw)
        self.assertEqual(png_dimensions(preview), (2, 1))
        at, compressed = 8, b''
        while at < len(preview):
            size = struct.unpack_from('>I', preview, at)[0]
            if preview[at + 4:at + 8] == b'IDAT':
                compressed += preview[at + 8:at + 8 + size]
            at += size + 12
        self.assertEqual(zlib.decompress(compressed), b'\0' + raw[-6:])
        for bad in [raw[:-1], raw + b'extra', b'P6\n99999 1\n255\n', b'P3\n2 1\n255\n']:
            with self.assertRaises(ValueError):
                ppm_pixels(bad)

    def test_raw_reference_is_registered_and_labeled_not_called_original_png(self):
        pair, images = fixture()
        raw = b'P6\n2 1\n255\n' + bytes([1, 2, 3, 4, 5, 6])
        digest = hashlib.sha256(raw).hexdigest()
        preview = ppm_preview(raw)
        preview_digest = hashlib.sha256(preview).hexdigest()
        pair['reference'].update(path=f'media/{preview_digest}.png', sha256=preview_digest)
        images['docs/dashboard/' + pair['reference']['path']] = preview
        pair['reference']['original'] = dict(path=f'media/{digest}.ppm', sha256=digest)
        raw_path = 'docs/dashboard/media/' + digest + '.ppm'
        images[raw_path] = raw
        paths = validate_comparisons([pair], images.__getitem__)
        self.assertIn(raw_path, paths)
        self.assertEqual(problems(raw_path, raw, paths), [])
        self.assertTrue(problems(raw_path, raw, set()))
        page = blog(dict(updates=[], captures=[], comparisons=[pair], repo='https://example.org/repo'))
        self.assertIn('Download unchanged original PPM', page)
        self.assertIn('lossless viewing preview', page)
        images[raw_path] = raw[:-1] + b'X'
        with self.assertRaises(ValueError):
            validate_comparisons([pair], images.__getitem__)


if __name__ == '__main__':
    unittest.main()
