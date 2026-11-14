// The mod's config screen over the Options screen, as Java's Mod
// Menu opens it and as the mod's page's first button does. While it is open on
// the mod's page it hides the options (`draw_options`), draws itself in
// `screen_render_post` and takes all of the screen's input; Done, Reset and
// Escape go back to the page, or out of the options when Mod Menu opened it.
// Also writes the icon and Mod Menu's metadata into the mod's data folder.
// Spliced into game.rs as `native_bm_screen`.

use super::native_bm_gui as gui;
use super::{Game, Screen};
use crate::game::client_ambient::native_bm_glue as glue;
use crate::game::client_ambient::native_bm_runtime as m;
use horizon_client::hud::HudBatch;
use horizon_client::mod_api::NativeScreenView;
use horizon_client::options_ui::Page;
use std::cell::{Cell, RefCell};

/// Mod Menu's Configure: `native_settings_click(id, MODMENU_CONFIGURE)` after it opened the mod's page.
pub const MODMENU_CONFIGURE: usize = 0x4D4D_4346;

/// Who opened the screen: the page's button, Mod Menu's Configure, or Options >
/// Mods' Settings button, which opens the screen in place of the page.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Page,
    ModMenu,
    Settings,
}

thread_local! {
    static OPEN: Cell<Option<Mode>> = const { Cell::new(None) };
    static SCREEN: RefCell<Option<gui::Gui>> = const { RefCell::new(None) };
    /// Mouse buttons whose press the screen took, so their release is taken too.
    static TAKEN: Cell<u8> = const { Cell::new(0) };
    /// The icon and the metadata were written this run.
    static WRITTEN: Cell<bool> = const { Cell::new(false) };
}

fn with<R>(f: impl FnOnce(&mut gui::Gui) -> R) -> Option<R> {
    SCREEN.with(|s| s.borrow_mut().as_mut().map(f))
}

/// The page's first button (Mod Menu's own `modmenu.configure`).
pub(super) fn configure_label(ru: bool) -> &'static str {
    glue::configure_label(ru)
}

/// The mod's page is the one the Options screen shows: its index in Options > Mods.
fn page(game: &Game<'_>) -> Option<usize> {
    if game.screen != Screen::Options {
        return None;
    }
    match game.ui.options.page() {
        Page::NativeMod(i) if game.settings.mods.installed.get(i).is_some_and(|mm| mm.id == m::ID) => Some(i),
        _ => None,
    }
}

/// The config screen is up over the mod's page: the page stays hidden and the
/// screen's input is the config screen's.
pub(super) fn covers(game: &Game<'_>) -> bool {
    OPEN.with(Cell::get).is_some() && page(game).is_some()
}

/// Opens the config screen over the mod's page (`settings_click`): button 0
/// of the page, or Mod Menu's Configure.
pub(super) fn open(game: &Game<'_>, from_mod_menu: bool) {
    open_mode(game, if from_mod_menu { Mode::ModMenu } else { Mode::Page });
}

/// Options > Mods' Settings button: the original screen opens instead of the
/// mod's page. Its Done and Escape go back to the mods list.
pub(super) fn open_from_settings(game: &Game<'_>) {
    open_mode(game, Mode::Settings);
}

fn open_mode(game: &Game<'_>, mode: Mode) {
    if page(game).is_none() || OPEN.with(Cell::get).is_some() {
        return;
    }
    SCREEN.with(|s| *s.borrow_mut() = Some(gui::Gui::new()));
    OPEN.with(|o| o.set(Some(mode)));
    TAKEN.with(|t| t.set(0));
}

fn close() {
    OPEN.with(|o| o.set(None));
    SCREEN.with(|s| *s.borrow_mut() = None);
}

/// Once a frame (from `bm_frame.rs`): the data files, what the screen asked
/// for (sounds), and its closing.
pub(super) fn frame(game: &mut Game<'_>) {
    if !WRITTEN.with(Cell::get) {
        WRITTEN.with(|w| w.set(true));
        write_data(game);
    }
    let Some(mode) = OPEN.with(Cell::get) else {
        return;
    };
    let Some(i) = page(game) else {
        // The options went elsewhere some other way: the screen goes, unsaved.
        close();
        return;
    };
    let actions = with(|g| std::mem::take(&mut g.actions)).unwrap_or_default();
    for action in actions {
        match action {
            gui::Action::Click => game.audio.play_button_click(),
        }
    }
    if with(|g| g.closed).unwrap_or(true) {
        close();
        match mode {
            // The mod's page again, its labels following the new values.
            Mode::Page => game.native_settings_refresh(i),
            // Back to the mods list, as the original's Done returned to Mod
            // Menu's list.
            Mode::Settings => {
                let events = game.ui.options.back(&mut game.settings);
                let _ = game.handle_option_events(events);
            }
            // Out of the options the way Escape goes: back to Mod Menu or the game.
            Mode::ModMenu => {
                let events = game.ui.options.back(&mut game.settings);
                let _ = game.handle_option_events(events);
            }
        }
    }
}

fn over(game: &Game<'_>, screen: &NativeScreenView<'_>) -> bool {
    screen.id == "options" && covers(game)
}

/// Draws the screen over the Options screen (`screen_render_post`).
pub(super) fn render(game: &Game<'_>, batch: &mut HudBatch, screen: &NativeScreenView<'_>) {
    if over(game, screen) {
        with(|g| g.draw(game, batch));
    }
}

fn bit(button: i32) -> u8 {
    1 << button.clamp(0, 7)
}

/// A mouse button pressed over the screen: true when it took it.
pub(super) fn pressed(game: &Game<'_>, screen: &NativeScreenView<'_>, button: i32, _modifiers: u32) -> bool {
    if !over(game, screen) {
        return false;
    }
    TAKEN.with(|t| t.set(t.get() | bit(button)));
    let p = (screen.mouse[0], screen.mouse[1]);
    with(|g| g.press(game, p, button));
    true
}

/// A mouse button let go: taken when its press was.
pub(super) fn released(game: &Game<'_>, screen: &NativeScreenView<'_>, button: i32) -> bool {
    let taken = TAKEN.with(|t| {
        let was = t.get() & bit(button) != 0;
        t.set(t.get() & !bit(button));
        was
    });
    if !over(game, screen) {
        return taken;
    }
    let p = (screen.mouse[0], screen.mouse[1]);
    with(|g| g.release(game, p, button));
    true
}

/// The pointer dragged with a button held.
pub(super) fn dragged(game: &Game<'_>, screen: &NativeScreenView<'_>) -> bool {
    if !over(game, screen) {
        return false;
    }
    let p = (screen.mouse[0], screen.mouse[1]);
    with(|g| g.drag(game, p));
    true
}

/// The wheel.
pub(super) fn scrolled(game: &Game<'_>, screen: &NativeScreenView<'_>, scroll: [f64; 2]) -> bool {
    if !over(game, screen) {
        return false;
    }
    let p = (screen.mouse[0], screen.mouse[1]);
    with(|g| g.scroll(game, p, scroll));
    true
}

/// A key: the screen takes every one.
pub(super) fn key_pressed(game: &Game<'_>, screen: &NativeScreenView<'_>, key: &str, modifiers: u32) -> bool {
    if !over(game, screen) {
        return false;
    }
    with(|g| g.key(game, key, modifiers));
    true
}

/// A key let go: nothing under the screen hears it.
pub(super) fn key_released(game: &Game<'_>, screen: &NativeScreenView<'_>) -> bool {
    over(game, screen)
}

/// A character: into the search field when it is focused.
pub(super) fn char_typed(game: &Game<'_>, screen: &NativeScreenView<'_>, c: char) -> bool {
    if !over(game, screen) {
        return false;
    }
    with(|g| g.char_typed(c));
    true
}

// ---- the icon and Mod Menu's metadata ---------------------------------------------------

/// The mod's icon (its `assets/bodymotion/icon.png`, 700x700).
const ICON: &[u8] = horizon_client::native_data!("bodymotion", "icon.png");

/// What Mod Menu shows of the mod.
fn modmenu_json() -> String {
    format!(
        r#"{{
  "schema": 1,
  "name": "{name}",
  "version": "{version}",
  "description": "{description}",
  "authors": ["romaeboj"],
  "license": "MIT",
  "links": {{}},
  "badges": [],
  "config_screen": true
}}
"#,
        name = m::NAME,
        version = m::VERSION,
        description = m::DESCRIPTION,
    )
}

/// `icon.png` and `modmenu.json` in the mod's data folder, each written only
/// when it differs from what is there.
fn write_data(game: &Game<'_>) {
    let dir = game.native_data_dir(m::ID);
    if std::fs::create_dir_all(&dir).is_err() {
        return;
    }
    let meta = modmenu_json();
    for (name, bytes) in [("icon.png", ICON), ("modmenu.json", meta.as_bytes())] {
        let path = dir.join(name);
        if std::fs::read(&path).ok().as_deref() != Some(bytes) {
            let _ = std::fs::write(&path, bytes);
        }
    }
}
