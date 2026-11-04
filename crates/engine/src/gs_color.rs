//! Original pointwise hardware-math oracle. Not the complete game's GS pipeline.
//! Contract/provenance: specs/static-colors-v1.md.
pub fn modulate_rgb(texture: [u8; 3], vertex: [u8; 3]) -> [u8; 3] {
    std::array::from_fn(|i| ((u16::from(texture[i]) * u16::from(vertex[i])) / 128).min(255) as u8)
}

pub fn srgb_byte_to_linear(value: u8) -> f32 {
    let x = f32::from(value) / 255.;
    if x <= 0.04045 {
        x / 12.92
    } else {
        ((x + 0.055) / 1.055).powf(2.4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn neutral_black_truncation_and_saturation() {
        assert_eq!(modulate_rgb([1, 128, 255], [128; 3]), [1, 128, 255]);
        assert_eq!(modulate_rgb([255; 3], [0; 3]), [0; 3]);
        assert_eq!(modulate_rgb([1, 129, 255], [64; 3]), [0, 64, 127]);
        assert_eq!(modulate_rgb([255; 3], [255; 3]), [255; 3]);
    }
    #[test]
    fn neutral_preserves_all_channel_values() {
        for value in 0..=255 {
            assert_eq!(modulate_rgb([value; 3], [128; 3]), [value; 3]);
        }
    }
    #[test]
    fn display_transfer_round_trips_bytes_and_is_not_linear_modulation() {
        for value in 0..=255 {
            let linear = srgb_byte_to_linear(value);
            let encoded = if linear <= 0.0031308 {
                linear * 12.92
            } else {
                1.055 * linear.powf(1. / 2.4) - 0.055
            };
            assert_eq!((encoded * 255.).round() as u8, value);
        }
        assert_eq!(modulate_rgb([128; 3], [64; 3]), [64; 3]);
        assert!((srgb_byte_to_linear(128) * 0.5 - srgb_byte_to_linear(64)).abs() > 0.05);
    }
}
