"""Validate and prepare explicitly reviewed original reference/Bevy PNG pairs."""
import argparse
import copy
import datetime
import hashlib
import json
from pathlib import Path
import re
import struct
import zlib

MAX_IMAGE_BYTES = 20 * 1024 * 1024
ROOT = Path(__file__).resolve().parents[1]


def require(condition, message):
    if not condition:
        raise ValueError(message)


def png_dimensions(data):
    """Check PNG structure/CRC and dimensions, not rendering or ownership."""
    require(len(data) <= MAX_IMAGE_BYTES and data[:8] == b'\x89PNG\r\n\x1a\n',
            'Expected an original PNG within the 20 MiB limit')
    at, dimensions, image_data = 8, None, False
    while at + 12 <= len(data):
        length = struct.unpack_from('>I', data, at)[0]
        end = at + 12 + length
        require(end <= len(data), 'Truncated PNG chunk')
        tag, payload = data[at + 4:at + 8], data[at + 8:end - 4]
        require(zlib.crc32(tag + payload) == struct.unpack_from('>I', data, end - 4)[0],
                'PNG CRC mismatch')
        if dimensions is None:
            require(tag == b'IHDR' and length == 13, 'PNG must start with IHDR')
            width, height, depth, color, compression, filtering, interlace = struct.unpack('>IIBBBBB', payload)
            require(0 < width <= 8192 and 0 < height <= 8192 and width * height <= 16777216,
                    'Screenshot dimensions outside limit')
            require(depth == 8 and color in (2, 6) and compression == filtering == 0 and interlace in (0, 1),
                    'Expected an 8-bit RGB/RGBA screenshot')
            dimensions = (width, height)
        elif tag == b'IHDR':
            raise ValueError('Duplicate IHDR')
        if tag == b'IDAT':
            image_data = True
        if tag == b'IEND':
            require(length == 0 and image_data and end == len(data), 'Invalid PNG end')
            return dimensions
        at = end
    raise ValueError('Missing PNG end')


def validate_comparisons(entries, read):
    """read receives a repository-relative path (working tree or Git index)."""
    paths, ids = set(), set()
    for pair in entries:
        require(set(pair) == {'id', 'date', 'title', 'feature', 'scenario', 'alignment',
                             'observations', 'limits', 'reviewed', 'reference', 'bevy'},
                'Unexpected or missing comparison fields')
        require(re.fullmatch(r'[a-z0-9][a-z0-9-]{0,79}', pair['id']) and pair['id'] not in ids,
                'Invalid or duplicate comparison id')
        ids.add(pair['id'])
        datetime.date.fromisoformat(pair['date'])
        require(pair['reviewed'] is True, 'Screenshot review required')
        require(pair['alignment'] in {'same-frame', 'same-landmark', 'unmatched'}, 'Invalid alignment')
        for field in ('title', 'feature', 'scenario'):
            require(isinstance(pair[field], str) and pair[field].strip(), 'Missing comparison text')
        for field in ('observations', 'limits'):
            require(isinstance(pair[field], list) and pair[field] and
                    all(isinstance(s, str) and s.strip() for s in pair[field]), 'Missing comparison findings/limits')
        for side in ('reference', 'bevy'):
            item = pair[side]
            require(set(item) == {'path', 'sha256', 'width', 'height', 'caption', 'profile',
                                 'runtime', 'revision', 'captured_at'}, 'Unexpected or missing image fields')
            digest = item['sha256']
            require(isinstance(digest, str) and re.fullmatch('[0-9a-f]{64}', digest), 'Invalid image hash')
            require(item['path'] == f'media/{digest}.png', 'Comparison path must be content-addressed PNG')
            require(item['profile'] in {'retail-usa', 'prototype-june-2006'}, 'Invalid reference profile')
            require(item['revision'] in {'unknown', 'uncommitted'} or
                    re.fullmatch('[0-9a-f]{40}', item['revision']), 'Use exact revision or explicit unknown')
            captured = datetime.datetime.fromisoformat(item['captured_at'].replace('Z', '+00:00'))
            require(captured.tzinfo is not None and captured.utcoffset() is not None, 'Capture timezone required')
            require(all(isinstance(item[k], str) and item[k].strip() for k in ('caption', 'runtime')),
                    'Capture caption/runtime required')
            path = 'docs/dashboard/' + item['path']
            data = read(path)
            require(hashlib.sha256(data).hexdigest() == digest, 'Screenshot hash mismatch')
            require(png_dimensions(data) == (item['width'], item['height']), 'Screenshot dimensions mismatch')
            paths.add(path)
        require(pair['reference']['profile'] == pair['bevy']['profile'],
                'Never silently compare prototype and retail')
        require(pair['reference']['sha256'] != pair['bevy']['sha256'],
                'Reference and Bevy must be separate captures')
    return paths


def prepare(root, entry):
    """Copy only two explicit local sources, byte-for-byte; never sweep a folder."""
    root, pair, pending = Path(root).resolve(), copy.deepcopy(entry), {}
    for side in ('reference', 'bevy'):
        item = pair[side]
        source = Path(item.pop('source')).resolve()
        require(source.is_relative_to(root / 'recomp/experiments') and source.suffix.lower() == '.png',
                'Source must be an explicit private experiment PNG')
        require(source.stat().st_size <= MAX_IMAGE_BYTES, 'Original screenshot exceeds 20 MiB')
        data = source.read_bytes()
        width, height = png_dimensions(data)
        digest = hashlib.sha256(data).hexdigest()
        item.update(path=f'media/{digest}.png', sha256=digest, width=width, height=height)
        pending['docs/dashboard/' + item['path']] = data
    validate_comparisons([pair], pending.__getitem__)
    manifest = root / 'docs/dashboard/data.json'
    data = json.loads(manifest.read_text(encoding='utf-8'))
    require(not any(p['id'] == pair['id'] for p in data.get('comparisons', [])),
            'Existing comparisons are immutable; use a new id')
    for path, payload in pending.items():
        destination = root / path
        destination.parent.mkdir(parents=True, exist_ok=True)
        if destination.exists():
            require(destination.read_bytes() == payload, 'Existing screenshot differs')
        else:
            destination.write_bytes(payload)
    data.setdefault('comparisons', []).insert(0, pair)
    manifest.write_text(json.dumps(data, indent=2, ensure_ascii=False) + '\n', encoding='utf-8')
    return pair['id']


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--entry', type=Path, required=True,
                        help='Private JSON entry with explicit reference/bevy source paths and provenance')
    args = parser.parse_args()
    print('Prepared comparison:', prepare(ROOT, json.loads(args.entry.read_text(encoding='utf-8'))))


if __name__ == '__main__':
    main()
