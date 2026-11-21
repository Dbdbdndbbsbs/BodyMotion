fn bm_screen_render(
    game: &crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    screen: &mod_api::ScreenView<'_>,
) {
    crate::game::native_bm_screen::render(game, batch, screen);
}
