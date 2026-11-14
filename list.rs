// The mod's row in Options > Mods.
fn bm_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    crate::game::client_ambient::native_bm_glue::listed(mods);
}
