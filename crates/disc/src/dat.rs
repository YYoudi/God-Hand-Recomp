//! Original tagged section catalog. No payload interpretation or extraction.
//! Contract and provenance: specs/tagged-dat-v1.md.
use std::io::{self, Read, Seek, SeekFrom};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DatSection {
    pub tag: [u8; 4],
    pub offset: u64,
    /// Storage span, including any alignment padding.
    pub size: u64,
}

#[derive(Debug)]
pub struct DatCatalog {
    sections: Vec<DatSection>,
    source_length: u64,
}

fn invalid(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

impl DatCatalog {
    pub fn read(mut source: impl Read + Seek) -> io::Result<Self> {
        let source_length = source.seek(SeekFrom::End(0))?;
        source.seek(SeekFrom::Start(0))?;
        let mut word = [0; 4];
        source.read_exact(&mut word)?;
        let count = u32::from_le_bytes(word);
        if count > 65_536 {
            return Err(invalid("DAT section count exceeds limit"));
        }
        let table_end = 4 + u64::from(count) * 8;
        if table_end > source_length {
            return Err(invalid("DAT table exceeds source bounds"));
        }
        let mut offsets = Vec::with_capacity(count as usize);
        let mut previous = table_end;
        for _ in 0..count {
            source.read_exact(&mut word)?;
            let offset = u64::from(u32::from_le_bytes(word));
            if offset < previous || offset > source_length || offset % 16 != 0 {
                return Err(invalid("DAT descending, unaligned or out-of-bounds offset"));
            }
            offsets.push(offset);
            previous = offset;
        }
        let mut sections = Vec::with_capacity(count as usize);
        for (i, &offset) in offsets.iter().enumerate() {
            source.read_exact(&mut word)?;
            let end = offsets.get(i + 1).copied().unwrap_or(source_length);
            sections.push(DatSection {
                tag: word,
                offset,
                size: end - offset,
            });
        }
        Ok(Self {
            sections,
            source_length,
        })
    }

    pub fn sections(&self) -> &[DatSection] {
        &self.sections
    }

    pub fn open_section<'a, R: Read + Seek>(
        &self,
        source: &'a mut R,
        ordinal: usize,
    ) -> io::Result<crate::FileSlice<'a, R>> {
        let section = self
            .sections
            .get(ordinal)
            .ok_or_else(|| invalid("DAT section ordinal out of range"))?;
        if source.seek(SeekFrom::End(0))? != self.source_length {
            return Err(invalid("DAT backing source length changed"));
        }
        Ok(crate::FileSlice {
            source,
            start: section.offset,
            length: section.size,
            position: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    fn fixture(offsets: &[u32], length: usize) -> Vec<u8> {
        let mut bytes = vec![0; length];
        bytes[..4].copy_from_slice(&(offsets.len() as u32).to_le_bytes());
        for (i, offset) in offsets.iter().enumerate() {
            bytes[4 + i * 4..8 + i * 4].copy_from_slice(&offset.to_le_bytes());
            let tag = 4 + offsets.len() * 4 + i * 4;
            bytes[tag..tag + 4].copy_from_slice(b"SYN\0");
        }
        bytes
    }
    #[test]
    fn preserves_repeated_tags_empty_sections_and_storage_spans() {
        let bytes = fixture(&[32, 48, 48], 64);
        let catalog = DatCatalog::read(Cursor::new(bytes)).unwrap();
        assert_eq!(
            catalog.sections.iter().map(|s| s.size).collect::<Vec<_>>(),
            [16, 0, 16]
        );
        assert_eq!(catalog.sections[0].tag, catalog.sections[1].tag);
        assert!(
            DatCatalog::read(Cursor::new(0u32.to_le_bytes()))
                .unwrap()
                .sections
                .is_empty()
        );
    }
    #[test]
    fn rejects_bad_counts_truncation_alignment_bounds_and_order() {
        for offsets in [
            &[16u32, 48][..],
            &[32, 17],
            &[48, 32],
            &[32, 80],
            &[u32::MAX],
        ] {
            assert!(DatCatalog::read(Cursor::new(fixture(offsets, 64))).is_err());
        }
        let good = fixture(&[32, 48], 64);
        for size in 0..20 {
            assert!(DatCatalog::read(Cursor::new(&good[..size])).is_err());
        }
        assert!(DatCatalog::read(Cursor::new(65_537u32.to_le_bytes())).is_err());
    }
    #[test]
    fn nested_member_cannot_read_neighbor_or_seek_outside_parent() {
        let mut bytes = fixture(&[32, 64], 80);
        bytes[32..64].copy_from_slice(&fixture(&[16], 32));
        bytes[48..64].fill(7);
        bytes[64..].fill(99);
        let mut source = Cursor::new(bytes);
        let outer = DatCatalog::read(&mut source).unwrap();
        let mut window = outer.open_section(&mut source, 0).unwrap();
        let inner = DatCatalog::read(&mut window).unwrap();
        let mut payload = inner.open_section(&mut window, 0).unwrap();
        let mut data = Vec::new();
        payload.read_to_end(&mut data).unwrap();
        assert_eq!(data, vec![7; 16]);
        assert!(payload.seek(SeekFrom::Start(17)).is_err());
        assert!(outer.open_section(&mut source, 2).is_err());
        source.get_mut().push(0);
        assert!(outer.open_section(&mut source, 0).is_err());
    }
    #[test]
    fn reads_only_header_and_tables() {
        struct Counted {
            inner: Cursor<Vec<u8>>,
            read: usize,
        }
        impl Read for Counted {
            fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
                let n = self.inner.read(out)?;
                self.read += n;
                Ok(n)
            }
        }
        impl Seek for Counted {
            fn seek(&mut self, at: SeekFrom) -> io::Result<u64> {
                self.inner.seek(at)
            }
        }
        let mut source = Counted {
            inner: Cursor::new(fixture(&[64, 128], 4096)),
            read: 0,
        };
        DatCatalog::read(&mut source).unwrap();
        assert_eq!(source.read, 20);
    }
}
