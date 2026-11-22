// MapAnimation: the pose of a filled map in a player's third-person hand,
// from the map pose helpers
// `MapRenderer.renderFirstPersonMap`. `hand` is `held_item_transform`'s matrix
// (the arm, then `ItemInHandLayer`'s offset), which this undoes to rebuild
// `translateToHand` chain.
use glam::{Mat4, Vec3};

/// The map pose for one arm, then
/// `MapRenderer.renderFirstPersonMap`. `left` is the arm; `main_hand` says
/// whether it is the main arm; `off_hand_empty` picks the two-handed pose.
pub fn held_map_matrix(hand: Mat4, left: bool, main_hand: bool, off_hand_empty: bool) -> Mat4 {
    let deg = |d: f32| d.to_radians();
    let f = if left { -1.0f32 } else { 1.0 };
    // Undo `ItemInHandLayer`: `X-90 * Y180 * T(+-1/16, 2/16, -10/16)`.
    let undo = Mat4::from_translation(Vec3::new(-f / 16.0, -2.0 / 16.0, 10.0 / 16.0))
        * Mat4::from_rotation_y(deg(-180.0))
        * Mat4::from_rotation_x(deg(90.0));
    let (outer, inner) = if main_hand && off_hand_empty {
        // Two-handed (one map, empty off hand): the map is held level in both
        // hands, chest-high.
        (
            Mat4::from_rotation_x(deg(-89.5))
                * Mat4::from_rotation_y(deg(203.0))
                * Mat4::from_rotation_z(deg(11.0))
                * Mat4::from_translation(Vec3::new(f / 16.0, 0.235, -0.62)),
            if left {
                Mat4::from_rotation_y(deg(156.0))
                    * Mat4::from_rotation_z(deg(165.0))
                    * Mat4::from_scale(Vec3::splat(0.375))
                    * Mat4::from_translation(Vec3::new(0.58, -1.22, 0.14))
                    * Mat4::from_scale(Vec3::splat(0.013_7))
            } else {
                Mat4::from_rotation_y(deg(153.5))
                    * Mat4::from_rotation_z(deg(215.0))
                    * Mat4::from_scale(Vec3::splat(0.375))
                    * Mat4::from_translation(Vec3::new(-0.95, -1.78, 0.02))
                    * Mat4::from_scale(Vec3::splat(0.013_7))
            },
        )
    } else if left {
        // Left hand: the single-handed pose, held at the same distance as the
        // right so both hands sit level.
        (
            Mat4::from_rotation_x(deg(-89.0))
                * Mat4::from_rotation_y(deg(201.5))
                * Mat4::from_translation(Vec3::new(f / 16.0, 0.13, -0.62)),
            Mat4::from_rotation_y(deg(158.5))
                * Mat4::from_rotation_z(deg(182.0))
                * Mat4::from_scale(Vec3::splat(0.375))
                * Mat4::from_translation(Vec3::new(-0.12, -1.18, 0.02))
                * Mat4::from_scale(Vec3::splat(0.009_75)),
        )
    } else {
        // Right hand: the same pose, with a small Z push toward the face.
        (
            Mat4::from_rotation_x(deg(-89.0))
                * Mat4::from_rotation_y(deg(201.5))
                * Mat4::from_translation(Vec3::new(f / 16.0, 0.13, -0.62)),
            Mat4::from_rotation_y(deg(161.5))
                * Mat4::from_rotation_z(deg(178.0))
                * Mat4::from_scale(Vec3::splat(0.375))
                * Mat4::from_translation(Vec3::new(-0.08, -1.22, -0.18))
                * Mat4::from_scale(Vec3::splat(0.009_75)),
        )
    };
    hand * undo * outer * inner
}
