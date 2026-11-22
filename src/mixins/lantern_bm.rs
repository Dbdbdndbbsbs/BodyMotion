// Draws a lantern held in a player's (or mannequin's) third-person hand as its
// block model, hanging from a pivot above the hand and trailing the hand's
// motion. `lantern_hide_bm.rs` makes the resolver return an empty state for
// the same item so the flat item model is not drawn. The trail state lives in
// `native_bm_lantern` (lantern_glue.rs).
fn bm_held_lantern(
    game: &crate::game::Game<'_>,
    mob: &horizon_client_lib::mobs::Mob,
    camera: glam::DVec3,
    alpha: f64,
    light: u32,
    out: &mut Vec<crate::mesher::Vertex>,
) {
    use bodymotion::glue as glue;
    use crate::game::native_bm_lantern as lantern;
    let s = glue::settings();
    if !s.animate_lanterns || s.lantern_items.is_empty() {
        return;
    }
    if &*mob.kind.id != "minecraft:player"
        && mob.species() != horizon_client_lib::mobs::Species::Mannequin
    {
        return;
    }
    let data = game.res.mob_data();
    // The local player's own model draws the stack selected this frame, so read
    // the live slots. `mob.extra` updates on the next tick: reading it would
    // leave the previous lantern hanging in the air for a frame after a switch.
    let local_player = game
        .player_model
        .as_ref()
        .is_some_and(|p| std::ptr::eq(p, mob));
    for off_hand in [false, true] {
        let stack = if local_player {
            game.__horizon_mixins_bodymotion_stack_in(if off_hand {
                super::shield::OFFHAND_SLOT
            } else {
                game.selected
            })
            .cloned()
        } else {
            horizon_client_lib::mobs::render::held_item_stack(&mob.extra, off_hand)
        };
        let Some(stack) = stack else { continue };
        let item = &*stack.item;
        if !s.lantern_items.iter().any(|i| i.as_str() == item) {
            continue;
        }
        let left = off_hand != mob.main_arm_left();
        let Some(hand) = horizon_client_lib::mobs::render::held_item_transform(
            mob,
            alpha as f32,
            camera,
            data,
            left,
        ) else {
            continue;
        };
        // `held_item_transform` ends with `ItemInHandLayer`'s offsets; the
        // vanilla hangs the lantern from `translateToHand`, so undo them.
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
        // The view yaw the sway is measured against.
        let t = hand.w_axis;
        let vel = lantern::spring(
            mob,
            game.world.game_time,
            [t.x as f64, t.y as f64, t.z as f64],
            alpha as f32,
            mob.yaw as f64,
        );
        let yaw = mob.yaw.to_radians() as f64;
        let right = [yaw.cos(), 0.0, -yaw.sin()];
        let forward = [-yaw.sin(), 0.0, yaw.cos()];
        let off_right = vel[0] * right[0] + vel[2] * right[2];
        let off_forward = vel[0] * forward[0] + vel[2] * forward[2];
        let sway_roll = (off_right as f32 * 35.0).clamp(-26.0, 26.0);
        let sway_pitch = (off_forward as f32 * 35.0).clamp(-26.0, 26.0);
        // Scale down, move to the hang point, pivot to the top of the chain.
        // The pendulum pitches and rolls from the world-space offset, so the
        // lantern naturally sags below the hand.
        let m = hand
            * glam::Mat4::from_scale(glam::Vec3::splat(0.63))
            * glam::Mat4::from_translation(glam::Vec3::new(
                if left { -0.38 } else { -0.44 },
                0.36,
                -0.26,
            ))
            * glam::Mat4::from_translation(glam::Vec3::new(0.53, 0.57, 0.53))
            * glam::Mat4::from_rotation_x(deg(-92.0))
            * glam::Mat4::from_rotation_z(deg(sway_roll))
            * glam::Mat4::from_rotation_x(deg(sway_pitch * 0.6))
            * glam::Mat4::from_translation(glam::Vec3::new(-0.53, -0.57, -0.53));
        // The lantern item's block model (`Block.byItem(item).defaultBlockState()`).
        let Some(state) = game.pack.assets.registry.default_state(&item) else {
            continue;
        };
        game.block_layer_vertices(state, 0, m, 15790320, out);
    }
}
