fn bm_screen_wheel(
    game: &crate::game::Game<'_>,
    screen: &mod_api::ScreenView<'_>,
    scroll: [f64; 2],
    canceled: &mut bool,
) {
    *canceled |= crate::game::native_bm_screen::scrolled(game, screen, scroll);
}
