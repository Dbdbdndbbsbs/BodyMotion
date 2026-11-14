// The mod's game half: the mod's row in Options > Mods and its
// one setting. `runtime.rs` holds the setting; the animation itself is a
// source patch in the game (mobs/anim.rs, ArmPose::Eat).

use std::cell::RefCell;

use crate::game::client_ambient::native_bm_runtime as m;
use crate::game::Game;

struct State {
    settings: Option<m::Settings>,
}

thread_local! {
    static STATE: RefCell<Option<State>> = const { RefCell::new(None) };
}

// While the main hand holds a bow or crossbow, the off hand's item hides
// (the original mod's OffhandHidingAnimation); `hide_offhand_bm.rs` suppresses its
// vanilla render for the frame. A shield never hides.
thread_local! {
    static HIDE_OFFHAND: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static HIDE_MAIN: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

const ITEM_TAB_HOLDING: usize = m::ITEM_TAB_HOLDING;
const ITEM_TAB_LANTERN: usize = m::ITEM_TAB_LANTERN;

pub(crate) fn set_hide_offhand(v: bool) {
    HIDE_OFFHAND.with(|c| c.set(v));
}

pub(crate) fn hide_offhand() -> bool {
    HIDE_OFFHAND.with(|c| c.get())
}

pub(crate) fn set_hide_main(v: bool) {
    HIDE_MAIN.with(|c| c.set(v));
}

pub(crate) fn hide_main() -> bool {
    HIDE_MAIN.with(|c| c.get())
}

fn with<R>(f: impl FnOnce(&mut State) -> R) -> R {
    STATE.with(|cell| {
        f(cell
            .borrow_mut()
            .get_or_insert_with(|| State { settings: None }))
    })
}

pub(crate) fn settings(game: &Game<'_>) -> m::Settings {
    with(|s| {
        s.settings
            .get_or_insert_with(|| m::Settings::load(|key| game.native_setting(m::ID, key)))
            .clone()
    })
}

pub(crate) fn click(game: &Game<'_>, button: usize) {
    let mut s = settings(game);
    s.click(button);
    for (key, value) in s.saved() {
        game.native_set_setting(m::ID, &key, &value);
    }
    with(|st| st.settings = Some(s));
}

/// The settings page's buttons: one per animation, named in the game's
/// language (the original mod's names).
pub(crate) fn labels(game: &Game<'_>) -> Vec<String> {
    let s = settings(game);
    let lang = &game.settings.language;
    s.labels(lang)
}

/// Mod Menu's Configure (the page's first button's label).
pub(crate) fn configure_label(ru: bool) -> &'static str {
    if ru { "Настройки..." } else { "Configure..." }
}

/// The settings screen's toggle: flip `button` in the kept settings, applied
/// at once. Saved when the screen closes (`save`).
pub(crate) fn toggle(game: &Game<'_>, button: usize) {
    let mut s = settings(game);
    s.click(button);
    with(|st| st.settings = Some(s));
}

/// The settings screen's slider `i`: set it to `v` in the kept settings.
pub(crate) fn set_slider(game: &Game<'_>, i: usize, v: f32) {
    let mut s = settings(game);
    s.set_slider(i, v);
    with(|st| st.settings = Some(s));
}

/// The settings screen's enum button `i`: move to the next value.
pub(crate) fn cycle_choice(game: &Game<'_>, i: usize) {
    let mut s = settings(game);
    s.cycle_choice(i);
    with(|st| st.settings = Some(s));
}

/// The settings screen's Done: write every setting to the game.
pub(crate) fn save(game: &Game<'_>) {
    for (key, value) in settings(game).saved() {
        game.native_set_setting(m::ID, &key, &value);
    }
}

/// The settings screen's Reset: the defaults, written at once.
pub(crate) fn reset(game: &Game<'_>) {
    let s = m::Settings::default();
    for (key, value) in s.saved() {
        game.native_set_setting(m::ID, &key, &value);
    }
    with(|st| st.settings = Some(s));
}

/// Whether the item with the id `id` is in the tab's list.
pub(crate) fn item_on(s: &m::Settings, tab: usize, id: &str) -> bool {
    let list = if tab == ITEM_TAB_HOLDING {
        s.holding_items.as_slice()
    } else {
        s.lantern_items.as_slice()
    };
    list.iter().any(|i| i == id)
}

/// Toggle the item with the id `id` in the given tab's list and persist.
pub(crate) fn toggle_item(game: &Game<'_>, tab: usize, id: &str) {
    let mut s = settings(game);
    s.toggle_item(tab != ITEM_TAB_HOLDING, id);
    for (key, value) in s.saved() {
        game.native_set_setting(m::ID, &key, &value);
    }
    with(|st| st.settings = Some(s));
}

pub(crate) fn listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: Vec::new(),
    });
}
