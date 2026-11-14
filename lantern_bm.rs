// the original mod's LanternAnimation: draws a lantern held in a player's (or mannequin's)
// third-person hand as its block model, hanging from a pivot above the hand
// and swinging with the player's movement. `lantern_hide_bm.rs` makes the
// resolver return an empty state for the same item so the flat item model is
// not drawn. The spring state lives in `native_bm_lantern` (lantern_glue.rs).
fn bm_held_lantern(
    game: &crate::game::Game<'_>,
    mob: &crate::mobs::Mob,
    camera: glam::DVec3,
    alpha: f64,
    light: u32,
    out: &mut Vec<crate::mesher::Vertex>,
) {
    use crate::game::client_ambient::native_bm_glue as glue;
    use crate::game::client_ambient::native_bm_lantern as lantern;
    let s = glue::settings(game);
    if !s.animate_lanterns || s.lantern_items.is_empty() {
        return;
    }
    if &*mob.kind.id != "minecraft:player"
        && mob.species() != crate::mobs::Species::Mannequin
    {
        return;
    }
    let data = game.res.mob_data();
    for (item, off_hand) in [
        (&mob.extra.main_hand, false),
        (&mob.extra.off_hand, true),
    ] {
        let Some(item) = item else { continue };
        if !s.lantern_items.iter().any(|i| i == &**item) {
            continue;
        }
        let left = off_hand != mob.main_arm_left();
        let Some(hand) = crate::mobs::render::held_item_transform(
            mob,
            alpha as f32,
            camera,
            data,
            left,
        ) else {
            continue;
        };
        // `held_item_transform` ends with `ItemInHandLayer`'s offsets; the
        // original hangs the lantern from `translateToHand`, so undo them.
        let f = if left { -1.0f32 } else { 1.0 };
        let deg = |d: f32| d.to_radians();
        let hand = hand
            * glam::Mat4::from_translation(glam::Vec3::new(
                -f / 16.0,
                -2.0 / 16.0,
                10.0 / 16.0,
            ))
            * glam::Mat4::from_rotation_y(deg(-180.0))
            * glam::Mat4::from_rotation_x(deg(90.0));
        // The view axes the swing is measured against.
        let yaw = mob.yaw.to_radians() as f64;
        let forward = [-yaw.sin(), 0.0, yaw.cos()];
        let right = [forward[2], 0.0, -forward[0]];
        let t = hand.w_axis;
        let vel = lantern::spring(
            mob,
            game.world.game_time,
            [t.x as f64, t.y as f64, t.z as f64],
            alpha as f32,
            forward,
            right,
        );
        let forward_vel = vel[0] * forward[0] + vel[2] * forward[2];
        let right_vel = vel[0] * right[0] + vel[2] * right[2];
        let swing_x = (forward_vel as f32 * 90.0).clamp(-90.0, 90.0) - mob.head_pitch * 0.25;
        let swing_z = (-(right_vel as f32) * 90.0).clamp(-90.0, 90.0);
        // Scale down, move to the hang point, pivot to the top of the chain,
        // the chain hangs down (-90 around X) and swings.
        let m = hand
            * glam::Mat4::from_scale(glam::Vec3::splat(0.6))
            * glam::Mat4::from_translation(glam::Vec3::new(
                if left { -0.45 } else { -0.5 },
                0.4,
                -0.3,
            ))
            * glam::Mat4::from_translation(glam::Vec3::new(0.5, 0.6, 0.5))
            * glam::Mat4::from_rotation_x(deg(-90.0))
            * glam::Mat4::from_rotation_x(deg(swing_x))
            * glam::Mat4::from_rotation_z(deg(swing_z))
            * glam::Mat4::from_translation(glam::Vec3::new(-0.5, -0.6, -0.5));
        // The lantern item's block model (`Block.byItem(item).defaultBlockState()`).
        let Some(state) = game.pack.assets.registry.default_state(&item) else {
            continue;
        };
        game.block_layer_vertices(state, 0, m, 15728880, out);
    }
}
