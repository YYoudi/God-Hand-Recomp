"""Synthetic negative cases for the publication guard; no original game data."""
import unittest
from check_public_tree import problems


class PublicTreeTests(unittest.TestCase):
    def test_original_rust_allowed(self):
        self.assertEqual(problems('crates/disc/src/lib.rs', b'pub fn read() {}', set()), [])

    def test_local_or_generated_paths_refused(self):
        for path in ['God Hand (USA).iso', 'recomp/output/main.cpp', 'PS2Recomp/src/test.rs',
                     'docs/local/notes.md', 'scripts/native.ps1', 'docs/prototype-tcrf-notes.md']:
            with self.subTest(path=path):
                self.assertTrue(problems(path, b'synthetic data', set()))

    def test_binary_masquerading_as_text_refused(self):
        self.assertTrue(problems('docs/report.md', b'\x7fELF\0synthetic', set()))

    def test_unregistered_images_refused(self):
        self.assertTrue(problems('docs/dashboard/captures/unknown.jpg', b'\xff\xd8\xff', set()))

    def test_registered_screenshot_requires_correct_magic(self):
        path = 'docs/dashboard/captures/dashboard.jpg'
        self.assertEqual(problems(path, b'\xff\xd8\xffsynthetic', {path}), [])
        self.assertTrue(problems(path, b'not a screenshot', {path}))

    def test_large_file_refused(self):
        self.assertTrue(problems('docs/data.txt', b'a' * (1024 * 1024 + 1), set()))

    def test_credential_shape_refused(self):
        secret = ('gh' + 'p_' + 'X' * 40).encode()
        self.assertTrue(problems('docs/notes.md', secret, set()))


if __name__ == '__main__':
    unittest.main()
