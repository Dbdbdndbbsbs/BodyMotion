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
            Mat4::from_rotation_x(deg(-87.0))
                * Mat4::from_rotation_y(deg(202.0))
                * Mat4::from_rotation_z(deg(13.0))
                * Mat4::from_translation(Vec3::new(f / 16.0, 0.23, -0.62)),
            if left {
                Mat4::from_rotation_y(deg(152.0))
                    * Mat4::from_rotation_z(deg(164.0))
                    * Mat4::from_scale(Vec3::splat(0.37))
                    * Mat4::from_translation(Vec3::new(0.57, -1.21, 0.14))
                    * Mat4::from_scale(Vec3::splat(0.013_4))
            } else {
                Mat4::from_rotation_y(deg(153.0))
                    * Mat4::from_rotation_z(deg(216.0))
                    * Mat4::from_scale(Vec3::splat(0.37))
                    * Mat4::from_translation(Vec3::new(-0.93, -1.76, 0.02))
                    * Mat4::from_scale(Vec3::splat(0.013_4))
            },
        )
    } else if left {
        // Left hand: the single-handed pose, held at the same distance as the
        // right so both hands sit level.
        (
            Mat4::from_rotation_x(deg(-88.0))
                * Mat4::from_rotation_y(deg(197.0))
                * Mat4::from_translation(Vec3::new(f / 16.0, 0.12, -0.61)),
            Mat4::from_rotation_y(deg(157.0))
                * Mat4::from_rotation_z(deg(183.0))
                * Mat4::from_scale(Vec3::splat(0.37))
                * Mat4::from_translation(Vec3::new(-0.12, -1.18, 0.01))
                * Mat4::from_scale(Vec3::splat(0.009_5)),
        )
    } else {
        // Right hand: the same pose, with a small Z push toward the face.
        (
            Mat4::from_rotation_x(deg(-88.0))
                * Mat4::from_rotation_y(deg(197.0))
                * Mat4::from_translation(Vec3::new(f / 16.0, 0.12, -0.61)),
            Mat4::from_rotation_y(deg(157.0))
                * Mat4::from_rotation_z(deg(183.0))
                * Mat4::from_scale(Vec3::splat(0.37))
                * Mat4::from_translation(Vec3::new(-0.12, -1.18, -0.19))
                * Mat4::from_scale(Vec3::splat(0.009_5)),
        )
    };
    hand * undo * outer * inner
}
