//! Original read-only implementation of the bounded catalog contract in specs/afs-catalog-v1.md.
use std::io::{self, Read, Seek, SeekFrom};

const MAX_ENTRIES: u32 = 65_536;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AfsEntry {
    pub offset: u32,
    pub size: u32,
}

#[derive(Debug)]
pub struct AfsCatalog {
    pub entries: Vec<AfsEntry>,
    pub archive_size: u64,
}

fn invalid(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

impl AfsCatalog {
    pub fn open_entry<'a, R: Read + Seek>(
        &self,
        source: &'a mut R,
        ordinal: usize,
    ) -> io::Result<crate::FileSlice<'a, R>> {
        let entry = self
            .entries
            .get(ordinal)
            .ok_or_else(|| invalid("AFS ordinal out of range"))?;
        let length = source.seek(SeekFrom::End(0))?;
        if length != self.archive_size
            || u64::from(entry.offset) + u64::from(entry.size) > length
            || (entry.size != 0 && u64::from(entry.offset) < 8 + self.entries.len() as u64 * 8)
        {
            return Err(invalid("AFS entry or backing source bounds changed"));
        }
        Ok(crate::FileSlice {
            source,
            start: u64::from(entry.offset),
            length: u64::from(entry.size),
            position: 0,
        })
    }

    pub fn read(mut source: impl Read + Seek) -> io::Result<Self> {
        let archive_size = source.seek(SeekFrom::End(0))?;
        source.seek(SeekFrom::Start(0))?;
        let mut header = [0u8; 8];
        source.read_exact(&mut header)?;
        if &header[..4] != b"AFS\0" {
            return Err(invalid("unsupported AFS signature"));
        }
        let count = u32::from_le_bytes(header[4..].try_into().unwrap());
        if count > MAX_ENTRIES {
            return Err(invalid("AFS entry count exceeds catalog limit"));
        }
        let table_end = 8 + u64::from(count) * 8;
        if table_end > archive_size {
            return Err(invalid("AFS table exceeds archive bounds"));
        }
        let mut entries = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let mut bytes = [0u8; 8];
            source.read_exact(&mut bytes)?;
            let offset = u32::from_le_bytes(bytes[..4].try_into().unwrap());
            let size = u32::from_le_bytes(bytes[4..].try_into().unwrap());
            if u64::from(offset) + u64::from(size) > archive_size {
                return Err(invalid("AFS entry exceeds archive bounds"));
            }
            if size != 0 && u64::from(offset) < table_end {
                return Err(invalid("AFS payload overlaps catalog"));
            }
            entries.push(AfsEntry { offset, size });
        }
        Ok(Self {
            entries,
            archive_size,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn fixture(entries: &[(u32, u32)], len: usize) -> Vec<u8> {
        let mut data = vec![0; len];
        data[..4].copy_from_slice(b"AFS\0");
        data[4..8].copy_from_slice(&(entries.len() as u32).to_le_bytes());
        for (i, &(offset, size)) in entries.iter().enumerate() {
            let base = 8 + i * 8;
            data[base..base + 4].copy_from_slice(&offset.to_le_bytes());
            data[base + 4..base + 8].copy_from_slice(&size.to_le_bytes());
        }
        data
    }

    #[test]
    fn accepts_catalog_with_aliased_and_empty_entries() {
        let bytes = fixture(&[(32, 8), (32, 8), (0, 0)], 40);
        let catalog = AfsCatalog::read(Cursor::new(bytes)).unwrap();
        assert_eq!(catalog.archive_size, 40);
        assert_eq!(catalog.entries.len(), 3);
        assert_eq!(catalog.entries[0], catalog.entries[1]);
    }

    #[test]
    fn rejects_bad_signature_and_truncation() {
        assert!(AfsCatalog::read(Cursor::new(vec![0; 7])).is_err());
        assert!(AfsCatalog::read(Cursor::new(vec![0; 8])).is_err());
        let mut bytes = fixture(&[], 8);
        bytes[4..8].copy_from_slice(&1u32.to_le_bytes());
        assert!(AfsCatalog::read(Cursor::new(bytes)).is_err());
    }

    #[test]
    fn rejects_excessive_count_before_allocating() {
        let mut bytes = fixture(&[], 8);
        bytes[4..8].copy_from_slice(&(MAX_ENTRIES + 1).to_le_bytes());
        assert!(AfsCatalog::read(Cursor::new(bytes)).is_err());
    }

    #[test]
    fn rejects_out_of_range_and_table_overlapping_payloads() {
        for pair in [(31, 2), (u32::MAX, 2), (8, 1), (u32::MAX, 0)] {
            assert!(AfsCatalog::read(Cursor::new(fixture(&[pair], 32))).is_err());
        }
    }

    #[test]
    fn reads_only_header_and_catalog() {
        struct Counted {
            cursor: Cursor<Vec<u8>>,
            read_bytes: usize,
        }
        impl Read for Counted {
            fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
                let size = self.cursor.read(out)?;
                self.read_bytes += size;
                Ok(size)
            }
        }
        impl Seek for Counted {
            fn seek(&mut self, seek: SeekFrom) -> io::Result<u64> {
                self.cursor.seek(seek)
            }
        }
        let mut source = Counted {
            cursor: Cursor::new(fixture(&[(2048, 4096)], 8192)),
            read_bytes: 0,
        };
        AfsCatalog::read(&mut source).unwrap();
        assert_eq!(source.read_bytes, 16);
    }

    #[test]
    fn member_view_cannot_read_or_seek_into_its_neighbor() {
        let mut bytes = fixture(&[(32, 2), (34, 2)], 36);
        bytes[32..].copy_from_slice(&[11, 22, 33, 44]);
        let mut source = Cursor::new(bytes);
        let catalog = AfsCatalog::read(&mut source).unwrap();
        let mut member = catalog.open_entry(&mut source, 0).unwrap();
        let mut data = Vec::new();
        member.read_to_end(&mut data).unwrap();
        assert_eq!(data, [11, 22]);
        assert!(member.seek(SeekFrom::Start(3)).is_err());
        assert!(catalog.open_entry(&mut source, 2).is_err());
        source.get_mut().truncate(35);
        assert!(catalog.open_entry(&mut source, 0).is_err());
    }
}
