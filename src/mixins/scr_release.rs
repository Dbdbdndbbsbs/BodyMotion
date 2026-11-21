fn bm_screen_release(
    game: &crate::game::Game<'_>,
    screen: &mod_api::ScreenView<'_>,
    button: i32,
    canceled: &mut bool,
) {
    *canceled |= crate::game::native_bm_screen::released(game, screen, button);
}
