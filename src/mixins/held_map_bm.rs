// MapRendering: draws a filled map held in a player's or mannequin's hand
// instead of the flat item icon, and an empty map as its paper. Inlined at the
// start of `held_item_geometry`; `map_hide_bm` then makes the resolver return
// an empty state for the same third-person map so the flat icon is not drawn.
fn bm_held_map(
    game: &crate::game::Game<'_>,
    mob: &horizon_client_lib::mobs::Mob,
    camera: glam::DVec3,
    alpha: f64,
    light: u32,
    out: &mut Vec<crate::mesher::Vertex>,
) {
    use bodymotion::glue as glue;
    if !glue::settings().maps {
        return;
    }
    if &*mob.kind.id != "minecraft:player"
        && mob.species() != horizon_client_lib::mobs::Species::Mannequin
    {
        return;
    }
    // While a bow or crossbow is charging, both arms aim the weapon, so the
    // 3D map is skipped; `map_hide_bm` keeps the flat icon hidden too. The
    // same goes for a loaded crossbow held up in either hand. The game tracks
    // the loaded crossbow in the main hand only, so the local off hand reads
    // its stack here; a remote off hand has no stack data.
    let local_player = game
        .player_model
        .as_ref()
        .is_some_and(|p| std::ptr::eq(p, mob));
    let using_hand: Option<&str> = if mob.extra.using_off_hand {
        mob.extra.off_hand_id()
    } else {
        mob.extra.main_hand_id()
    };
    let main_charged =
        mob.extra.main_hand_id() == Some("minecraft:crossbow") && mob.extra.charged;
    let off_charged = local_player
        && game.offhand.as_ref().is_some_and(|s| {
            &*s.item == "minecraft:crossbow"
                && s.data().is_some_and(|d| !d.charged.is_empty())
        });
    let two_handed = (mob.extra.using_item > 0
        && matches!(
            using_hand,
            Some("minecraft:bow") | Some("minecraft:crossbow")
        ))
        || main_charged
        || off_charged;
    if two_handed {
        return;
    }
    let data = game.res.mob_data();
    for (item, off_hand) in [
        (mob.extra.main_hand_id(), false),
        (mob.extra.off_hand_id(), true),
    ] {
        let Some(item) = item else { continue };
        // An empty map stays vanilla: a flat item, never the 3D map render.
        if item == "minecraft:map" {
            continue;
        }
        let left = off_hand != mob.main_arm_left();
        let stack = if local_player {
            let Some(stack) = game.__horizon_mixins_bodymotion_stack_in(if off_hand {
                super::shield::OFFHAND_SLOT
            } else {
                game.selected
            }) else {
                continue;
            };
            stack.clone()
        } else {
            let Some(stack) =
                horizon_client_lib::mobs::render::held_item_stack(&mob.extra, off_hand)
            else {
                continue;
            };
            stack
        };
        let main_hand = !off_hand;
        let off_hand_empty = main_hand && mob.extra.off_hand_id().is_none();
        let Some(hand) =
            horizon_client_lib::mobs::render::held_item_transform(mob, alpha as f32, camera, data, left)
        else {
            continue;
        };
        let m = crate::game::native_bm_render::held_map_matrix(
            hand,
            left,
            main_hand,
            off_hand_empty,
        );
        let Some(id) = stack.data().and_then(|d| d.map_id) else {
            continue;
        };
        let Some(rect) = crate::game::native_bm_maps::map_rect(game, id) else {
            continue;
        };
        crate::game::native_bm_maps::map_quads(game, id, rect, false, m, light, out);
    }
}
