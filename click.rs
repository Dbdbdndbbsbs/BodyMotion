// A click on the mod's settings page: Configure... (or Mod Menu's Configure)
// opens the config screen; the other buttons toggle their animation at once.
fn bm_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::client_ambient::native_bm_runtime as m;
    if mod_id != m::ID {
        return;
    }
    if button == crate::game::native_bm_screen::MODMENU_CONFIGURE || button == 0 {
        crate::game::native_bm_screen::open(game, button != 0);
        return;
    }
    crate::game::client_ambient::native_bm_glue::click(game, button - 1);
}
