// The pointer dragged with a button held: the config screen's scrollbar.
fn bm_screen_drag(
    game: &crate::game::Game<'_>,
    screen: &horizon_client::mod_api::NativeScreenView<'_>,
    canceled: &mut bool,
) {
    if crate::game::native_bm_screen::dragged(game, screen) {
        *canceled = true;
    }
}
