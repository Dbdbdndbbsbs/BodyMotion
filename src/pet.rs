/// Converts a world-space offset into humanoid model space.
pub fn target_in_model(delta: [f32; 3], body_yaw: f32) -> [f32; 3] {
    let [dx, dy, dz] = delta;
    let yaw = f64::from((body_yaw - 180.0).to_radians());
    let (sin_y, cos_y) = (yaw.sin(), yaw.cos());
    [
        (f64::from(dx) * cos_y + f64::from(dz) * sin_y) as f32,
        dy,
        (-f64::from(dx) * sin_y + f64::from(dz) * cos_y) as f32,
    ]
}

/// Returns arm yaw and pitch toward target offset in humanoid model space.
pub fn aim_angles([x, y, z]: [f32; 3]) -> [f32; 2] {
    let horizontal = x.hypot(z).max(0.001);
    [x.atan2(-z), y.atan2(horizontal)]
}

pub fn lerp_degrees(from: f32, to: f32, alpha: f32) -> f32 {
    let delta = (to - from + 180.0).rem_euclid(360.0) - 180.0;
    from + delta * alpha
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_offsets_follow_interpolated_body_orientation() {
        let close = |actual: f32, expected: f32| (actual - expected).abs() < 1e-5;
        let front = target_in_model([0.0, 0.0, 1.0], 0.0);
        assert!(close(front[0], 0.0) && close(front[2], -1.0), "front: {front:?}");

        let right = target_in_model([1.0, 0.0, 0.0], 0.0);
        assert!(close(right[0], -1.0) && close(right[2], 0.0), "right: {right:?}");

        let turned_front = target_in_model([-1.0, 0.0, 0.0], 90.0);
        assert!(
            close(turned_front[0], 0.0) && close(turned_front[2], -1.0),
            "turned front: {turned_front:?}"
        );
    }

    #[test]
    fn aim_angles_point_toward_side_and_rear_targets() {
        let right = aim_angles([-1.0, 0.0, -1.0]);
        assert!(right[0] < 0.0, "right arm yaw: {}", right[0]);

        let left = aim_angles([1.0, 0.0, -1.0]);
        assert!(left[0] > 0.0, "left arm yaw: {}", left[0]);

        let behind = aim_angles([0.0, 0.0, 1.0]);
        assert!(close_to_pi(behind[0]), "rear arm yaw: {}", behind[0]);
    }

    #[test]
    fn body_yaw_interpolation_takes_shortest_path() {
        assert!((lerp_degrees(179.0, -179.0, 0.5).abs() - 180.0).abs() < 1e-5);
    }

    fn close_to_pi(value: f32) -> bool {
        (value.abs() - std::f32::consts::PI).abs() < 1e-5
    }
}
