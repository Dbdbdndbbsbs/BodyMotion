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
            Mat4::from_rotation_x(deg(-76.0))
                * Mat4::from_rotation_y(deg(188.0))
                * Mat4::from_rotation_z(deg(16.0))
                * Mat4::from_translation(Vec3::new(f / 16.0, 0.26, -0.58)),
            if left {
                Mat4::from_rotation_y(deg(140.0))
                    * Mat4::from_rotation_z(deg(152.0))
                    * Mat4::from_scale(Vec3::splat(0.34))
                    * Mat4::from_translation(Vec3::new(0.52, -1.1, 0.1))
                    * Mat4::from_scale(Vec3::splat(0.012_2))
            } else {
                Mat4::from_rotation_y(deg(146.0))
                    * Mat4::from_rotation_z(deg(198.0))
                    * Mat4::from_scale(Vec3::splat(0.34))
                    * Mat4::from_translation(Vec3::new(-0.88, -1.62, 0.02))
                    * Mat4::from_scale(Vec3::splat(0.012_2))
            },
        )
    } else if left {
        // Left hand: the single-handed pose, held at the same distance as the
        // right so both hands sit level.
        (
            Mat4::from_rotation_x(deg(-78.0))
                * Mat4::from_rotation_y(deg(194.0))
                * Mat4::from_translation(Vec3::new(f / 16.0, 0.14, -0.58)),
            Mat4::from_rotation_y(deg(150.0))
                * Mat4::from_rotation_z(deg(170.0))
                * Mat4::from_scale(Vec3::splat(0.34))
                * Mat4::from_translation(Vec3::new(-0.08, -1.12, 0.0))
                * Mat4::from_scale(Vec3::splat(0.009)),
        )
    } else {
        // Right hand: the same pose, with a small Z push toward the face.
        (
            Mat4::from_rotation_x(deg(-78.0))
                * Mat4::from_rotation_y(deg(194.0))
                * Mat4::from_translation(Vec3::new(f / 16.0, 0.14, -0.58)),
            Mat4::from_rotation_y(deg(152.0))
                * Mat4::from_rotation_z(deg(176.0))
                * Mat4::from_scale(Vec3::splat(0.34))
                * Mat4::from_translation(Vec3::new(-0.06, -1.14, -0.22))
                * Mat4::from_scale(Vec3::splat(0.009)),
        )
    };
    hand * undo * outer * inner
}
