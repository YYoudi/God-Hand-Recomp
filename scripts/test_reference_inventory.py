"""Synthetic reference-profile tests; no game files are required."""
from pathlib import Path
import tempfile
import unittest
from reference_inventory import inventory, probe_image


class ReferenceInventoryTests(unittest.TestCase):
    def fixture(self, root, mode='MODE1/2048'):
        folder = root / 'GH-Prototype'
        folder.mkdir()
        data = bytearray(17 * 2048)
        data[16 * 2048:16 * 2048 + 7] = b'\x01CD001\x01'
        binary = folder / 'PS2 - Godhand Preview 2006-06-21.bin'
        binary.write_bytes(data)
        cue = binary.with_suffix('.cue')
        cue.write_text(f'FILE "missing-name.bin" BINARY\n  TRACK 01 {mode}\n    INDEX 01 00:00:00\n')
        return binary, cue

    def test_missing_images_are_reported_without_creating_them(self):
        with tempfile.TemporaryDirectory() as temp:
            report = inventory(Path(temp))
            self.assertFalse(report['retail']['present'])
            self.assertFalse(report['prototype']['present'])
            self.assertEqual(list(Path(temp).iterdir()), [])

    def test_mismatched_cue_fixed_only_in_private_output(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            binary, cue = self.fixture(root)
            original = cue.read_bytes()
            report = inventory(root, prepare_cue=True)
            self.assertFalse(report['prototype_cue']['declared_file_exists'])
            self.assertIn(binary.resolve().as_posix(), Path(report['prototype_cue']['prepared_local_cue']).read_text())
            self.assertEqual(cue.read_bytes(), original)

    def test_unknown_track_layout_is_not_guessed(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            self.fixture(root, mode='MODE2/2352')
            with self.assertRaises(ValueError):
                inventory(root, prepare_cue=True)
            self.assertFalse((root / 'recomp').exists())

    def test_truncated_image_has_no_iso_signature(self):
        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp) / 'small.bin'
            path.write_bytes(b'synthetic')
            self.assertFalse(probe_image(path)['iso9660_at_2048_byte_sectors'])

    def test_multitrack_or_shifted_index_is_not_flattened(self):
        for shifted in (False, True):
            with self.subTest(shifted=shifted), tempfile.TemporaryDirectory() as temp:
                root = Path(temp)
                _, cue = self.fixture(root)
                text = cue.read_text()
                text = text.replace('00:00:00', '00:02:00') if shifted else text + '  TRACK 02 AUDIO\n    INDEX 01 01:00:00\n'
                cue.write_text(text)
                with self.assertRaises(ValueError):
                    inventory(root, prepare_cue=True)
                self.assertFalse((root / 'recomp').exists())


if __name__ == '__main__':
    unittest.main()
