fn bm_screen_press(
    game: &crate::game::Game<'_>,
    screen: &mod_api::ScreenView<'_>,
    button: i32,
    modifiers: u32,
    canceled: &mut bool,
) {
    *canceled |= crate::game::native_bm_screen::pressed(game, screen, button, modifiers);
}
