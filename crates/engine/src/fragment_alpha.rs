//! Original GPU-free alpha-stage permissions, not final fragment visibility.
//! Contract/provenance: specs/fragment-write-permissions-v1.md.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlphaComparison {
    Never,
    Always,
    Less,
    LessEqual,
    Equal,
    GreaterEqual,
    Greater,
    NotEqual,
}

impl AlphaComparison {
    pub fn passes(self, alpha: u8, reference: u8) -> bool {
        match self {
            Self::Never => false,
            Self::Always => true,
            Self::Less => alpha < reference,
            Self::LessEqual => alpha <= reference,
            Self::Equal => alpha == reference,
            Self::GreaterEqual => alpha >= reference,
            Self::Greater => alpha > reference,
            Self::NotEqual => alpha != reference,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlphaFailure {
    Keep,
    FramebufferOnly,
    DepthOnly,
    RgbOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WritePermissions {
    pub rgb: bool,
    pub framebuffer_alpha: bool,
    pub depth: bool,
}

/// Permissions before other tests, component masks, blending and depth comparison.
#[derive(Debug, Clone, Copy)]
pub struct AlphaStage {
    pub enabled: bool,
    pub comparison: AlphaComparison,
    pub reference: u8,
    pub failure: AlphaFailure,
    pub depth_write_masked: bool,
}

impl AlphaStage {
    pub fn permissions(self, alpha: u8) -> WritePermissions {
        self.branch_permissions(!self.enabled || self.comparison.passes(alpha, self.reference))
    }

    /// Pipeline permissions for the selected comparison branch; not final pixel visibility.
    pub fn branch_permissions(self, passed: bool) -> WritePermissions {
        let (rgb, framebuffer_alpha, depth) = if passed {
            (true, true, true)
        } else {
            match self.failure {
                AlphaFailure::Keep => (false, false, false),
                AlphaFailure::FramebufferOnly => (true, true, false),
                AlphaFailure::DepthOnly => (false, false, true),
                AlphaFailure::RgbOnly => (true, false, false),
            }
        };
        WritePermissions {
            rgb,
            framebuffer_alpha,
            depth: depth && !self.depth_write_masked,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use AlphaComparison::*;
    use AlphaFailure::*;

    #[test]
    fn exhaustive_comparison_partitions_and_counts() {
        for reference in 0..=255u8 {
            let r = usize::from(reference);
            for (method, count) in [
                (Never, 0),
                (Always, 256),
                (Less, r),
                (LessEqual, r + 1),
                (Equal, 1),
                (GreaterEqual, 256 - r),
                (Greater, 255 - r),
                (NotEqual, 255),
            ] {
                let actual = (0..=255u8).filter(|&a| method.passes(a, reference)).count();
                assert_eq!(actual, count, "{method:?}, reference {reference}");
            }
            for alpha in 0..=255u8 {
                // Independent three-way truth table, not the implementation's operators.
                let expected = match (i16::from(alpha) - i16::from(reference)).signum() {
                    -1 => [false, true, true, true, false, false, false, true],
                    0 => [false, true, false, true, true, true, false, false],
                    1 => [false, true, false, false, false, true, true, true],
                    _ => unreachable!(),
                };
                for (method, result) in [
                    Never,
                    Always,
                    Less,
                    LessEqual,
                    Equal,
                    GreaterEqual,
                    Greater,
                    NotEqual,
                ]
                .into_iter()
                .zip(expected)
                {
                    assert_eq!(method.passes(alpha, reference), result);
                }
                assert_ne!(
                    Equal.passes(alpha, reference),
                    NotEqual.passes(alpha, reference)
                );
                assert_ne!(
                    Less.passes(alpha, reference),
                    GreaterEqual.passes(alpha, reference)
                );
                assert_ne!(
                    Greater.passes(alpha, reference),
                    LessEqual.passes(alpha, reference)
                );
                assert_eq!(
                    Equal.passes(alpha, reference),
                    alpha.wrapping_sub(reference) == 0
                );
            }
            assert!(!Less.passes(reference, reference));
            assert!(!Greater.passes(reference, reference));
            assert!(LessEqual.passes(reference, reference));
            assert!(GreaterEqual.passes(reference, reference));
        }
    }

    #[test]
    fn all_failure_actions_and_depth_mask_precedence() {
        for masked in [false, true] {
            for (failure, rgb, framebuffer_alpha, depth) in [
                (Keep, false, false, false),
                (FramebufferOnly, true, true, false),
                (DepthOnly, false, false, true),
                (RgbOnly, true, false, false),
            ] {
                let stage = AlphaStage {
                    enabled: true,
                    comparison: Never,
                    reference: 0,
                    failure,
                    depth_write_masked: masked,
                };
                assert_eq!(
                    stage.permissions(0),
                    WritePermissions {
                        rgb,
                        framebuffer_alpha,
                        depth: depth && !masked,
                    }
                );
            }
        }
    }

    #[test]
    fn disabled_or_passing_test_does_not_apply_failure_action() {
        for masked in [false, true] {
            for failure in [Keep, FramebufferOnly, DepthOnly, RgbOnly] {
                for (enabled, comparison) in [(false, Never), (true, Always)] {
                    let stage = AlphaStage {
                        enabled,
                        comparison,
                        reference: 255,
                        failure,
                        depth_write_masked: masked,
                    };
                    for alpha in [0, 127, 128, 255] {
                        assert_eq!(
                            stage.permissions(alpha),
                            WritePermissions {
                                rgb: true,
                                framebuffer_alpha: true,
                                depth: !masked,
                            }
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn greater_127_splits_color_and_depth_without_normalizing_alpha() {
        for failure in [Keep, FramebufferOnly] {
            let stage = AlphaStage {
                enabled: true,
                comparison: Greater,
                reference: 127,
                failure,
                depth_write_masked: false,
            };
            for alpha in 0..=255u8 {
                let writes = stage.permissions(alpha);
                assert_eq!(writes.depth, alpha >= 128);
                assert_eq!(writes.rgb, alpha >= 128 || failure == FramebufferOnly);
                assert_eq!(writes.framebuffer_alpha, writes.rgb);
            }
            assert_eq!(stage.reference, 127);
        }
    }
}
