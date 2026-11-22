// the mod's MapRendering: uploads map textures for all held maps before the mob
// geometry pass draws them. Inlined at the start of `entity_geometry`.
fn bm_warm_held_maps(game: &mut crate::game::Game<'_>) {
    crate::game::native_bm_maps::clear_frame();
    let mut ids = Vec::new();
    {
        let mut note = |stack: Option<&crate::items::ItemStack>| {
            if let Some(id) = stack.and_then(|s| s.data()).and_then(|d| d.map_id) {
                ids.push(id);
            }
        };
        for mob in &game.mobs {
            for off_hand in [false, true] {
                let stack =
                    horizon_client_lib::mobs::render::held_item_stack(&mob.extra, off_hand);
                note(stack.as_ref());
            }
        }
        note(game.__horizon_mixins_bodymotion_stack_in(game.selected));
        note(game.__horizon_mixins_bodymotion_stack_in(crate::game::shield::OFFHAND_SLOT));
    }
    for id in ids {
        if let Some(rect) = game.__horizon_mixins_bodymotion_map_texture(id) {
            crate::game::native_bm_maps::remember_rect(id, rect);
        }
    }
}
