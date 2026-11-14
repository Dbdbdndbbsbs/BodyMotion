// After the vanilla `humanoid` pose (this hook runs on the player model's
// visibility call, right after it): apply the original mod's animations.
fn bm_visibility_head(p: &mut crate::mobs::anim::Poses) {
    crate::mobs::anim::native_bm_anim::apply_all(p);
}
