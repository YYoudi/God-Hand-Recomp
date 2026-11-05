//! Bounded bank metadata and original direct GS transfer-address mapping.
//! See specs/tm3-tim3-v1.md; no game program is loaded or executed.
use std::io::{self, Read, Seek, SeekFrom};

#[derive(Debug)]
pub struct TextureEntry {
    pub name: [u8; 8],
    pub offset: u64,
    pub size: u64,
}
#[derive(Debug)]
pub struct TextureBank {
    entries: Vec<TextureEntry>,
    length: u64,
}
fn invalid(m: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, m)
}

impl TextureBank {
    pub fn read(mut source: impl Read + Seek) -> io::Result<Self> {
        let length = source.seek(SeekFrom::End(0))?;
        source.seek(SeekFrom::Start(0))?;
        let mut header = [0; 16];
        source.read_exact(&mut header)?;
        if &header[..4] != b"TM3\0" {
            return Err(invalid("unsupported TM3 signature"));
        }
        let count = u32::from_le_bytes(header[4..8].try_into().unwrap());
        let names = 16 + u64::from(count) * 4 + u64::from(count % 2) * 4;
        let table_end = names + u64::from(count) * 8;
        if count > 4096 || table_end > length {
            return Err(invalid("TM3 count or metadata exceeds limit"));
        }
        let mut offsets = Vec::with_capacity(count as usize);
        let mut previous = None;
        for _ in 0..count {
            let mut word = [0; 4];
            source.read_exact(&mut word)?;
            let offset = u64::from(u32::from_le_bytes(word));
            if offset < table_end
                || offset >= length
                || offset % 16 != 0
                || previous.is_some_and(|v| offset <= v)
            {
                return Err(invalid("TM3 image offsets outside bounds or overlapping"));
            }
            offsets.push(offset);
            previous = Some(offset);
        }
        source.seek(SeekFrom::Start(names))?;
        let mut entries = Vec::with_capacity(count as usize);
        for (i, &offset) in offsets.iter().enumerate() {
            let mut name = [0; 8];
            source.read_exact(&mut name)?;
            entries.push(TextureEntry {
                name,
                offset,
                size: offsets.get(i + 1).copied().unwrap_or(length) - offset,
            });
        }
        Ok(Self { entries, length })
    }
    pub fn entries(&self) -> &[TextureEntry] {
        &self.entries
    }
    pub fn open_image<'a, R: Read + Seek>(
        &self,
        source: &'a mut R,
        ordinal: usize,
    ) -> io::Result<crate::FileSlice<'a, R>> {
        let image = self
            .entries
            .get(ordinal)
            .ok_or_else(|| invalid("TM3 texture ordinal outside bank"))?;
        if source.seek(SeekFrom::End(0))? != self.length {
            return Err(invalid("TM3 backing length changed"));
        }
        Ok(crate::FileSlice {
            source,
            start: image.offset,
            length: image.size,
            position: 0,
        })
    }
}

pub fn decode(source: impl Read) -> io::Result<crate::tim2::RgbaImage> {
    Ok(decode_raw(source)?.into_presentation())
}

pub fn decode_raw(source: impl Read) -> io::Result<crate::tim2::RawRgbaImage> {
    let mut bytes = Vec::new();
    source.take(16 * 1024 * 1024 + 1).read_to_end(&mut bytes)?;
    crate::tim2::decode_indexed_raw(&bytes, true)
}

fn bit(value: usize, index: usize) -> usize {
    (value >> index) & 1
}

/// Returns the source byte and nibble shift for one indexed GS read.
pub(crate) fn transfer_address(x: usize, y: usize, width: usize, kind: u8) -> (usize, usize) {
    let four = kind == 4;
    let page_height = if four { 128 } else { 64 };
    let page = (y / page_height) * (width / 128) + x / 128;
    let mut address = 0usize;
    let bits: [usize; 14] = if four {
        [
            bit(y, 1),
            bit(x, 3),
            bit(x, 4),
            bit(x, 0),
            bit(y, 0),
            bit(x, 1),
            bit(x, 2) ^ bit(y, 1) ^ bit(y, 2),
            bit(y, 2),
            bit(y, 3),
            bit(y, 4),
            bit(x, 5),
            bit(y, 5),
            bit(x, 6),
            bit(y, 6),
        ]
    } else {
        [
            bit(y, 1),
            bit(x, 3),
            bit(x, 0),
            bit(y, 0),
            bit(x, 1),
            bit(x, 2) ^ bit(y, 1) ^ bit(y, 2),
            bit(y, 2),
            bit(y, 3),
            bit(x, 4),
            bit(y, 4),
            bit(x, 5),
            bit(y, 5),
            bit(x, 6),
            0,
        ]
    };
    for (i, b) in bits.into_iter().enumerate() {
        address |= b << i;
    }
    let shift = if four { (address & 1) * 4 } else { 0 };
    if four {
        address >>= 1;
    }
    // Invert one PSMCT32 page to the raster order used by the transfer.
    let mut sx = 0usize;
    let mut sy = 0usize;
    for (i, at) in [2, 4, 5, 8, 10, 12].into_iter().enumerate() {
        sx |= bit(address, at) << i;
    }
    for (i, at) in [3, 6, 7, 9, 11].into_iter().enumerate() {
        sy |= bit(address, at) << i;
    }
    let pages_per_row = width / 128;
    sx += (page % pages_per_row) * 64;
    sy += (page / pages_per_row) * 32;
    ((sy * (width / 2) + sx) * 4 + (address & 3), shift)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    fn fixture() -> Vec<u8> {
        let mut b = vec![0; 96];
        b[..4].copy_from_slice(b"TM3\0");
        b[4] = 1;
        b[16] = 32;
        b[24..32].copy_from_slice(b"original");
        b[32..].fill(23);
        b
    }
    #[test]
    fn odd_bank_padding_and_bounded_images() {
        let mut data = Cursor::new(fixture());
        let bank = TextureBank::read(&mut data).unwrap();
        assert_eq!(&bank.entries()[0].name, b"original");
        let mut image = bank.open_image(&mut data, 0).unwrap();
        let mut bytes = Vec::new();
        image.read_to_end(&mut bytes).unwrap();
        assert_eq!(bytes, vec![23; 64]);
        assert!(image.seek(SeekFrom::Start(65)).is_err());
        assert!(bank.open_image(&mut data, 1).is_err());
        data.get_mut().push(0);
        assert!(bank.open_image(&mut data, 0).is_err());
    }
    #[test]
    fn rejects_bad_bank_header_count_offsets_and_truncation() {
        for (at, value) in [(0, 0u32), (4, 4097), (16, 16), (16, 33), (16, 96)] {
            let mut b = fixture();
            b[at..at + 4].copy_from_slice(&value.to_le_bytes());
            assert!(TextureBank::read(Cursor::new(b)).is_err());
        }
        assert!(TextureBank::read(Cursor::new(&fixture()[..31])).is_err());
    }
    #[test]
    fn transfer_maps_cover_each_source_bit_once() {
        for (w, h, kind) in [(128, 128, 4), (256, 128, 4), (128, 128, 5), (256, 256, 5)] {
            let mut seen = std::collections::HashSet::new();
            for y in 0..h {
                for x in 0..w {
                    let (at, shift) = transfer_address(x, y, w, kind);
                    assert!(at < if kind == 4 { w * h / 2 } else { w * h });
                    assert!(seen.insert((at, shift)));
                }
            }
            assert_eq!(seen.len(), w * h);
        }
    }

    #[test]
    fn direct_map_matches_independent_table_reference_images() {
        // FNV-1a over synthetic index images from an independent PSMCT32-upload /
        // indexed-table-read oracle. No game data; see the versioned contract.
        for (w, h, kind, expected) in [
            (128usize, 128usize, 4u8, 17615709926516263811u64),
            (256, 128, 4, 18177759005446472084),
            (128, 128, 5, 9473997573475588510),
            (256, 256, 5, 10847477878144182148),
        ] {
            let mut hash = 14695981039346656037u64;
            for y in 0..h {
                for x in 0..w {
                    let (at, shift) = transfer_address(x, y, w, kind);
                    let mut index = ((at * 73 + (at / 251) * 19 + 11) % 256) as u8;
                    if kind == 4 {
                        index = (index >> shift) & 15;
                    }
                    hash = (hash ^ u64::from(index)).wrapping_mul(1099511628211);
                }
            }
            assert_eq!(hash, expected, "{w}x{h}, type {kind}");
        }
        assert_eq!(transfer_address(17, 3, 128, 4), (278, 4));
        assert_eq!(transfer_address(129, 65, 256, 5), (17156, 0));
    }
}
