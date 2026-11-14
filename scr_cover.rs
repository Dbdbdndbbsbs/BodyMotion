// The Options screen stays hidden under the config screen.
fn bm_screen_cover(game: &crate::game::Game<'_>) -> core::ops::ControlFlow<()> {
    if crate::game::native_bm_screen::covers(game) {
        return core::ops::ControlFlow::Break(());
    }
    core::ops::ControlFlow::Continue(())
}
