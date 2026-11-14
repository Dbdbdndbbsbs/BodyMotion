// A character typed: the config screen has no text field.
fn bm_screen_char(
    game: &crate::game::Game<'_>,
    screen: &horizon_client::mod_api::NativeScreenView<'_>,
    c: char,
    canceled: &mut bool,
) {
    if !*canceled && crate::game::native_bm_screen::char_typed(game, screen, c) {
        *canceled = true;
    }
}
