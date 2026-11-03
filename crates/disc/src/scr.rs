//! Bounded SCR node metadata only; mesh payloads are not decoded here.
//! Contract/provenance: specs/scr-nodes-v1.md.
use std::io::{self, Read, Seek, SeekFrom};

#[derive(Debug)]
pub struct ScrNode {
    pub record_offset: u64,
    pub name: [u8; 8],
    pub mdb_offset: u64,
    pub scale: [f32; 3],
    /// Raw candidate Euler values; units and rotation composition unverified.
    pub rotation: [f32; 3],
    pub translation: [f32; 3],
}

#[derive(Debug)]
pub struct ScrCatalog {
    pub opaque_header_word: u32,
    pub nodes: Vec<ScrNode>,
    source_length: u64,
    mdb_spans: Vec<(u64, u64)>,
}

fn invalid(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

impl ScrCatalog {
    pub fn read(mut source: impl Read + Seek) -> io::Result<Self> {
        let length = source.seek(SeekFrom::End(0))?;
        source.seek(SeekFrom::Start(0))?;
        let mut header = [0; 16];
        source.read_exact(&mut header)?;
        if &header[..4] != b"scr\0" {
            return Err(invalid("unsupported SCR signature"));
        }
        let count = u32::from_le_bytes(header[8..12].try_into().unwrap());
        let table_end = 16 + u64::from(count) * 4;
        if count > 4096 || table_end > length {
            return Err(invalid("SCR node count or table exceeds limit"));
        }
        let mut offsets = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let mut word = [0; 4];
            source.read_exact(&mut word)?;
            let at = i64::from(i32::from_le_bytes(word));
            if at < 0 || (at as u64) < table_end || (at as u64) + 112 > length {
                return Err(invalid("SCR node record outside bounds"));
            }
            offsets.push(at as u64);
        }
        let mut sorted = offsets.clone();
        sorted.sort_unstable();
        if sorted.windows(2).any(|p| p[0] + 112 > p[1]) {
            return Err(invalid("SCR node records overlap"));
        }
        let mut nodes = Vec::with_capacity(count as usize);
        for at in offsets {
            source.seek(SeekFrom::Start(at))?;
            let mut bytes = [0; 112];
            source.read_exact(&mut bytes)?;
            let relative = i64::from(i32::from_le_bytes(bytes[..4].try_into().unwrap()));
            let target = i128::from(at) + i128::from(relative);
            if target < i128::from(table_end) || target + 4 > i128::from(length) {
                return Err(invalid("SCR MDB reference outside bounds"));
            }
            let target = target as u64;
            if sorted
                .iter()
                .any(|node| target < node + 112 && target + 4 > *node)
            {
                return Err(invalid("SCR MDB signature overlaps node metadata"));
            }
            source.seek(SeekFrom::Start(target))?;
            let mut signature = [0; 4];
            source.read_exact(&mut signature)?;
            if &signature != b"mdb\0" {
                return Err(invalid("SCR reference does not point to MDB signature"));
            }
            let mut transform = [0f32; 9];
            for (i, value) in transform.iter_mut().enumerate() {
                let start = 16 + i * 4;
                *value = f32::from_le_bytes(bytes[start..start + 4].try_into().unwrap());
                if !value.is_finite() {
                    return Err(invalid("SCR transform contains nonfinite value"));
                }
            }
            nodes.push(ScrNode {
                record_offset: at,
                name: bytes[8..16].try_into().unwrap(),
                mdb_offset: target,
                scale: transform[..3].try_into().unwrap(),
                rotation: transform[3..6].try_into().unwrap(),
                translation: transform[6..9].try_into().unwrap(),
            });
        }
        let mdb_spans = nodes
            .iter()
            .map(|node| {
                let start = node.mdb_offset;
                let end = nodes
                    .iter()
                    .flat_map(|n| [n.record_offset, n.mdb_offset])
                    .filter(|v| *v > start)
                    .min()
                    .unwrap_or(length);
                (start, end - start)
            })
            .collect();
        Ok(Self {
            opaque_header_word: u32::from_le_bytes(header[4..8].try_into().unwrap()),
            nodes,
            source_length: length,
            mdb_spans,
        })
    }

    pub fn open_mdb<'a, R: Read + Seek>(
        &self,
        source: &'a mut R,
        ordinal: usize,
    ) -> io::Result<crate::FileSlice<'a, R>> {
        let &(start, length) = self
            .mdb_spans
            .get(ordinal)
            .ok_or_else(|| invalid("SCR node ordinal out of range"))?;
        if source.seek(SeekFrom::End(0))? != self.source_length {
            return Err(invalid("SCR source length changed"));
        }
        Ok(crate::FileSlice {
            source,
            start,
            length,
            position: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    fn fixture() -> Vec<u8> {
        let mut data = vec![0; 176];
        data[..4].copy_from_slice(b"scr\0");
        data[8] = 1;
        data[16] = 64;
        data[32..36].copy_from_slice(b"mdb\0");
        data[64..68].copy_from_slice(&(-32i32).to_le_bytes());
        data[72..80].copy_from_slice(b"original");
        for i in 0..3 {
            data[80 + i * 4..84 + i * 4].copy_from_slice(&1f32.to_le_bytes());
        }
        data
    }
    #[test]
    fn preserves_raw_transform_and_signed_back_reference() {
        let data = fixture();
        let catalog = ScrCatalog::read(Cursor::new(data)).unwrap();
        assert_eq!(catalog.nodes[0].mdb_offset, 32);
        assert_eq!(catalog.nodes[0].scale, [1.0; 3]);
        assert_eq!(catalog.nodes[0].translation, [0.0; 3]);
        assert_eq!(&catalog.nodes[0].name, b"original");
    }

    #[test]
    fn mesh_window_stops_before_node_metadata_and_checks_backing_length() {
        let mut source = Cursor::new(fixture());
        let mut catalog = ScrCatalog::read(&mut source).unwrap();
        // Public diagnostic metadata cannot change the validated window.
        catalog.nodes[0].mdb_offset = u64::MAX;
        let mut window = catalog.open_mdb(&mut source, 0).unwrap();
        let mut bytes = Vec::new();
        window.read_to_end(&mut bytes).unwrap();
        assert_eq!(bytes.len(), 32);
        assert_eq!(&bytes[..4], b"mdb\0");
        assert!(window.seek(SeekFrom::Start(33)).is_err());
        assert!(catalog.open_mdb(&mut source, 1).is_err());
        source.get_mut().push(0);
        assert!(catalog.open_mdb(&mut source, 0).is_err());
    }
    #[test]
    fn rejects_bad_table_node_and_target_ranges() {
        let original = fixture();
        for (at, value) in [
            (8, 4097u32),
            (16, u32::MAX),
            (16, 16),
            (16, 176),
            (64, i32::MIN as u32),
            (64, 0),
            (64, 1000),
        ] {
            let mut data = original.clone();
            data[at..at + 4].copy_from_slice(&value.to_le_bytes());
            assert!(
                ScrCatalog::read(Cursor::new(data)).is_err(),
                "offset {at}, value {value}"
            );
        }
        for end in [0, 15, 19, 64, 175] {
            assert!(ScrCatalog::read(Cursor::new(&original[..end])).is_err());
        }
    }
    #[test]
    fn rejects_nonfinite_values_signatures_and_overlapping_nodes() {
        for value in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            let mut data = fixture();
            data[80..84].copy_from_slice(&value.to_le_bytes());
            assert!(ScrCatalog::read(Cursor::new(data)).is_err());
        }
        for offset in [0, 32] {
            let mut data = fixture();
            data[offset] = 0;
            assert!(ScrCatalog::read(Cursor::new(data)).is_err());
        }
        let mut data = fixture();
        data[8] = 2;
        data[20] = 64;
        assert!(ScrCatalog::read(Cursor::new(data)).is_err());
    }
}
