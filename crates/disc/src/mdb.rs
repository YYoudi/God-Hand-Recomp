//! Original bounded static mesh decoder; contract/provenance in specs/static-mdb-v1.md.
use std::io::{self, Read, Seek, SeekFrom};

#[derive(Debug)]
pub struct StaticPrimitive {
    pub positions: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
    pub material: u16,
    /// Unmodified four-byte candidate RGBA stream; absent is not invented white.
    pub colors: Option<Vec<[u8; 4]>>,
}

#[derive(Debug)]
pub struct StaticMesh {
    pub primitives: Vec<StaticPrimitive>,
}

fn invalid(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}
fn u32_at(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(b[at..at + 4].try_into().unwrap())
}
fn u16_at(b: &[u8], at: usize) -> u16 {
    u16::from_le_bytes(b[at..at + 2].try_into().unwrap())
}

impl StaticMesh {
    pub fn read(mut source: impl Read + Seek) -> io::Result<Self> {
        let length = source.seek(SeekFrom::End(0))?;
        source.seek(SeekFrom::Start(0))?;
        let mut header = [0; 32];
        source.read_exact(&mut header)?;
        if &header[..4] != b"mdb\0" || u16_at(&header, 30) != 0 || u16_at(&header, 8) != 1 {
            return Err(invalid("unsupported MDB signature, encoding or bone count"));
        }
        let count = usize::from(u16_at(&header, 10));
        if count > 4096 || 32 + count as u64 * 4 > length {
            return Err(invalid("MDB buffer count exceeds bounds"));
        }
        let mut offsets = Vec::with_capacity(count);
        for _ in 0..count {
            let mut word = [0; 4];
            source.read_exact(&mut word)?;
            offsets.push(u64::from(u32::from_le_bytes(word)));
        }
        let mut ranges = vec![(0, 32 + count as u64 * 4, 0u8)];
        fn region(
            ranges: &mut Vec<(u64, u64, u8)>,
            start: u64,
            size: u64,
            kind: u8,
            length: u64,
        ) -> io::Result<()> {
            let end = start
                .checked_add(size)
                .ok_or_else(|| invalid("MDB region overflow"))?;
            if end > length {
                return Err(invalid("MDB region outside mesh window"));
            }
            if size != 0 {
                for &(a, b, k) in ranges.iter() {
                    if start < b && end > a && !(kind >= 2 && kind == k && start == a && end == b) {
                        return Err(invalid("MDB incompatible regions overlap"));
                    }
                }
                ranges.push((start, end, kind));
            }
            Ok(())
        }
        let bone = u64::from(u32_at(&header, 4));
        region(&mut ranges, bone, 16, 1, length)?;
        source.seek(SeekFrom::Start(bone))?;
        let mut bone_data = [0; 16];
        source.read_exact(&mut bone_data)?;
        if (0..3)
            .any(|i| f32::from_le_bytes(bone_data[i * 4..i * 4 + 4].try_into().unwrap()) != 0.0)
        {
            return Err(invalid("MDB nonzero static bone translation unsupported"));
        }
        for &at in &offsets {
            region(&mut ranges, at, 24, 1, length)?;
        }
        let mut total_vertices = 0usize;
        let mut primitives = Vec::with_capacity(count);
        for at in offsets {
            source.seek(SeekFrom::Start(at))?;
            let mut descriptor = [0; 24];
            source.read_exact(&mut descriptor)?;
            let n = usize::from(u16_at(&descriptor, 20));
            total_vertices += n;
            if total_vertices > 1_048_576 {
                return Err(invalid("MDB decoded vertex limit exceeded"));
            }
            if u32_at(&descriptor, 16) != 0 {
                return Err(invalid("MDB weighted vertices unsupported"));
            }
            let position = at + u64::from(u32_at(&descriptor, 0));
            let uv = at + u64::from(u32_at(&descriptor, 8));
            region(&mut ranges, position, n as u64 * 8, 2, length)?;
            region(&mut ranges, uv, n as u64 * 4, 3, length)?;
            for (offset, kind) in [(4, 4), (12, 5)] {
                let relative = u32_at(&descriptor, offset);
                if relative != 0 {
                    region(
                        &mut ranges,
                        at + u64::from(relative),
                        n as u64 * 4,
                        kind,
                        length,
                    )?;
                }
            }
            let mut primitive = StaticPrimitive {
                positions: Vec::with_capacity(n),
                uvs: Vec::with_capacity(n),
                indices: Vec::new(),
                material: u16_at(&descriptor, 22),
                colors: None,
            };
            source.seek(SeekFrom::Start(position))?;
            for i in 0..n {
                let mut vertex = [0; 8];
                source.read_exact(&mut vertex)?;
                let mut point = [0f32; 3];
                for (axis, v) in point.iter_mut().enumerate() {
                    *v = f32::from(i16::from_le_bytes(
                        vertex[axis * 2..axis * 2 + 2].try_into().unwrap(),
                    )) * 0.01;
                }
                primitive.positions.push(point);
                match u16_at(&vertex, 6) {
                    32768 => {}
                    flag @ (0 | 1) if i >= 2 => {
                        let i = i as u32;
                        primitive.indices.extend(if flag == 0 {
                            [i - 2, i - 1, i]
                        } else {
                            [i - 1, i - 2, i]
                        });
                    }
                    _ => {
                        return Err(invalid(
                            "MDB unknown triangle flag or early triangle emission",
                        ));
                    }
                }
            }
            source.seek(SeekFrom::Start(uv))?;
            for _ in 0..n {
                let mut pair = [0; 4];
                source.read_exact(&mut pair)?;
                primitive.uvs.push([
                    f32::from(i16::from_le_bytes(pair[..2].try_into().unwrap())) / 4096.0,
                    f32::from(i16::from_le_bytes(pair[2..].try_into().unwrap())) / 4096.0 + 1.0,
                ]);
            }
            let color = u32_at(&descriptor, 12);
            if color != 0 {
                source.seek(SeekFrom::Start(at + u64::from(color)))?;
                let mut colors = Vec::with_capacity(n);
                for _ in 0..n {
                    let mut value = [0; 4];
                    source.read_exact(&mut value)?;
                    colors.push(value);
                }
                primitive.colors = Some(colors);
            }
            primitives.push(primitive);
        }
        Ok(Self { primitives })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    fn fixture() -> Vec<u8> {
        let mut b = vec![0; 160];
        b[..4].copy_from_slice(b"mdb\0");
        b[4] = 48;
        b[8] = 1;
        b[10] = 1;
        b[32] = 64;
        b[64] = 32;
        b[72] = 64;
        b[84] = 4;
        b[86] = 7;
        for (i, (p, flag)) in [
            ([-100i16, 0, 100], 32768u16),
            ([100, 0, 100], 32768),
            ([0, 100, 0], 0),
            ([0, -100, 0], 1),
        ]
        .iter()
        .enumerate()
        {
            for axis in 0..3 {
                let at = 96 + i * 8 + axis * 2;
                b[at..at + 2].copy_from_slice(&p[axis].to_le_bytes());
            }
            b[102 + i * 8..104 + i * 8].copy_from_slice(&flag.to_le_bytes());
        }
        b[128..130].copy_from_slice(&(-4096i16).to_le_bytes());
        b
    }
    #[test]
    fn signed_positions_uvs_and_explicit_winding() {
        let mesh = StaticMesh::read(Cursor::new(fixture())).unwrap();
        let p = &mesh.primitives[0];
        assert_eq!(p.positions[0], [-1.0, 0.0, 1.0]);
        assert_eq!(p.indices, [0, 1, 2, 2, 1, 3]);
        assert_eq!(p.uvs[0], [-1.0, 1.0]);
        assert_eq!(p.material, 7);
        assert!(p.colors.is_none());
    }
    #[test]
    fn preserves_raw_color_bytes_and_rejects_bad_color_regions() {
        let mut bytes = fixture();
        bytes[76..80].copy_from_slice(&80u32.to_le_bytes());
        let colors = [
            [0, 128, 255, 64],
            [255, 1, 2, 128],
            [3, 4, 5, 0],
            [7, 8, 9, 255],
        ];
        for (i, color) in colors.iter().enumerate() {
            bytes[144 + i * 4..148 + i * 4].copy_from_slice(color);
        }
        let mesh = StaticMesh::read(Cursor::new(bytes)).unwrap();
        assert_eq!(mesh.primitives[0].colors.as_deref(), Some(&colors[..]));
        for bad in [72u32, 96] {
            let mut bytes = fixture();
            bytes[76..80].copy_from_slice(&bad.to_le_bytes());
            assert!(StaticMesh::read(Cursor::new(bytes)).is_err());
        }
    }
    #[test]
    fn rejects_unknown_encoding_flags_and_early_triangle() {
        for (at, value) in [(30, 1), (8, 2), (102, 0), (118, 2)] {
            let mut b = fixture();
            b[at..at + 2].copy_from_slice(&(value as u16).to_le_bytes());
            assert!(StaticMesh::read(Cursor::new(b)).is_err());
        }
    }
    #[test]
    fn rejects_stream_overlap_missing_data_and_skinning() {
        for (at, value) in [
            (4, 0u32),
            (32, 140),
            (64, 0),
            (72, 32),
            (80, 1),
            (84, 65535),
        ] {
            let mut b = fixture();
            b[at..at + 4].copy_from_slice(&value.to_le_bytes());
            assert!(StaticMesh::read(Cursor::new(b)).is_err(), "field {at}");
        }
        assert!(StaticMesh::read(Cursor::new(&fixture()[..143])).is_err());
    }
}
