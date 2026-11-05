//! Original byte-input source-over arithmetic; not a full GS color pipeline.
//! Contract: specs/byte-source-over-v1.md.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ByteBlend {
    #[default]
    Overwrite,
    SourceOver,
}
impl ByteBlend {
    pub fn rgb(self, source: [u8; 4], destination: [u8; 3]) -> [u8; 3] {
        std::array::from_fn(|i| match self {
            Self::Overwrite => source[i],
            Self::SourceOver => {
                let difference = i32::from(source[i]) - i32::from(destination[i]);
                let result = i32::from(destination[i]) + ((difference * i32::from(source[3])) >> 7);
                result.clamp(0, 255) as u8
            }
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_byte_triples_match_weighted_rational_oracle() {
        for source in 0..=255u8 {
            for destination in 0..=255u8 {
                for alpha in 0..=255u8 {
                    let numerator = i64::from(source) * i64::from(alpha)
                        + i64::from(destination) * (128 - i64::from(alpha));
                    let expected = numerator.div_euclid(128).clamp(0, 255) as u8;
                    assert_eq!(
                        ByteBlend::SourceOver
                            .rgb([source, source, source, alpha], [destination; 3]),
                        [expected; 3]
                    );
                }
            }
        }
    }
    #[test]
    fn alpha_unity_extrapolation_and_negative_floor_are_explicit() {
        assert_eq!(
            ByteBlend::SourceOver.rgb([0, 255, 100, 0], [255, 0, 20]),
            [255, 0, 20]
        );
        assert_eq!(
            ByteBlend::SourceOver.rgb([17, 83, 249, 128], [255, 0, 20]),
            [17, 83, 249]
        );
        assert_eq!(
            ByteBlend::SourceOver.rgb([0, 255, 100, 255], [255, 0, 20]),
            [0, 255, 179]
        );
        assert_eq!(
            ByteBlend::SourceOver.rgb([0, 2, 4, 1], [1, 1, 1]),
            [0, 1, 1]
        );
    }
    #[test]
    fn overwrite_ignores_alpha_and_destination() {
        for alpha in 0..=255u8 {
            assert_eq!(
                ByteBlend::Overwrite.rgb([20, 40, 60, alpha], [255, 17, 0]),
                [20, 40, 60]
            );
        }
    }
}
