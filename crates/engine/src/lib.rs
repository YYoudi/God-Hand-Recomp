//! Original GPU-free mechanics and inspection math; game behavior remains unverified.
pub mod gs_color;
pub mod inspection_camera;
pub mod placement;
pub const TICK_HZ: u32 = 60;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Move on X/Z; Y is height. Diagonal input cannot increase speed.
pub fn step(position: &mut Position, axis: [f32; 2], speed: f32, dt: f32) {
    if !axis.iter().chain([&speed, &dt]).all(|v| v.is_finite()) || speed < 0.0 || dt < 0.0 {
        return;
    }
    let length = axis[0].hypot(axis[1]).max(1.0);
    position.x += axis[0] / length * speed * dt;
    position.z += axis[1] / length * speed * dt;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn diagonal_speed_is_bounded_and_height_preserved() {
        let mut p = Position {
            y: 3.0,
            ..Default::default()
        };
        step(&mut p, [1.0, 1.0], 4.0, 0.5);
        assert!((p.x.hypot(p.z) - 2.0).abs() < 0.00001);
        assert_eq!(p.y, 3.0);
    }
    #[test]
    fn rejects_nonfinite_and_negative_steps() {
        let mut p = Position::default();
        step(&mut p, [f32::NAN, 1.0], 4.0, 1.0);
        step(&mut p, [1.0, 1.0], -1.0, 1.0);
        assert_eq!(p, Position::default());
    }
}
