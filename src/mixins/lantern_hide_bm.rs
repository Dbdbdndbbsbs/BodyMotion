// the mod's LanternAnimation: a lantern held in a player's (or mannequin's)
// third-person hand is drawn by `bm_held_lantern`, so the
// item model resolver returns an empty state for it and the flat item model
// is not drawn.
fn bm_hide_lantern(
    game: &crate::game::Game<'_>,
    stack: &crate::items::ItemStack,
    display: horizon_assets::item_model::DisplayContext,
    owner: Option<&horizon_assets::item_model::OwnerState>,
) -> core::ops::ControlFlow<ItemRenderState> {
    use bodymotion::glue as glue;
    if !glue::settings().animate_lanterns {
        return core::ops::ControlFlow::Continue(());
    }
    let third_person = matches!(
        display,
        horizon_assets::item_model::DisplayContext::ThirdPersonLeftHand
            | horizon_assets::item_model::DisplayContext::ThirdPersonRightHand
    );
    let avatar = owner.is_some_and(|o| {
        o.entity_type == "minecraft:player" || o.entity_type == "minecraft:mannequin"
    });
    let listed = glue::settings()
        .lantern_items
        .iter()
        .any(|i| i == &*stack.item);
    if third_person && avatar && listed {
        return core::ops::ControlFlow::Break(ItemRenderState {
            display,
            layers: Vec::new(),
            oversized_in_gui: false,
            animated: false,
            cit_glint: None,
            hdr_tier: 0,
            hdr_light: 0,
        });
    }
    core::ops::ControlFlow::Continue(())
}
