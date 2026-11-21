fn bm_screen_drag(
    game: &crate::game::Game<'_>,
    screen: &mod_api::ScreenView<'_>,
    canceled: &mut bool,
) {
    *canceled |= crate::game::native_bm_screen::dragged(game, screen);
}
