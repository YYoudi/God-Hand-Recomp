"""Check Git's index, never walk the local disc or dependency directories."""
from pathlib import Path, PurePosixPath
import json
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
ROOT_FILES = {'.gitignore', '.gitattributes', 'AGENTS.md', 'README.md', 'LICENSE',
              'NOTICE.md', 'CONTRIBUTING.md', 'ROADMAP.md', 'Cargo.toml', 'Cargo.lock'}
PREFIXES = ('.github/', 'scripts/', 'prompts/', 'docs/', 'crates/', 'specs/')
TEXT = {'.md', '.py', '.ps1', '.sh', '.json', '.yml', '.yaml', '.html', '.css', '.js', '.txt', '.rs', '.toml'}
MAX_SIZE = 1024 * 1024


def problems(path, data, captures):
    errors = []
    p = PurePosixPath(path)
    if path not in ROOT_FILES and not path.startswith(PREFIXES):
        errors.append('path outside public allowlist')
    if path.startswith('docs/local/') or path in {'docs/prototype-tcrf-notes.md', 'scripts/native.ps1', 'scripts/ghidra_analyze_retail.sh', 'docs/toolchains.lock.json'} or (path.startswith('prompts/0') and not path.startswith('prompts/00-')):
        errors.append('local research file')
    if len(data) > MAX_SIZE:
        errors.append('file larger than 1 MiB')
    if p.suffix in {'.png', '.jpg'} and path in captures:
        if not (data.startswith(b'\x89PNG\r\n\x1a\n') if p.suffix == '.png' else data.startswith(b'\xff\xd8\xff')):
            errors.append('invalid dashboard screenshot')
    elif path not in ROOT_FILES and p.suffix not in TEXT and path != 'docs/.nojekyll':
        errors.append('unsupported public file type')
    else:
        try:
            content = data.decode('utf-8-sig')
            if '\0' in content:
                errors.append('binary content in text file')
            # Match secret assignments, not documentation naming an environment variable.
            import re
            if re.search(r'(?:gh[pousr]_[A-Za-z0-9]{30,}|github_pat_[A-Za-z0-9_]{40,}|-----BEGIN (?:RSA |OPENSSH |EC )?PRIVATE KEY-----)', content):
                errors.append('credential-like content')
        except UnicodeDecodeError:
            errors.append('non-UTF-8 content')
    return errors


def main():
    def git(*args):
        return subprocess.check_output(['git', '-C', str(ROOT), *args])
    paths = git('ls-files', '-z').decode().split('\0')
    captures = set()
    manifest = 'docs/dashboard/data.json'
    if manifest in paths:
        data = json.loads(git('show', ':' + manifest))
        for capture in data.get('captures', []):
            if capture.get('kind') != 'dashboard':
                sys.exit('Only dashboard captures may be public.')
            path = 'docs/dashboard/' + capture['path']
            if '..' in PurePosixPath(path).parts or not path.startswith('docs/dashboard/captures/'):
                sys.exit('Invalid capture path.')
            captures.add(path)
    failures = []
    for path in filter(None, paths):
        mode = git('ls-files', '-s', '--', path).split()[0]
        if mode not in (b'100644', b'100755'):
            failures.append(f'{path}: links and embedded repositories forbidden')
            continue
        for error in problems(path, git('show', ':' + path), captures):
            failures.append(f'{path}: {error}')
    if failures:
        sys.exit('\n'.join(failures))
    print(f'Public tree: {len(list(filter(None, paths)))} files checked; no forbidden content detected.')


if __name__ == '__main__':
    main()
