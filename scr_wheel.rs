// The wheel over a screen: the config screen's list.
fn bm_screen_wheel(
    game: &crate::game::Game<'_>,
    screen: &horizon_client::mod_api::NativeScreenView<'_>,
    scroll: [f64; 2],
    canceled: &mut bool,
) {
    if !*canceled && crate::game::native_bm_screen::scrolled(game, screen, scroll) {
        *canceled = true;
    }
}
