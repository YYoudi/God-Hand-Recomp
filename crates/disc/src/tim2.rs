//! Narrow, original indexed TIM2 decoder. See specs/tim2-indexed-v1.md.
use std::io::{self, Read};

#[derive(Debug)]
pub struct RgbaImage {
    pub width: u16,
    pub height: u16,
    pub pixels: Vec<u8>,
}

fn invalid(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

pub fn decode(source: impl Read) -> io::Result<RgbaImage> {
    let mut data = Vec::new();
    source.take(16 * 1024 * 1024 + 1).read_to_end(&mut data)?;
    decode_indexed(&data, false)
}

pub(crate) fn decode_indexed(data: &[u8], tim3: bool) -> io::Result<RgbaImage> {
    if data.len() > 16 * 1024 * 1024 || data.len() < 64 {
        return Err(invalid("indexed texture input size invalid"));
    }
    let u16_at = |at| u16::from_le_bytes(data[at..at + 2].try_into().unwrap());
    let u32_at = |at| u32::from_le_bytes(data[at..at + 4].try_into().unwrap()) as usize;
    let signature = if tim3 { b"TIM3" } else { b"TIM2" };
    if &data[..4] != signature
        || data[4] != 4
        || !(data[5] == 0 || (tim3 && data[5] == 6))
        || u16_at(6) != 1
    {
        return Err(invalid(
            "unsupported indexed texture version, storage selector or picture count",
        ));
    }
    let total = u32_at(16);
    let palette_size = u32_at(20);
    let image_size = u32_at(24);
    let colors = usize::from(u16_at(30));
    let width = u16_at(36);
    let height = u16_at(38);
    let count = usize::from(width) * usize::from(height);
    if u16_at(28) != 48 || data[32] != 0 || data[33] != 1 || data[34] != 3 {
        return Err(invalid(
            "unsupported indexed texture header, mipmaps or palette mode",
        ));
    }
    if width == 0 || height == 0 || width > 4096 || height > 4096 || count > 4_194_304 {
        return Err(invalid("indexed texture dimensions exceed limits"));
    }
    if data[5] == 6
        && (width < 128 || height < 128 || !width.is_power_of_two() || !height.is_power_of_two())
    {
        return Err(invalid("unsupported TIM3 transfer dimensions"));
    }
    let expected_colors = match data[35] {
        4 => 16,
        5 => 256,
        _ => return Err(invalid("unsupported indexed texture index type")),
    };
    let expected_image = if data[35] == 4 {
        count.div_ceil(2)
    } else {
        count
    };
    if colors != expected_colors
        || palette_size != colors * 4
        || image_size != expected_image
        || total != 48 + image_size + palette_size
        || 16 + total > data.len()
    {
        return Err(invalid(
            "indexed texture picture sizes or palette count inconsistent",
        ));
    }
    let palette = &data[64 + image_size..64 + image_size + palette_size];
    let mut pixels = Vec::with_capacity(count * 4);
    for i in 0..count {
        let (byte, shift) = if data[5] == 6 {
            crate::tm3::transfer_address(
                i % usize::from(width),
                i / usize::from(width),
                usize::from(width),
                data[35],
            )
        } else if data[35] == 4 {
            (i / 2, (i % 2) * 4)
        } else {
            (i, 0)
        };
        if byte >= image_size {
            return Err(invalid("TIM3 transfer address outside image data"));
        }
        let index = if data[35] == 4 {
            usize::from((data[64 + byte] >> shift) & 15)
        } else {
            let index = usize::from(data[64 + byte]);
            (index & !24) | ((index & 8) << 1) | ((index & 16) >> 1)
        };
        let color = &palette[index * 4..index * 4 + 4];
        pixels.extend_from_slice(&color[..3]);
        pixels.push(((u16::from(color[3]) * 255 + 64) / 128).min(255) as u8);
    }
    Ok(RgbaImage {
        width,
        height,
        pixels,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture(kind: u8, indices: &[u8], width: u16) -> Vec<u8> {
        let colors = if kind == 4 { 16usize } else { 256 };
        let total = 48 + indices.len() + colors * 4;
        let mut data = vec![0u8; 16 + total];
        data[..4].copy_from_slice(b"TIM2");
        data[4] = 4;
        data[6] = 1;
        data[16..20].copy_from_slice(&(total as u32).to_le_bytes());
        data[20..24].copy_from_slice(&((colors * 4) as u32).to_le_bytes());
        data[24..28].copy_from_slice(&(indices.len() as u32).to_le_bytes());
        data[28] = 48;
        data[30..32].copy_from_slice(&(colors as u16).to_le_bytes());
        data[33] = 1;
        data[34] = 3;
        data[35] = kind;
        data[36..38].copy_from_slice(&width.to_le_bytes());
        data[38] = 1;
        data[64..64 + indices.len()].copy_from_slice(indices);
        for i in 0..colors {
            let at = 64 + indices.len() + i * 4;
            data[at..at + 4].copy_from_slice(&[i as u8, 17, 29, 128]);
        }
        data
    }
    #[test]
    fn low_nibble_first_and_odd_pixel_count() {
        let bytes = fixture(4, &[0x31, 0xf2], 3);
        let image = decode(bytes.as_slice()).unwrap();
        assert_eq!((image.width, image.height), (3, 1));
        assert_eq!(
            image.pixels,
            [1, 17, 29, 255, 3, 17, 29, 255, 2, 17, 29, 255]
        );
    }
    #[test]
    fn csm1_palette_exchange_and_alpha() {
        let mut bytes = fixture(5, &[8, 16, 32], 3);
        let palette = 67;
        bytes[palette + 16 * 4 + 3] = 64;
        bytes[palette + 8 * 4 + 3] = 255;
        bytes[palette + 32 * 4 + 3] = 0;
        assert_eq!(
            decode(bytes.as_slice()).unwrap().pixels,
            [16, 17, 29, 128, 8, 17, 29, 255, 32, 17, 29, 0]
        );
    }
    #[test]
    fn refuses_unsupported_layouts_and_bad_sizes() {
        let original = fixture(4, &[0x10], 2);
        for (at, value) in [
            (4, 3),
            (5, 1),
            (6, 2),
            (28, 49),
            (32, 1),
            (33, 2),
            (34, 67),
            (35, 3),
            (36, 0),
            (30, 15),
            (20, 1),
            (24, 2),
            (16, 0),
        ] {
            let mut bad = original.clone();
            bad[at] = value;
            assert!(decode(bad.as_slice()).is_err(), "offset {at}");
        }
        for end in 0..original.len() {
            assert!(decode(&original[..end]).is_err());
        }
    }

    #[test]
    fn tim3_transfer_image_matches_synthetic_reference_indices() {
        let indices: Vec<u8> = (0..8192)
            .map(|at| ((at * 73 + (at / 251) * 19 + 11) % 256) as u8)
            .collect();
        let mut bytes = fixture(4, &indices, 128);
        bytes[..4].copy_from_slice(b"TIM3");
        bytes[5] = 6;
        bytes[38..40].copy_from_slice(&128u16.to_le_bytes());
        let image = crate::tm3::decode(bytes.as_slice()).unwrap();
        assert_eq!(
            &image.pixels[(3 * 128 + 17) * 4..(3 * 128 + 17) * 4 + 4],
            &[6, 17, 29, 255]
        );
        let hash = image
            .pixels
            .chunks_exact(4)
            .fold(14695981039346656037u64, |hash, p| {
                (hash ^ u64::from(p[0])).wrapping_mul(1099511628211)
            });
        assert_eq!(hash, 17615709926516263811);
        bytes[38..40].copy_from_slice(&1u16.to_le_bytes());
        assert!(crate::tm3::decode(bytes.as_slice()).is_err());
        bytes[5] = 1;
        assert!(crate::tm3::decode(bytes.as_slice()).is_err());
        let mut linear = fixture(5, &[8, 16], 2);
        linear[..4].copy_from_slice(b"TIM3");
        assert_eq!(
            crate::tm3::decode(linear.as_slice()).unwrap().pixels,
            [16, 17, 29, 255, 8, 17, 29, 255]
        );
    }
}
