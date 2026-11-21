fn bm_screen_key_released(
    game: &crate::game::Game<'_>,
    screen: &mod_api::ScreenView<'_>,
    canceled: &mut bool,
) {
    *canceled |= crate::game::native_bm_screen::key_released(game, screen);
}
