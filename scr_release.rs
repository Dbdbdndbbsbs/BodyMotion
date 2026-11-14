// A mouse button let go: taken when the config screen took its press.
fn bm_screen_release(
    game: &crate::game::Game<'_>,
    screen: &horizon_client::mod_api::NativeScreenView<'_>,
    button: i32,
    canceled: &mut bool,
) {
    if crate::game::native_bm_screen::released(game, screen, button) {
        *canceled = true;
    }
}
