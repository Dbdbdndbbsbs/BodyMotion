// The mod's settings page: Configure... first (the config screen), then one
// button per animation, on or off. Options > Mods' Settings button opens the
// original screen directly (`open_from_settings`).
fn bm_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::client_ambient::native_bm_runtime as m;
    if mod_id != m::ID {
        return;
    }
    crate::game::native_bm_screen::open_from_settings(game);
    let ru = m::russian(&game.settings.language);
    buttons.push(crate::game::native_bm_screen::configure_label(ru).to_string());
    buttons.extend(crate::game::client_ambient::native_bm_glue::labels(game));
}
