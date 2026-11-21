fn bm_screen_key_pressed(
    game: &crate::game::Game<'_>,
    screen: &mod_api::ScreenView<'_>,
    key: &str,
    modifiers: u32,
    canceled: &mut bool,
) {
    *canceled |= crate::game::native_bm_screen::key_pressed(game, screen, key, modifiers);
}
