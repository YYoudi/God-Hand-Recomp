//! Explicit placement hypotheses, not established game-runtime semantics.
//! Contract and provenance: specs/scr-placement-v1.md.
use std::{error::Error, fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EulerOrder {
    Xyz,
    Xzy,
    Yxz,
    Yzx,
    Zxy,
    Zyx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlacementError(pub &'static str);

impl fmt::Display for PlacementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}
impl Error for PlacementError {}

impl FromStr for EulerOrder {
    type Err = PlacementError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "xyz" => Ok(Self::Xyz),
            "xzy" => Ok(Self::Xzy),
            "yxz" => Ok(Self::Yxz),
            "yzx" => Ok(Self::Yzx),
            "zxy" => Ok(Self::Zxy),
            "zyx" => Ok(Self::Zyx),
            _ => Err(PlacementError(
                "rotation order must be xyz/xzy/yxz/yzx/zxy/zyx",
            )),
        }
    }
}

impl EulerOrder {
    fn axes(self) -> [usize; 3] {
        match self {
            Self::Xyz => [0, 1, 2],
            Self::Xzy => [0, 2, 1],
            Self::Yxz => [1, 0, 2],
            Self::Yzx => [1, 2, 0],
            Self::Zxy => [2, 0, 1],
            Self::Zyx => [2, 1, 0],
        }
    }
}

#[derive(Debug)]
pub struct Placement {
    scale: [f64; 3],
    rotation: [(f64, f64); 3],
    translation: [f64; 3],
    axes: [usize; 3],
}

impl Placement {
    pub fn new(
        scale: [f32; 3],
        rotation: [f32; 3],
        translation: [f32; 3],
        order: Option<EulerOrder>,
    ) -> Result<Self, PlacementError> {
        if !scale
            .iter()
            .chain(&rotation)
            .chain(&translation)
            .all(|v| v.is_finite())
        {
            return Err(PlacementError("placement contains nonfinite data"));
        }
        if order.is_none() && rotation.iter().filter(|&&v| v != 0.0).count() > 1 {
            return Err(PlacementError(
                "ambiguous multiaxis rotation: choose an explicit diagnostic order",
            ));
        }
        Ok(Self {
            scale: scale.map(f64::from),
            rotation: rotation.map(|v| f64::from(v).sin_cos()),
            translation: translation.map(f64::from),
            // No order inference: for <= 1 active axis every sequence agrees.
            axes: order.unwrap_or(EulerOrder::Xyz).axes(),
        })
    }

    pub fn point(&self, point: [f32; 3]) -> Result<[f32; 3], PlacementError> {
        if !point.iter().all(|v| v.is_finite()) {
            return Err(PlacementError("placement point contains nonfinite data"));
        }
        let mut value = std::array::from_fn::<_, 3, _>(|i| f64::from(point[i]) * self.scale[i]);
        for axis in self.axes {
            let (sin, cos) = self.rotation[axis];
            // Cyclic axis pair gives the same right-handed formula for X/Y/Z.
            let a = (axis + 1) % 3;
            let b = (axis + 2) % 3;
            let (x, y) = (value[a], value[b]);
            value[a] = x * cos - y * sin;
            value[b] = x * sin + y * cos;
        }
        let output = std::array::from_fn(|i| (value[i] + self.translation[i]) as f32);
        if !output.iter().all(|v| v.is_finite()) {
            return Err(PlacementError("placement result exceeds finite f32 range"));
        }
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::FRAC_PI_2;

    fn near(actual: [f32; 3], expected: [f32; 3]) {
        for (a, e) in actual.into_iter().zip(expected) {
            assert!(
                (a - e).abs() < 1e-5,
                "actual {actual:?}, expected {expected:?}"
            );
        }
    }

    #[test]
    fn identity_and_literal_zero_negative_nonuniform_scales() {
        near(
            Placement::new([1.; 3], [0.; 3], [0.; 3], None)
                .unwrap()
                .point([1., -2., 3.])
                .unwrap(),
            [1., -2., 3.],
        );
        near(
            Placement::new([-2., 0., 3.], [0.; 3], [4., 5., 6.], None)
                .unwrap()
                .point([1., 2., 3.])
                .unwrap(),
            [2., 5., 15.],
        );
    }

    #[test]
    fn signed_quarter_turns_about_each_axis() {
        for axis in 0..3 {
            for sign in [-1., 1.] {
                let mut r = [0.; 3];
                r[axis] = sign * FRAC_PI_2;
                let mut p = [0.; 3];
                p[(axis + 1) % 3] = 1.;
                let mut e = [0.; 3];
                e[(axis + 2) % 3] = sign;
                near(
                    Placement::new([1.; 3], r, [0.; 3], None)
                        .unwrap()
                        .point(p)
                        .unwrap(),
                    e,
                );
            }
        }
    }

    #[test]
    fn scale_then_rotate_then_translate_not_the_reverse() {
        near(
            Placement::new([2., 3., 4.], [0., 0., FRAC_PI_2], [10., 20., 30.], None)
                .unwrap()
                .point([1., 2., 3.])
                .unwrap(),
            [4., 22., 42.],
        );
    }

    #[test]
    fn six_noncommuting_orders_match_hand_computed_quarter_turns() {
        // Starting (1,2,3), each +90-degree rotation only permutes/signs axes.
        for (name, expected) in [
            ("xyz", [3., 2., -1.]),
            ("xzy", [2., 1., -3.]),
            ("yxz", [-1., 3., 2.]),
            ("yzx", [-2., 1., 3.]),
            ("zxy", [1., -3., 2.]),
            ("zyx", [3., -2., 1.]),
        ] {
            near(
                Placement::new(
                    [1.; 3],
                    [FRAC_PI_2; 3],
                    [0.; 3],
                    Some(name.parse().unwrap()),
                )
                .unwrap()
                .point([1., 2., 3.])
                .unwrap(),
                expected,
            );
        }
    }

    #[test]
    fn refuses_ambiguous_order_even_for_small_nonzero_angles() {
        assert!(Placement::new([1.; 3], [0., 0.1, 0.2], [0.; 3], None).is_err());
        assert!(Placement::new([1.; 3], [f32::MIN_POSITIVE, 0.1, 0.], [0.; 3], None).is_err());
        assert!("automatic".parse::<EulerOrder>().is_err());
    }

    #[test]
    fn refuses_nonfinite_input_and_finite_input_overflow() {
        for value in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            assert!(Placement::new([value; 3], [0.; 3], [0.; 3], None).is_err());
            assert!(Placement::new([1.; 3], [value; 3], [0.; 3], Some(EulerOrder::Xyz)).is_err());
            assert!(Placement::new([1.; 3], [0.; 3], [value; 3], None).is_err());
            assert!(
                Placement::new([1.; 3], [0.; 3], [0.; 3], None)
                    .unwrap()
                    .point([value; 3])
                    .is_err()
            );
        }
        assert!(
            Placement::new([f32::MAX; 3], [0.; 3], [0.; 3], None)
                .unwrap()
                .point([2.; 3])
                .is_err()
        );
    }
}
