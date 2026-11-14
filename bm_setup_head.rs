// At the start of the model setup: capture the player's animation state, to
// be applied after the vanilla pose (the model-visibility hook).
fn bm_setup_head(m: &crate::mobs::Mob, s: &crate::mobs::anim::State) {
    crate::mobs::anim::native_bm_anim::set_state(
        crate::mobs::anim::native_bm_anim::BmState::capture(m, s),
    );
}
