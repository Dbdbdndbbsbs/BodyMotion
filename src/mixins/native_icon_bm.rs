// The game lists native mods without an icon (`ModInfo::of(m, None)` in
// `list_native_mods`). This mod's row gets its icon here, right after that
// list is built, so the Mods screen shows it like any content mod's. The PNG
// lives in data folder (`icon.png`), which the player can replace.
fn bm_native_icon(value: (), game: &mut crate::game::Game<'_>) {
    let _ = value;
    if let Some(info) = game.settings.mods.info.get_mut("bodymotion") {
        let path = crate::logging::game_home()
            .join("horizon-mods")
            .join(".horizon-mod-data")
            .join("bodymotion")
            .join("icon.png");
        if let Ok(bytes) = std::fs::read(path) {
            info.icon = Some(std::sync::Arc::from(bytes));
        }
    }
}
