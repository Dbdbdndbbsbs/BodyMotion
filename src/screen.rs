// The mod's config screen over the Options screen: it draws itself in
// `screen_render_post` when the mod's own page in Options > Mods is open,
// takes all of the screen's input there, and Done, Reset and Escape close it.
// The page's own rows are hidden (`draw_options` is cancelled). Settings live
// in the glue cache; `Hooks::client_tick_pre` writes them when dirty.
// Spliced into game.rs as `native_bm_screen`.

use super::native_bm_gui as gui;
use super::{Game, Screen};
use horizon_client::hud::HudBatch;
use mod_api::ScreenView;
use std::cell::RefCell;

thread_local! {
    static SCREEN: RefCell<Option<gui::Gui>> = const { RefCell::new(None) };
    /// Mouse buttons whose press the screen took, so their release is taken too.
    static TAKEN: std::cell::Cell<u8> = const { std::cell::Cell::new(0) };
}

pub(super) fn dbg(_msg: &str) {}

fn with<R>(f: impl FnOnce(&mut gui::Gui) -> R) -> Option<R> {
    SCREEN.with(|s| s.borrow_mut().as_mut().map(f))
}

/// The mod's id in the installed list (the manifest's `id`).
const ID: &str = "bodymotion";

/// Whether the open screen is this mod's own page in Options > Mods.
fn mine(game: &Game<'_>) -> bool {
    use crate::options_ui::Page;
    game.screen == Screen::Options
        && match game.ui.options.page() {
            Page::NativeMod(i) => game
                .settings
                .mods
                .installed
                .get(i)
                .is_some_and(|m| m.id == ID),
            _ => false,
        }
}

/// The screen opens lazily on the first hook that fires for it.
fn ensure_open() {
    SCREEN.with(|s| {
        if s.borrow().is_none() {
            *s.borrow_mut() = Some(gui::Gui::new());
            TAKEN.with(|t| t.set(0));
        }
    });
}

/// Whether `draw_options` must be skipped: this mod's page is open, and the
/// config screen is drawn over it. The page's own rows are hidden even before
/// the screen has opened once (`frame` opens it the same frame), so the
/// registered rows never flash in.
pub(super) fn covers(game: &Game<'_>) -> bool {
    mine(game)
}

/// Once a frame (from `bm_frame.rs`): the sounds the screen asked for, and
/// its closing (Done, Reset's Done, and Escape set `closed`).
pub(super) fn frame(game: &mut Game<'_>) {
    if !mine(game) {
        // The page was left some other way (Back, another screen): the config
        // screen must not stay behind for the next visit.
        if SCREEN.with(|s| s.borrow().is_some()) {
            close();
        }
        return;
    }
    ensure_open();
    let actions = with(|g| std::mem::take(&mut g.actions)).unwrap_or_default();
    for action in actions {
        match action {
            gui::Action::Click => game.audio.play_button_click(),
        }
    }
    if with(|g| g.closed).unwrap_or(true) {
        close();
        // Back to the mods list. Closing the screen alone left its page open,
        // and `frame` opened the screen again on the next frame.
        let _ = game.ui.options.back(&mut game.settings);
    }
}

fn close() {
    SCREEN.with(|s| *s.borrow_mut() = None);
    TAKEN.with(|t| t.set(0));
}

fn over(game: &Game<'_>, screen: &ScreenView<'_>) -> bool {
    screen.id == "options" && mine(game)
}

fn bit(button: i32) -> u8 {
    1 << button.clamp(0, 7)
}

/// Draws the screen (the `screen_render_post` dispatcher).
pub(super) fn render(game: &Game<'_>, batch: &mut HudBatch, screen: &ScreenView<'_>) {
    if !over(game, screen) {
        return;
    }
    ensure_open();
    with(|g| g.draw(game, batch));
}

/// A mouse button pressed over the screen: true when it took it.
pub(super) fn pressed(game: &Game<'_>, screen: &ScreenView<'_>, button: i32, _modifiers: u32) -> bool {
    let over = over(game, screen);
    dbg(&format!("pressed over={} b={} cur=({},{})", over, button, game.cursor.0, game.cursor.1));
    if !over {
        return false;
    }
    TAKEN.with(|t| t.set(t.get() | bit(button)));
    let p = game.cursor;
    with(|g| g.press(game, p, button));
    true
}

/// A mouse button let go: taken when its press was.
pub(super) fn released(game: &Game<'_>, screen: &ScreenView<'_>, button: i32) -> bool {
    let taken = TAKEN.with(|t| {
        let was = t.get() & bit(button) != 0;
        t.set(t.get() & !bit(button));
        was
    });
    if !over(game, screen) {
        dbg(&format!("release over=false taken={}", taken));
        return taken;
    }
    let p = game.cursor;
    dbg(&format!("release taken={} cur=({},{})", taken, p.0, p.1));
    with(|g| g.release(game, p, button));
    true
}

/// The pointer dragged with a button held.
pub(super) fn dragged(game: &Game<'_>, screen: &ScreenView<'_>) -> bool {
    if !over(game, screen) {
        return false;
    }
    let p = game.cursor;
    with(|g| g.drag(game, p));
    true
}

/// The wheel.
pub(super) fn scrolled(game: &Game<'_>, screen: &ScreenView<'_>, scroll: [f64; 2]) -> bool {
    if !over(game, screen) {
        return false;
    }
    let p = game.cursor;
    with(|g| g.scroll(game, p, scroll));
    true
}

/// A key: the screen takes every one.
pub(super) fn key_pressed(game: &Game<'_>, screen: &ScreenView<'_>, key: &str, modifiers: u32) -> bool {
    let over = over(game, screen);
    dbg(&format!("key over={} key={}", over, key));
    if !over {
        return false;
    }
    with(|g| g.key(game, key, modifiers));
    true
}

/// A key let go: nothing under the screen hears it.
pub(super) fn key_released(game: &Game<'_>, screen: &ScreenView<'_>) -> bool {
    over(game, screen)
}

/// A character: into the search field when it is focused.
pub(super) fn char_typed(game: &Game<'_>, screen: &ScreenView<'_>, c: char) -> bool {
    if !over(game, screen) {
        return false;
    }
    with(|g| g.char_typed(c));
    true
}
