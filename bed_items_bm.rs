// the original mod `dontHoldItemsInBed`: a sleeping player (or villager) holds no item.
// `held_item_geometry` draws both hands' items, so cancelling it drops the
// whole ItemInHandLayer for the frame.
fn bm_bed_items(
    game: &crate::game::Game<'_>,
    mob: &crate::mobs::Mob,
) -> core::ops::ControlFlow<()> {
    use crate::game::client_ambient::native_bm_glue as glue;
    if !glue::settings(game).dont_hold_items_in_bed {
        return core::ops::ControlFlow::Continue(());
    }
    if crate::mobs::villager::sleep_pose(mob).is_some() {
        return core::ops::ControlFlow::Break(());
    }
    core::ops::ControlFlow::Continue(())
}
