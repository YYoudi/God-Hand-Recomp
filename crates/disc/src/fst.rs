//! Original bounded namespace reader. See specs/fst-catalog-v1.md for provenance.
use crate::afs::AfsCatalog;
use std::collections::HashSet;
use std::io::{self, Read};

const MAX_BYTES: u64 = 16 * 1024 * 1024;
const MAX_RECORDS: usize = 65_536;

fn invalid(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

#[derive(Debug, PartialEq, Eq)]
pub struct FstFile {
    pub path: String,
    pub ordinal: u32,
    /// Unknown metadata; never used as a payload size or executable instruction.
    pub opaque_flags: u32,
}

#[derive(Debug)]
pub struct FstCatalog {
    pub files: Vec<FstFile>,
    pub directories: Vec<String>,
}

struct Walker<'a> {
    pool: &'a [u8],
    rows: Vec<[u32; 3]>,
    visited: HashSet<usize>,
    names: HashSet<String>,
    path_bytes: usize,
    catalog: FstCatalog,
}

impl Walker<'_> {
    fn listing(&mut self, offset: u32, parent: &str, depth: usize) -> io::Result<()> {
        if depth > 64 || !offset.is_multiple_of(12) {
            return Err(invalid("FST directory depth or alignment invalid"));
        }
        let mut i = offset as usize / 12;
        loop {
            let &[flags, name_offset, value] = self
                .rows
                .get(i)
                .ok_or_else(|| invalid("FST listing is out of bounds or unterminated"))?;
            if !self.visited.insert(i) {
                return Err(invalid("FST cyclic or shared directory listing"));
            }
            if flags == 3 && name_offset == 0 && value == 0 {
                return Ok(());
            }
            let name_at = name_offset as usize;
            if name_at != 0 && self.pool.get(name_at - 1) != Some(&0) {
                return Err(invalid("FST name does not begin at a string boundary"));
            }
            let tail = self
                .pool
                .get(name_at..)
                .ok_or_else(|| invalid("FST name out of bounds"))?;
            let len = tail
                .iter()
                .position(|b| *b == 0)
                .ok_or_else(|| invalid("FST name unterminated"))?;
            let name = &tail[..len];
            if name.is_empty()
                || len > 255
                || name == b"."
                || name == b".."
                || !name
                    .iter()
                    .all(|b| (0x20..=0x7e).contains(b) && !b"/\\:".contains(b))
            {
                return Err(invalid("FST name is not a safe ASCII component"));
            }
            let name = std::str::from_utf8(name).unwrap();
            let path = if parent.is_empty() {
                name.to_owned()
            } else {
                format!("{parent}/{name}")
            };
            self.path_bytes += path.len();
            if self.path_bytes > MAX_BYTES as usize {
                return Err(invalid("FST expanded paths exceed allocation limit"));
            }
            if !self.names.insert(path.clone()) {
                return Err(invalid("FST duplicate path"));
            }
            match flags & 255 {
                1 if flags == 1 => {
                    self.catalog.directories.push(path.clone());
                    if value != 0 {
                        self.listing(value, &path, depth + 1)?;
                    }
                }
                2 => self.catalog.files.push(FstFile {
                    path,
                    ordinal: value,
                    opaque_flags: flags >> 8,
                }),
                _ => return Err(invalid("FST unsupported record kind")),
            }
            i += 1;
        }
    }
}

impl FstCatalog {
    pub fn read(source: impl Read) -> io::Result<Self> {
        let mut bytes = Vec::new();
        source.take(MAX_BYTES + 1).read_to_end(&mut bytes)?;
        if bytes.len() as u64 > MAX_BYTES || bytes.len() < 16 || &bytes[..8] != b"FST\0\0\0\x80\x3f"
        {
            return Err(invalid("FST size, signature or version unsupported"));
        }
        let pool = u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize;
        let table = u32::from_le_bytes(bytes[12..16].try_into().unwrap()) as usize;
        if pool != 16
            || table < pool
            || table > bytes.len()
            || !(bytes.len() - table).is_multiple_of(12)
        {
            return Err(invalid("FST header offsets or table size invalid"));
        }
        let count = (bytes.len() - table) / 12;
        if count == 0 || count > MAX_RECORDS {
            return Err(invalid("FST record count outside limit"));
        }
        let rows = bytes[table..]
            .chunks_exact(12)
            .map(|row| {
                [
                    u32::from_le_bytes(row[..4].try_into().unwrap()),
                    u32::from_le_bytes(row[4..8].try_into().unwrap()),
                    u32::from_le_bytes(row[8..12].try_into().unwrap()),
                ]
            })
            .collect();
        let mut walker = Walker {
            pool: &bytes[pool..table],
            rows,
            visited: HashSet::new(),
            names: HashSet::new(),
            path_bytes: 0,
            catalog: Self {
                files: Vec::new(),
                directories: Vec::new(),
            },
        };
        walker.listing(0, "", 0)?;
        for (i, row) in walker.rows.iter().enumerate() {
            if !walker.visited.contains(&i) && !(i + 1 == count && *row == [0; 3]) {
                return Err(invalid("FST unreachable records"));
            }
        }
        Ok(walker.catalog)
    }

    pub fn validate_archive(&self, afs: &AfsCatalog) -> io::Result<()> {
        let mut seen = HashSet::new();
        if self.files.len() != afs.entries.len()
            || self
                .files
                .iter()
                .any(|f| f.ordinal as usize >= afs.entries.len() || !seen.insert(f.ordinal))
        {
            return Err(invalid("FST ordinals do not cover AFS exactly once"));
        }
        Ok(())
    }

    pub fn file(&self, path: &str) -> io::Result<&FstFile> {
        self.files
            .iter()
            .find(|f| f.path == path)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "FST path not found"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture(pool: &[u8], rows: &[[u32; 3]]) -> Vec<u8> {
        let mut data = b"FST\0\0\0\x80\x3f".to_vec();
        data.extend(16u32.to_le_bytes());
        data.extend((16 + pool.len() as u32).to_le_bytes());
        data.extend(pool);
        for row in rows {
            for n in row {
                data.extend(n.to_le_bytes());
            }
        }
        data
    }
    #[test]
    fn nested_namespace_and_opaque_metadata() {
        let bytes = fixture(
            b"folder\0asset\0empty\0",
            &[
                [1, 0, 36],
                [1, 13, 0],
                [3, 0, 0],
                [0x123402, 7, 0],
                [3, 0, 0],
                [0; 3],
            ],
        );
        let fs = FstCatalog::read(bytes.as_slice()).unwrap();
        assert_eq!(fs.file("folder/asset").unwrap().opaque_flags, 0x1234);
        assert_eq!(fs.directories, ["folder", "empty"]);
        let afs = AfsCatalog {
            entries: vec![crate::afs::AfsEntry {
                offset: 16,
                size: 1,
            }],
            archive_size: 17,
        };
        fs.validate_archive(&afs).unwrap();
        assert!(fs.file("missing").is_err());
    }
    #[test]
    fn rejects_cycles_aliases_alignment_and_missing_end() {
        for rows in [
            vec![[1, 0, 24], [3, 0, 0], [1, 0, 24], [3, 0, 0]],
            vec![[1, 0, 36], [1, 2, 36], [3, 0, 0], [3, 0, 0]],
            vec![[1, 0, 1], [3, 0, 0]],
            vec![[2, 0, 0]],
        ] {
            assert!(FstCatalog::read(fixture(b"a\0b\0", &rows).as_slice()).is_err());
        }
    }
    #[test]
    fn rejects_bad_names_duplicates_and_unreachable_records() {
        for pool in [
            b"..\0".as_slice(),
            b"a/b\0",
            b"a\\b\0",
            b"no-end",
            b"\xff\0",
        ] {
            assert!(FstCatalog::read(fixture(pool, &[[2, 0, 0], [3, 0, 0]]).as_slice()).is_err());
        }
        for rows in [
            vec![[2, 0, 0], [2, 0, 1], [3, 0, 0]],
            vec![[3, 0, 0], [2, 0, 0]],
            vec![[2, 1, 0], [3, 0, 0]],
        ] {
            assert!(FstCatalog::read(fixture(b"abc\0", &rows).as_slice()).is_err());
        }
    }
    #[test]
    fn rejects_bad_header_truncation_and_archive_binding() {
        let good = fixture(b"a\0b\0", &[[2, 0, 0], [2, 2, 0], [3, 0, 0]]);
        for size in 0..good.len() {
            assert!(FstCatalog::read(&good[..size]).is_err());
        }
        let fs = FstCatalog::read(good.as_slice()).unwrap();
        let mut afs = AfsCatalog {
            entries: vec![
                crate::afs::AfsEntry {
                    offset: 32,
                    size: 0
                };
                2
            ],
            archive_size: 32,
        };
        assert!(fs.validate_archive(&afs).is_err());
        afs.entries.clear();
        assert!(fs.validate_archive(&afs).is_err());
        let mut bad = good;
        bad[4] = 1;
        assert!(FstCatalog::read(bad.as_slice()).is_err());
    }
}
