//! Original comparison instrumentation, not game camera behavior.
//! Contract: specs/inspection-camera-v1.md.
use crate::placement::PlacementError;

pub const NEAR: f32 = 0.01;
pub const FAR: f32 = 2000.;

pub fn parse_vec3(value: &str) -> Result<[f32; 3], PlacementError> {
    let values: Vec<_> = value.split(',').map(str::trim).collect();
    if values.len() != 3 {
        return Err(PlacementError("camera vector requires exactly X,Y,Z"));
    }
    let mut result = [0f32; 3];
    for (output, input) in result.iter_mut().zip(values) {
        *output = input
            .parse()
            .map_err(|_| PlacementError("invalid camera coordinate"))?;
        if !output.is_finite() {
            return Err(PlacementError("nonfinite camera coordinate"));
        }
    }
    Ok(result)
}

pub fn parse_view_size(value: &str) -> Result<[u32; 2], PlacementError> {
    let parts: Vec<_> = value.split(',').collect();
    if parts.len() != 2 {
        return Err(PlacementError("view size requires WIDTH,HEIGHT"));
    }
    let mut result = [0; 2];
    for (output, input) in result.iter_mut().zip(parts) {
        *output = input
            .trim()
            .parse()
            .map_err(|_| PlacementError("invalid view dimension"))?;
        if !(64..=4096).contains(output) {
            return Err(PlacementError("view dimensions must be within 64..4096"));
        }
    }
    if u64::from(result[0]) * u64::from(result[1]) > 4_194_304 {
        return Err(PlacementError("view pixel budget exceeded"));
    }
    Ok(result)
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a.into_iter().zip(b).map(|(a, b)| a * b).sum()
}
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

#[derive(Debug, Clone, Copy)]
pub struct InspectionCamera {
    eye: [f32; 3],
    forward: [f64; 3],
    right: [f64; 3],
    up: [f64; 3],
    fov: f32,
}

impl InspectionCamera {
    pub fn new(eye: [f32; 3], target: [f32; 3], fov: f32) -> Result<Self, PlacementError> {
        if !eye
            .iter()
            .chain(&target)
            .chain([&fov])
            .all(|v| v.is_finite())
            || !(10.0..=120.0).contains(&fov)
        {
            return Err(PlacementError(
                "invalid finite camera pose or FOV outside 10..120",
            ));
        }
        let delta = std::array::from_fn(|i| f64::from(target[i]) - f64::from(eye[i]));
        let length = dot(delta, delta).sqrt();
        if length == 0. || delta[0].hypot(delta[2]) < length * 1e-6 {
            return Err(PlacementError(
                "camera target coincides with eye or direction is parallel to Y-up",
            ));
        }
        let forward = delta.map(|v| v / length);
        let right = cross(forward, [0., 1., 0.]);
        let length = dot(right, right).sqrt();
        let right = right.map(|v| v / length);
        Ok(Self {
            eye,
            forward,
            right,
            up: cross(right, forward),
            fov,
        })
    }

    pub fn eye(self) -> [f32; 3] {
        self.eye
    }
    pub fn forward(self) -> [f32; 3] {
        self.forward.map(|v| v as f32)
    }
    pub fn fov_radians(self) -> f32 {
        self.fov.to_radians()
    }

    pub fn project(self, point: [f32; 3], size: [u32; 2]) -> Option<[f64; 2]> {
        if !point.iter().all(|v| v.is_finite()) || size.contains(&0) {
            return None;
        }
        let relative = std::array::from_fn(|i| f64::from(point[i]) - f64::from(self.eye[i]));
        let depth = dot(relative, self.forward);
        if depth < f64::from(NEAR) || depth > f64::from(FAR) {
            return None;
        }
        let half_y = (f64::from(self.fov) * std::f64::consts::PI / 360.).tan() * depth;
        let half_x = half_y * f64::from(size[0]) / f64::from(size[1]);
        Some([
            (dot(relative, self.right) / half_x + 1.) * f64::from(size[0]) * 0.5,
            (1. - dot(relative, self.up) / half_y) * f64::from(size[1]) * 0.5,
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_finite_vector_and_bounded_viewport_parsing() {
        assert_eq!(parse_vec3("1, -2,3.5").unwrap(), [1., -2., 3.5]);
        for s in ["1,2", "1,2,3,4", "1,2,NaN", "1,inf,3", "a,2,3", ",,", ""] {
            assert!(parse_vec3(s).is_err());
        }
        assert_eq!(parse_view_size("512,448").unwrap(), [512, 448]);
        for s in [
            "1,448",
            "4097,512",
            "4096,4096",
            "512,448,2",
            "512,0",
            "x,448",
        ] {
            assert!(parse_view_size(s).is_err());
        }
    }
    #[test]
    fn refuses_undefined_poses_and_fov() {
        for target in [
            [0.; 3],
            [0., 1., 0.],
            [0., -1., 0.],
            [1e-8, 1., 0.],
            [0., f32::NAN, 1.],
        ] {
            assert!(InspectionCamera::new([0.; 3], target, 45.).is_err());
        }
        for fov in [0., 9., 121., f32::NAN] {
            assert!(InspectionCamera::new([0.; 3], [0., 0., -1.], fov).is_err());
        }
        let extreme = InspectionCamera::new([f32::MAX, 0., 0.], [-f32::MAX, 0., 0.], 45.).unwrap();
        assert_eq!(extreme.forward(), [-1., 0., 0.]);
    }
    #[test]
    fn projects_center_right_up_and_leaves_offscreen_unclamped() {
        let c = InspectionCamera::new([0.; 3], [0., 0., -1.], 90.).unwrap();
        assert_eq!(c.project([0., 0., -10.], [512, 448]).unwrap(), [256., 224.]);
        let right = c.project([10., 0., -10.], [512, 448]).unwrap();
        assert!((right[0] - 480.).abs() < 1e-8);
        assert!(c.project([0., 10., -10.], [512, 448]).unwrap()[1].abs() < 1e-8);
        assert!(c.project([100., 0., -10.], [512, 448]).unwrap()[0] > 512.);
        assert!(c.project([0., 0., 10.], [512, 448]).is_none());
        assert!(c.project([0., 0., -0.001], [512, 448]).is_none());
        assert!(c.project([0., 0., -2001.], [512, 448]).is_none());
        assert!(c.project([f32::INFINITY, 0., -10.], [512, 448]).is_none());
    }
    #[test]
    fn opposite_heading_flips_world_x_not_screen_right() {
        let c = InspectionCamera::new([1., 2., 3.], [1., 2., 4.], 90.).unwrap();
        assert_eq!(c.project([1., 2., 13.], [512, 448]).unwrap(), [256., 224.]);
        assert!(c.project([-1., 2., 13.], [512, 448]).unwrap()[0] > 256.);
        assert!(c.project([1., 4., 13.], [512, 448]).unwrap()[1] < 224.);
    }
}
