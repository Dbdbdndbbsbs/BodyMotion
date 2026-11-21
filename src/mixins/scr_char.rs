fn bm_screen_char(
    game: &crate::game::Game<'_>,
    screen: &mod_api::ScreenView<'_>,
    c: char,
    canceled: &mut bool,
) {
    *canceled |= crate::game::native_bm_screen::char_typed(game, screen, c);
}
