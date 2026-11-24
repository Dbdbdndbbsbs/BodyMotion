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
        // The pivot is the fist: the palm plus a small own grip offset. The
        // lantern hangs from its handle's top ring, so the block is placed by
        // the swing direction alone; no item-layer offsets are undone.
        let palm = hand.w_axis.truncate();
        let pivot = palm + glam::Vec3::Y * 0.02;
        // Remove the camera-relative entity origin before projecting into the
        // body frame. Projecting the absolute hand position makes a body yaw
        // change look like arm motion, even when the hand is fixed to the body.
        let origin = horizon_client_lib::mobs::render::base_transform(
            mob,
            alpha as f32,
            camera,
        )
        .w_axis
        .truncate();
        let relative_hand = palm - origin;
        // The body's own frame: head turns and body turns do not feed the
        // pendulum; only the hand's position relative to the torso does.
        let body_yaw = horizon_client_lib::mobs::anim::body_rot(mob, alpha as f32);
        let yaw = body_yaw.to_radians();
        let right = glam::Vec3::new(-yaw.cos(), 0.0, -yaw.sin());
        let forward = glam::Vec3::new(-yaw.sin(), 0.0, yaw.cos());
        let local = [
            relative_hand.dot(right) as f64,
            relative_hand.y as f64,
            relative_hand.dot(forward) as f64,
        ];
        let angles = lantern::pendulum(
            mob,
            game.world.game_time,
            local,
            alpha as f32,
            off_hand,
        );
        // The handle swings away from vertical in the body's frame; the
        // block's local +Y points back up the suspension, toward the pivot.
        let hang = (glam::Vec3::NEG_Y
            + right * angles[0].sin()
            + forward * angles[1].sin())
        .normalize_or_zero();
        let up = -hang;
        // Keep the block's own yaw on the body, so it never spins against the
        // hand while the player turns.
        let side = (right - up * right.dot(up)).normalize_or_zero();
        let back = side.cross(up);
        let rot = glam::Mat4::from_cols(
            side.extend(0.0),
            up.extend(0.0),
            back.extend(0.0),
            glam::Vec4::W,
        );
        // The handle's top is at 11/16 of the block model, so shifting the
        // model down by that puts the ring in the fist.
        let m = glam::Mat4::from_translation(pivot)
            * rot
            * glam::Mat4::from_scale(glam::Vec3::splat(0.55))
            * glam::Mat4::from_translation(glam::Vec3::new(-0.5, -0.6875, -0.5));
        // The lantern item's block model (`Block.byItem(item).defaultBlockState()`).
        let Some(state) = game.pack.assets.registry.default_state(&item) else {
            continue;
        };
        game.block_layer_vertices(state, 0, m, 15790320, out);
    }
}
