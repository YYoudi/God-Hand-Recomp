"""Inventory private reference images without executing or extracting game code."""
import argparse
import hashlib
import json
from pathlib import Path
import re


def probe_image(path, with_hash=False):
    path = Path(path).resolve()
    result = {'path': str(path), 'present': path.is_file()}
    if not result['present']:
        return result
    result['bytes'] = path.stat().st_size
    with path.open('rb') as stream:
        stream.seek(16 * 2048)
        header = stream.read(7)
        result['iso9660_at_2048_byte_sectors'] = header == b'\x01CD001\x01'
        if with_hash:
            stream.seek(0)
            digest = hashlib.sha256()
            for chunk in iter(lambda: stream.read(1024 * 1024), b''):
                digest.update(chunk)
            result['sha256'] = digest.hexdigest()
    return result


def inventory(root, with_hash=False, prepare_cue=False):
    root = Path(root).resolve()
    retail = probe_image(root / 'God Hand (USA).iso', with_hash)
    binary = root / 'GH-Prototype/PS2 - Godhand Preview 2006-06-21.bin'
    prototype = probe_image(binary, with_hash)
    cue = binary.with_suffix('.cue')
    cue_status = {'path': str(cue), 'present': cue.is_file()}
    if cue.is_file():
        source = cue.read_bytes()
        if len(source) > 64 * 1024:
            raise ValueError('CUE exceeds the inventory read limit')
        text = source.decode('utf-8-sig')
        names = re.findall(r'^\s*FILE\s+"([^"]+)"\s+BINARY\s*$', text, re.MULTILINE | re.IGNORECASE)
        tracks = re.findall(r'^\s*TRACK\s+(\d+)\s+(\S+)\s*$', text, re.MULTILINE | re.IGNORECASE)
        indexes = re.findall(r'^\s*INDEX\s+(\d+)\s+(\S+)\s*$', text, re.MULTILINE | re.IGNORECASE)
        cue_status['single_mode1_2048_track'] = bool(
            len(names) == 1 and [(n, mode.upper()) for n, mode in tracks] == [('01', 'MODE1/2048')]
            and indexes == [('01', '00:00:00')]
            and not re.search(r'^\s*(?:PREGAP|POSTGAP)\s+', text, re.MULTILINE | re.IGNORECASE))
        if len(names) == 1:
            cue_status['declared_file'] = names[0]
            cue_status['declared_file_exists'] = (cue.parent / names[0]).is_file()
            cue_status['matches_available_binary'] = (cue.parent / names[0]).resolve() == binary.resolve()
        if prepare_cue:
            if not cue_status['single_mode1_2048_track'] or not prototype.get('iso9660_at_2048_byte_sectors'):
                raise ValueError('Cannot prepare CUE: expected one MODE1/2048 image with an ISO descriptor')
            target = (root / 'recomp/references/prototype-local.cue').resolve()
            if not target.is_relative_to((root / 'recomp').resolve()):
                raise ValueError('Generated CUE must remain under private recomp/')
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(f'FILE "{binary.resolve().as_posix()}" BINARY\n  TRACK 01 MODE1/2048\n    INDEX 01 00:00:00\n', encoding='utf-8', newline='\n')
            cue_status['prepared_local_cue'] = str(target)
            assert cue.read_bytes() == source, 'Original CUE changed'
    elif prepare_cue:
        raise ValueError('Original CUE is missing; cannot infer its track layout')
    return {
        'target_profile': 'retail-usa',
        'retail': retail,
        'prototype': prototype,
        'prototype_cue': cue_status,
        'limits': 'Container presence/signature only; not a gameplay test or authenticity certificate. Prototype debug features remain unverified locally.'
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--workspace', type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument('--output', type=Path)
    parser.add_argument('--hash', action='store_true', help='Hash original images read-only; may take time')
    parser.add_argument('--prepare-prototype-cue', action='store_true')
    args = parser.parse_args()
    root = args.workspace.resolve()
    output = (args.output or root / 'docs/local/reference-inventory.json').resolve()
    if not any(output.is_relative_to(root / directory) for directory in ('docs/local', 'recomp')):
        parser.error('Inventory output must stay in private docs/local/ or recomp/')
    report = inventory(root, args.hash, args.prepare_prototype_cue)
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(report, indent=2) + '\n', encoding='utf-8', newline='\n')
    print(f'Private reference inventory: {output}')
    print(f'Retail present: {report["retail"]["present"]}; prototype present: {report["prototype"]["present"]}')
    print(f'Prototype CUE target exists: {report["prototype_cue"].get("declared_file_exists", False)}')


if __name__ == '__main__':
    main()
