// the mod `dontHoldItemsInBed`: a sleeping player (or villager) holds no item.
// `held_item_geometry` draws both hands' items, so cancelling it drops the
// whole ItemInHandLayer for the frame.
fn bm_bed_items(
    game: &crate::game::Game<'_>,
    mob: &horizon_client_lib::mobs::Mob,
) -> core::ops::ControlFlow<()> {
    use bodymotion::glue as glue;
    if !glue::settings().dont_hold_items_in_bed {
        return core::ops::ControlFlow::Continue(());
    }
    if horizon_client_lib::mobs::villager::sleep_pose(mob).is_some() {
        return core::ops::ControlFlow::Break(());
    }
    core::ops::ControlFlow::Continue(())
}
