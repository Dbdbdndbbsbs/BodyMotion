// settings screen, built like the game's own GUI screens. As in the
// the root panel has a fixed size and is centred on the screen; the game's GUI
// scale handles how large it looks, so nothing here re-scales with the window.
// This is the GUI redrawn pixel by pixel: the root panel, one `Settings` tab
// with the filled-map icon, the 12-row list (two headings, ten toggles), Done
// and Reset. Clicks act on release.
// Spliced into game.rs as `native_bm_gui`.

use super::native_bm_sprites as sprites;
use super::native_bm_screen::dbg;
use bodymotion::glue as glue;
use bodymotion::runtime as m;
use super::Game;
use horizon_client::hud::{rgba, HudAtlas, HudBatch};
use horizon_client::items::ItemStack;
use horizon_client::text::Text;
use horizon_client::ui;
use std::time::Instant;

thread_local! {
    static DUMPED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static DRAW_CNT: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
}

/// What the screen asks of `frame`.
pub(super) enum Action {
    Click,
}

// ---- drawing helpers -----------------------------------------------------------

/// An ARGB colour (as the Java code writes them) as the game packs colours.
fn c(argb: u32) -> u32 {
    rgba((argb >> 16) as u8, (argb >> 8) as u8, argb as u8, (argb >> 24) as u8)
}

fn fill(batch: &mut HudBatch, atlas: &HudAtlas, x: i32, y: i32, w: i32, h: i32, argb: u32) {
    if w > 0 && h > 0 {
        batch.fill(atlas, x as f32, y as f32, w as f32, h as f32, c(argb));
    }
}

fn width(atlas: &HudAtlas, t: &Text) -> i32 {
    atlas.font.width_text(&t.resolved()) as i32
}

fn text(batch: &mut HudBatch, atlas: &HudAtlas, t: &Text, x: i32, y: i32, argb: u32, shadow: bool) {
    batch.text_styled(atlas, t, x as f32, y as f32, c(argb), shadow);
}

/// A texture over `w` x `h`, nine-sliced with edges and centre tile
/// from their start (`blitNineSlicedSprite`).
fn sprite(batch: &mut HudBatch, atlas: &HudAtlas, s: &sprites::Sprite, x: i32, y: i32, w: i32, h: i32) {
    let (sw, sh) = (i32::from(s.w), i32::from(s.h));
    for dy in 0..h {
        let sy = slice(dy, h, sh);
        let at = |dx: i32| s.px[(sy * sw + slice(dx, w, sw)) as usize];
        let mut dx = 0;
        while dx < w {
            let raw = at(dx);
            let mut len = 1;
            while dx + len < w && at(dx + len) == raw {
                len += 1;
            }
            if raw >> 24 != 0 {
                batch.fill(atlas, (x + dx) as f32, (y + dy) as f32, len as f32, 1.0, c(raw));
            }
            dx += len;
        }
    }
}

/// A destination pixel's source pixel in a nine-slice whose edges and centre
/// tile come from their start (no border: a plain copy).
fn slice(d: i32, dst: i32, src: i32) -> i32 {
    if d < dst - src {
        d
    } else {
        src - (dst - d)
    }
}

/// A 1-px bevel: top and left edge, bottom and
/// right edge, the corner pixel, the inside.
fn bevel(batch: &mut HudBatch, atlas: &HudAtlas, x: i32, y: i32, w: i32, h: i32, colors: [u32; 4]) {
    let [edge, bottom, corner, inside] = colors;
    fill(batch, atlas, x, y, w - 1, 1, edge);
    fill(batch, atlas, x + w - 1, y, 1, 1, corner);
    fill(batch, atlas, x, y + 1, 1, h - 2, edge);
    fill(batch, atlas, x + 1, y + 1, w - 2, h - 2, inside);
    fill(batch, atlas, x + w - 1, y + 1, 1, h - 2, bottom);
    fill(batch, atlas, x, y + h - 1, 1, 1, corner);
    fill(batch, atlas, x + 1, y + h - 1, w - 1, 1, bottom);
}

/// `BackgroundPainter.VANILLA` (`panel_vanilla_modern`): a black frame around
/// a `0x7A000000` fill.
fn panel(batch: &mut HudBatch, atlas: &HudAtlas, x: i32, y: i32, w: i32, h: i32) {
    fill(batch, atlas, x, y, w, 1, 0xFF00_0000);
    fill(batch, atlas, x, y + h - 1, w, 1, 0xFF00_0000);
    fill(batch, atlas, x, y + 1, 1, h - 2, 0xFF00_0000);
    fill(batch, atlas, x + w - 1, y + 1, 1, h - 2, 0xFF00_0000);
    fill(batch, atlas, x + 1, y + 1, w - 2, h - 2, 0x7A00_0000);
}

/// One tab: `selected_vanilla_modern` (selected) or `unselected_vanilla_modern`
/// (unselected) — rounded 4-px corner, open at the bottom.
fn tab_rect(batch: &mut HudBatch, atlas: &HudAtlas, x: i32, y: i32, w: i32, selected: bool) {
    let a = |selected_alpha: u32, unselected_alpha: u32| {
        if selected { selected_alpha << 24 } else { unselected_alpha << 24 }
    };
    let black = 0xFF00_0000;
    fill(batch, atlas, x + 2, y, w - 4, 1, black);
    fill(batch, atlas, x + 1, y + 1, 1, 1, black);
    fill(batch, atlas, x + 2, y + 1, w - 4, 1, a(0xAB, 0x6D));
    fill(batch, atlas, x + w - 2, y + 1, 1, 1, black);
    for (row, inner, middle) in [
        (2, a(0xBD, 0x7D), a(0xBA, 0x7A)),
        (18, a(0xBD, 0x7D), a(0xBB, 0x7B)),
    ] {
        fill(batch, atlas, x + 1, y + row, 1, 1, inner);
        fill(batch, atlas, x + 2, y + row, w - 4, 1, middle);
        fill(batch, atlas, x + w - 2, y + row, 1, 1, inner);
    }
    fill(batch, atlas, x + 1, y + 2, 1, 16, a(0xBC, 0x7C));
    fill(batch, atlas, x + 2, y + 2, w - 4, 16, a(0xBA, 0x7A));
    fill(batch, atlas, x + w - 2, y + 2, 1, 16, a(0xBC, 0x7C));
}

/// A 1-px outline around the selected tab, in a soft amber. The tab bar has
/// no other marker for the active page (the unselected tabs are just dimmer),
/// so the outline makes the current tab unmistakable without shouting.
fn tab_outline(batch: &mut HudBatch, atlas: &HudAtlas, x: i32, y: i32, w: i32, h: i32, col: u32) {
    fill(batch, atlas, x + 2, y, w - 4, 1, col);
    fill(batch, atlas, x + 1, y + 1, 1, 1, col);
    fill(batch, atlas, x + w - 2, y + 1, 1, 1, col);
    fill(batch, atlas, x, y + 2, 1, h - 4, col);
    fill(batch, atlas, x + w - 1, y + 2, 1, h - 4, col);
    fill(batch, atlas, x + 1, y + h - 2, 1, 1, col);
    fill(batch, atlas, x + w - 2, y + h - 2, 1, 1, col);
    fill(batch, atlas, x + 2, y + h - 1, w - 4, 1, col);
}

/// `widget/tab/focus.png`: a rounded 1-px `0xFFFFFFA0` frame, `h` high.
fn focus_frame(batch: &mut HudBatch, atlas: &HudAtlas, x: i32, y: i32, w: i32, h: i32) {
    let col = 0xFFFF_FFA0;
    fill(batch, atlas, x + 2, y, w - 4, 1, col);
    fill(batch, atlas, x + 1, y + 1, 1, 1, col);
    fill(batch, atlas, x + w - 2, y + 1, 1, 1, col);
    fill(batch, atlas, x, y + 2, 1, h - 4, col);
    fill(batch, atlas, x + w - 1, y + 2, 1, h - 4, col);
    fill(batch, atlas, x + 1, y + h - 2, 1, 1, col);
    fill(batch, atlas, x + w - 2, y + h - 2, 1, 1, col);
    fill(batch, atlas, x + 2, y + h - 1, w - 4, 1, col);
}

// ---- the screen's parts ----------------------------------------------------------

/// The root panel's fixed size (the mod's, slightly larger).
const ROOT: (i32, i32) = (440, 352);
/// Root-relative rectangles.
const RIBBON: (i32, i32) = (10, 29);
const CARD: [i32; 4] = [8, 50, 424, 272];
const DONE: [i32; 4] = [12, 326, 48, 16];
const RESET: [i32; 4] = [380, 326, 48, 16];
/// The tab bar height and the tab text geometry.
const TAB_HEIGHT: i32 = 20;
const TAB_WIDTH: i32 = 28;
/// The row step (`setGap(-1)`): 22-high cells, one pixel apart.
const ROW_STEP: i32 = 21;
/// A row's height.
const ROW_HEIGHT: i32 = 22;
/// The checkbox side (the sprite's native size, never stretched).
const CHECKBOX: i32 = 18;
/// The checkbox's vertical offset inside a 22px row.
const CHECKBOX_OFFSET: i32 = (ROW_HEIGHT - CHECKBOX) / 2;
/// A toggle/slider widget's width (10 cells of 18-px grid).
const WIDGET_WIDTH: i32 = 180;
/// A widget's height (one grid cell).
const WIDGET_HEIGHT: i32 = 18;
/// The enum button's width (18 cells with -13 gap).
const CHOICE_WIDTH: i32 = 103;
/// The slider thumb's width.
const THUMB_WIDTH: i32 = 8;

/// The Settings tab's rows come from `runtime::row` (order).

/// Constants for item tabs: the tab bar height is the same as Settings, but
/// the content area is different. Item lists have a search field at the bottom
/// (20px high) and the rest is for the list.
const ITEM_SEARCH_HEIGHT: i32 = 20;

/// Item list rendering: each item row is 22px high with -1 gap.
const ITEM_ROW_STEP: i32 = ROW_STEP;
const ITEM_ROW_HEIGHT: i32 = ROW_HEIGHT;
const ITEM_INSETS: (i32, i32) = (4, 4);

/// Tab identifiers: 0 = Settings, 1 = Hold Up Items, 2 = Lanterns.
const TAB_COUNT: usize = 3;
const TAB_ICONS: [&str; TAB_COUNT] = ["minecraft:filled_map", "minecraft:torch", "minecraft:lantern"];

/// Item tab indices for settings access: holding_items (1), lantern_items (2).
const ITEM_TAB_HOLDING: usize = 1;
const ITEM_TAB_LANTERN: usize = 2;

/// A point under the pointer: the widget under it. `Hit::Item` for item lists.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Hit {
    Tab(usize),
    ScrollBar,
    Row(usize),
    Slider(usize),
    Choice(usize),
    Done,
    Reset,
    Panel,
    Item(usize), // The index of the item in the visible list for the current tab.
}

/// The keyboard focus: which widget Tab cycles through.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Widget {
    Tab,
    ScrollBar,
    Row(usize),
    Slider(usize),
    Choice(usize),
    Done,
    Reset,
    Item(usize), // Focus on a specific item in the item list.
    ItemSearch,  // Focus on the search field of the current item tab.
}

/// A scrollbar being dragged: the press's y and the value then.
#[derive(Clone, Copy)]
struct Slide {
    anchor: i32,
    value: i32,
}

/// The GUI for the settings screen. Extended to support item lists with search.
pub(super) struct Gui {
    pub(super) actions: Vec<Action>,
    pub(super) closed: bool,
    // Settings tab state.
    scrollbar_value: i32,
    // Item tabs state.
    item_tab: usize, // Current visible tab (0, 1, or 2).
    item_scroll: [i32; TAB_COUNT], // Per-tab scroll positions.
    // Search fields per item tab.
    item_search: [String; TAB_COUNT],
    /// The current item tab's rows as last drawn (item IDs), so hit/scroll
    /// agree with draw without needing `game`.
    item_visible: Vec<String>,
    // Focus and interaction state.
    focus: Option<Widget>,
    pressed: Option<Hit>,
    slide: Option<Slide>,
    /// The slider setting being dragged, if any.
    slider_drag: Option<usize>,
    /// The root panel's corner, as last drawn.
    origin: (i32, i32),
    /// The panel's drawn height (the window's, when shorter than the design).
    panel_h: i32,
    /// Each tab's (left, width) inside the root, filled in `draw`.
    tab_rects: [(i32, i32); TAB_COUNT],
    /// Each enum row's button rectangle (root-relative), filled in `draw`.
    choice_rects: [[i32; 4]; m::CHOICE_COUNT],
    opened: Instant,
    /// Whether the search field is focused for an item tab.
    item_search_focused: bool,
    /// The hovered Settings row's tooltip text, as last drawn.
    tooltip: Option<String>,
    /// Which row has been hovered and since when (the tooltip's delay).
    tooltip_hover: Option<(usize, Instant)>,
}

impl Gui {
    pub(super) fn new() -> Self {
        Self {
            actions: Vec::new(),
            closed: false,
            scrollbar_value: 0,
            item_tab: 0,
            item_scroll: [0; TAB_COUNT],
            item_search: [const { String::new() }; TAB_COUNT],
            item_visible: Vec::new(),
            focus: None,
            pressed: None,
            slide: None,
            slider_drag: None,
            origin: (0, 0),
            panel_h: ROOT.1,
            tab_rects: [(0, TAB_WIDTH); TAB_COUNT],
            choice_rects: [[0; 4]; m::CHOICE_COUNT],
            opened: Instant::now(),
            item_search_focused: false,
            tooltip: None,
            tooltip_hover: None,
        }
    }

    /// The buttons' row's y (the panel's bottom, minus an 8px margin).
    fn button_y(&self) -> i32 {
        self.panel_h - 26
    }

    // ---- layout -------------------------------------------------------------

    /// The visible rows: (the row index, its kind, its card-relative y). A row
    /// is drawn while any part of it is inside the card; a completely
    /// scrolled-out row is left out.
    fn rows(&self, ch: i32) -> Vec<(usize, m::Row, i32)> {
        let mut out = Vec::new();
        let bottom = CARD[1] + ch;
        let scroll = self.scrollbar_value.clamp(0, self.max_scroll(ch));
        for row in 0..m::ROW_COUNT {
            let ry = CARD[1] + 2 + ROW_STEP * row as i32;
            let ly = ry - scroll * ROW_STEP;
            if ly < bottom && ly + ROW_HEIGHT > CARD[1] {
                out.push((row, m::row(row), ly));
            }
        }
        out
    }

    /// The card-relative y of the visible row whose kind matches, if shown.
    fn row_y(&self, want: m::Row) -> Option<i32> {
        let ch = self.card_h();
        self.rows(ch)
            .into_iter()
            .find(|(_, kind, _)| *kind == want)
            .map(|(_, _, ly)| ly)
    }

    /// The card's drawn height (the window's, when shorter than the design).
    fn card_h(&self) -> i32 {
        (CARD[3] - (ROOT.1 - self.panel_h)).max(0)
    }

    /// `WListPanel.layout`'s visible cells: the list's height, its 4-px insets
    /// and the -1 gap around 22-px cells.
    fn window(&self, ch: i32) -> i32 {
        ((ch - 9) / ROW_STEP).clamp(1, m::ROW_COUNT as i32)
    }

    /// Vanilla-style scroll check: the list scrolls only when its content is
    /// taller than the visible area (`WScrollBar.checkValue`).
    fn needs_scroll(&self, ch: i32) -> bool {
        self.window(ch) < m::ROW_COUNT as i32
    }

    /// `maxValue - window`: the last row's index at the bottom.
    fn max_scroll(&self, ch: i32) -> i32 {
        (m::ROW_COUNT as i32 - self.window(ch)).max(0)
    }

    /// The widget under the card-relative point.
    fn hit(&self, p: (i32, i32)) -> Hit {
        let inside = |r: [i32; 4]| p.0 >= r[0] && p.0 < r[0] + r[2] && p.1 >= r[1] && p.1 < r[1] + r[3];
        let ch = self.card_h();
        if inside([CARD[0], CARD[1], CARD[2], ch]) {
            if self.item_tab == 0 {
                for (_, kind, ly) in self.rows(ch).into_iter().rev() {
                    let top = ly.max(CARD[1]);
                    let bottom = (ly + ROW_HEIGHT).min(CARD[1] + ch);
                    if p.1 < top || p.1 >= bottom {
                        continue;
                    }
                    let wx = CARD[0] + 4;
                    let wy = ly + 2;
                    let row_right = CARD[0] + CARD[2] - 8;
                    match kind {
                        m::Row::Toggle(i) => {
                            // The whole row is the click target (the check box
                            // column is narrow; clicks on the label count too).
                            if p.0 >= CARD[0] && p.0 < row_right {
                                return Hit::Row(i);
                            }
                        }
                        m::Row::Slider(i) => {
                            if p.0 >= CARD[0]
                                && p.0 < row_right
                                && p.1 >= wy
                                && p.1 < wy + WIDGET_HEIGHT
                            {
                                return Hit::Slider(i);
                            }
                        }
                        m::Row::Choice(i) => {
                            let r = self.choice_rects[i];
                            if within(p, r) {
                                return Hit::Choice(i);
                            }
                        }
                        _ => {}
                    }
                }
            } else {
                // Item tabs: the rows sit above the search field.
                let list_h = ch - ITEM_SEARCH_HEIGHT;
                let scroll = self.item_scroll[self.item_tab].max(0);
                for (vi, _) in self.item_visible.iter().skip(scroll as usize).enumerate() {
                    let ly = CARD[1] + 2 + ROW_STEP * vi as i32;
                    if ly >= CARD[1] + list_h {
                        break;
                    }
                    let top = ly.max(CARD[1]);
                    let bottom = (ly + ROW_HEIGHT).min(CARD[1] + list_h);
                    if p.0 >= CARD[0] + 4
                        && p.0 < CARD[0] + CARD[2] - 12
                        && p.1 >= top
                        && p.1 < bottom
                    {
                        return Hit::Item((scroll + vi as i32) as usize);
                    }
                }
            }
        }
        // The item tabs' scrollbar covers the list area (above the search field).
        let scrollable = if self.item_tab == 0 {
            self.needs_scroll(ch)
        } else {
            let list_h = ch - ITEM_SEARCH_HEIGHT;
            let window = ((list_h + ROW_STEP) / ROW_STEP).max(1);
            (self.item_visible.len() as i32 - window) > 0
        };
        let scroll_h = if self.item_tab == 0 { ch } else { ch - ITEM_SEARCH_HEIGHT };
        if scrollable && inside([CARD[0] + CARD[2] - 8, CARD[1], 8, scroll_h]) {
            Hit::ScrollBar
        } else if inside([DONE[0], self.button_y(), DONE[2], DONE[3]]) {
            Hit::Done
        } else if inside([RESET[0], self.button_y(), RESET[2], RESET[3]]) {
            Hit::Reset
        } else {
            // The tab ribbon: one rectangle per tab (filled in `draw`).
            for (i, (x, tw)) in self.tab_rects.iter().enumerate() {
                if p.0 >= *x && p.0 < x + tw && p.1 >= RIBBON.1 && p.1 < RIBBON.1 + TAB_HEIGHT {
                    return Hit::Tab(i);
                }
            }
            Hit::Panel
        }
    }

    /// Whether the point is inside the widget that holds the focus.
    fn focus_contains(&self, f: Widget, p: (i32, i32)) -> bool {
        let ch = self.card_h();
        let rect = match f {
            Widget::Tab => {
                let (x, w) = self.tab_rects[self.item_tab];
                [x, RIBBON.1, w, TAB_HEIGHT]
            }
            Widget::ScrollBar => [CARD[0] + CARD[2] - 8, CARD[1], 8, ch],
            Widget::Done => [DONE[0], self.button_y(), DONE[2], DONE[3]],
            Widget::Reset => [RESET[0], self.button_y(), RESET[2], RESET[3]],
            Widget::Row(i) => {
                let Some(ly) = self.row_y(m::Row::Toggle(i)) else {
                    return false;
                };
                [CARD[0] + 4, ly + 2, WIDGET_WIDTH, WIDGET_HEIGHT]
            }
            Widget::Slider(i) => {
                let Some(ly) = self.row_y(m::Row::Slider(i)) else {
                    return false;
                };
                [CARD[0] + 4, ly + 2, WIDGET_WIDTH, WIDGET_HEIGHT]
            }
            Widget::Choice(i) => self.choice_rects[i],
            Widget::ItemSearch => {
                let [cx, cy, cw, _] = CARD;
                [cx, cy + (ch - ITEM_SEARCH_HEIGHT), cw, ITEM_SEARCH_HEIGHT]
            }
            Widget::Item(i) => {
                let ly = CARD[1] + 2 + ROW_STEP * (i as i32 - self.item_scroll[self.item_tab]);
                [CARD[0] + 4, ly, CARD[2] - 16, ROW_HEIGHT]
            }
        };
        within(p, rect)
    }

    /// The screen point as root-relative whole pixels.
    fn local(&self, p: (f32, f32)) -> (i32, i32) {
        (p.0.floor() as i32 - self.origin.0, p.1.floor() as i32 - self.origin.1)
    }

    // ---- drawing ------------------------------------------------------------

    pub(super) fn draw(&mut self, game: &Game<'_>, batch: &mut HudBatch) {
        let atlas = &game.gui.atlas;
        let (w, h) = (batch.width.ceil() as i32, batch.height.ceil() as i32);
        // The panel never exceeds the window: a short window shrinks the panel
        // vertically, so the buttons stay visible (the rows just show fewer).
        let rw = ROOT.0.min(w);
        let rh = ROOT.1.min(h);
        self.origin = (((w - rw) / 2).max(0), ((h - rh) / 2).max(0));
        self.panel_h = rh;
        let (ox, oy) = self.origin;
        let dump = !DUMPED.with(|d| d.replace(true));
        if dump {
            let ch = self.card_h();
            dbg(&format!(
                "geom origin=({ox},{oy}) rw={rw} rh={rh} panel_h={} ch={ch} button_y={}",
                self.panel_h,
                self.button_y()
            ));
            for (i, (_, _, ly)) in self.rows(ch).into_iter().take(10).enumerate() {
                dbg(&format!("row {i} ly={ly}"));
            }
            for (i, (x, tw)) in self.tab_rects.iter().enumerate() {
                dbg(&format!("tab {i} x={x} tw={tw}"));
            }
        }
        let mouse = self.local(game.cursor);
        let lang = game.settings.language.as_str();
        game.apply_screen_blur();
        ui::menu_background(batch, atlas, 0.0, 0.0, w as f32, h as f32, false);
        panel(batch, atlas, ox, oy, rw, rh);

        // The tab ribbon: one rectangle per tab, icon then title, the active
        // one open at the bottom (the tab panel).
        for i in 0..TAB_COUNT {
            let label = Text::literal(m::tab_label_by_tab(i, lang));
            let tw = TAB_WIDTH + width(atlas, &label);
            let x = if i == 0 { RIBBON.0 } else { self.tab_rects[i - 1].0 + self.tab_rects[i - 1].1 + 1 };
            self.tab_rects[i] = (x, tw);
            let selected = self.item_tab == i;
            tab_rect(batch, atlas, ox + x, oy + RIBBON.1, tw, selected);
            // Every tab gets the same clean outline; the selected one is the
            // soft amber, the rest a neutral grey that covers the vanilla
            // black top edge (which otherwise shows as a broken border).
            tab_outline(batch, atlas, ox + x, oy + RIBBON.1, tw, 18, if selected {
                0xFFD9_A441
            } else {
                0xFF6B_6B6B
            });
            let icon_item = TAB_ICONS[i];
            // The map and torch sprites sit lower inside their 16px tile than
            // the lantern's, so they are raised to the tab's middle.
            let icon_y = oy + RIBBON.1 + if i <= 1 { 1 } else { 2 };
            game.draw_item(batch, &stack(icon_item), (ox + x + 6) as f32, icon_y as f32);
            text(
                batch,
                atlas,
                &label,
                ox + x + 24,
                oy + RIBBON.1 + 8,
                0xFFEE_EEEE,
                true,
            );
        }
        if self.focus == Some(Widget::Tab) {
            let last = self.tab_rects[TAB_COUNT - 1];
            focus_frame(
                batch,
                atlas,
                ox + self.tab_rects[0].0,
                oy + RIBBON.1,
                last.0 + last.1 - self.tab_rects[0].0,
                TAB_HEIGHT + 2,
            );
        }

        // The card.
        let [cx, cy, cw, _] = CARD;
        let ch = self.card_h();
        panel(batch, atlas, ox + cx, oy + cy, cw, ch);
        self.tooltip = None;
        self.draw_list(game, batch, mouse, ch);

        // Done and Reset (vanilla buttons, as custom ones).
        let by = self.button_y();
        for (hit, r, key) in [(Hit::Done, DONE, "gui.done"), (Hit::Reset, RESET, "controls.reset")] {
            let focused = self.focus == Some(if hit == Hit::Done { Widget::Done } else { Widget::Reset });
            let rect = [r[0], by, r[2], r[3]];
            let lit = within(mouse, rect) || focused;
            ui::button(
                batch,
                atlas,
                (ox + r[0]) as f32,
                (oy + by) as f32,
                r[2] as f32,
                r[3] as f32,
                &Text::translate(key, &[]),
                true,
                lit,
            );
        }

        // The title, over the panel.
        text(batch, atlas, &Text::literal(m::title(lang)), ox + 8, oy + 6, 0xFFBC_BCBC, true);

        // The hovered row's tooltip, last so nothing draws over it. It shows
        // only after the pointer rests on the row for a moment.
        if let (Some(tip), Some((_, since))) = (&self.tooltip, &self.tooltip_hover) {
            if since.elapsed() >= TOOLTIP_DELAY {
                draw_tooltip(batch, atlas, tip, game.cursor, w, h);
            }
        }
        if self.tooltip.is_none() {
            self.tooltip_hover = None;
        }
    }

    /// Settings tab's scrollbar (the mod 12-row list).
    fn draw_scroll_bar(&self, batch: &mut HudBatch, atlas: &HudAtlas, mouse: (i32, i32), ch: i32) {
        if !self.needs_scroll(ch) {
            return;
        }
        self.draw_scroll_bar_for(batch, atlas, mouse, ch, m::ROW_COUNT as i32, self.scrollbar_value, self.max_scroll(ch));
    }

    /// A scrollbar for any list: `count` rows, `value` scrolled of `max`.
    fn draw_scroll_bar_for(
        &self,
        batch: &mut HudBatch,
        atlas: &HudAtlas,
        mouse: (i32, i32),
        h: i32,
        count: i32,
        value: i32,
        max_scroll: i32,
    ) {
        if count <= 0 {
            return;
        }
        let [x, y, w, _] = [CARD[0] + CARD[2] - 8, CARD[1], 8, h];
        let (ox, oy) = self.origin;
        bevel(batch, atlas, ox + x, oy + y, w, h, sprites::SCROLL_BACKGROUND);
        let window = (h + ROW_STEP - 1) / ROW_STEP;
        let bar = h - 2;
        let ratio = if window >= count { 1.0 } else { window as f32 / count as f32 };
        let size = ((ratio * bar as f32) as i32).max(6);
        let movable = bar - size;
        let pos = (value as f32 / max_scroll.max(1) as f32 * movable as f32) as i32;
        let colors = if self.slide.is_some() {
            sprites::SCROLL_THUMB_PRESSED
        } else if within(mouse, [x, y, w, h]) {
            sprites::SCROLL_THUMB_HOVERED
        } else {
            sprites::SCROLL_THUMB
        };
        bevel(batch, atlas, ox + x + 1, oy + y + 1 + pos, 6, size, colors);
    }

    fn draw_list(&mut self, game: &Game<'_>, batch: &mut HudBatch, mouse: (i32, i32), ch: i32) {
        let atlas = &game.gui.atlas;
        let (ox, oy) = self.origin;
        let [cx, cy, cw, _] = CARD;
        // Settings tab: the mod 12-row toggle list.
        if self.item_tab == 0 {
            self.scrollbar_value = self.scrollbar_value.clamp(0, self.max_scroll(ch));
            self.draw_scroll_bar(batch, atlas, mouse, ch);
            self.draw_settings_rows(game, batch, atlas, mouse, ch);
        } else {
            // Item tabs: every registry item, sorted by id, with the search
            // field at the bottom.
            let items = self.tab_items(game);
            let mut active = self.item_search[self.item_tab].clone();
            // Visible rows: the matching items, scrolled within the visible
            // area above the search field.
            let list_h = ch - ITEM_SEARCH_HEIGHT;
            let needle = active.to_lowercase();
            let visible: Vec<String> = items
                .iter()
                .filter(|it| {
                    needle.is_empty()
                        || it.to_lowercase().contains(&needle)
                        || game.item_display_name(it).resolved().plain().to_lowercase().contains(&needle)
                })
                .cloned()
                .collect();
            let count = visible.len() as i32;
            let window = ((list_h + ROW_STEP) / ROW_STEP).max(1);
            let max_scroll = (count - window).max(0);
            let scroll = self.item_scroll[self.item_tab].clamp(0, max_scroll);
            self.item_visible = visible.clone();
            self.draw_scroll_bar_for(batch, atlas, mouse, list_h, count, scroll, max_scroll);
            self.draw_item_rows(game, batch, atlas, mouse, cy, list_h, &visible, scroll);
            // Search field.
            self.draw_search_field(game, batch, atlas, cx, cy + list_h, cw, ITEM_SEARCH_HEIGHT, &mut active);
            self.item_search[self.item_tab] = active;
        }
    }

    fn draw_settings_rows(
        &mut self,
        game: &Game<'_>,
        batch: &mut HudBatch,
        atlas: &HudAtlas,
        mouse: (i32, i32),
        ch: i32,
    ) {
        let (ox, oy) = self.origin;
        let [cx, cy, cw, _] = CARD;
        let s = glue::settings();
        let lang = game.settings.language.as_str();
        let start = batch.vertices.len();
        for (row, kind, ly) in self.rows(ch) {
            if ly >= cy + ch {
                continue;
            }
            let x = cx + 4;
            let wy = ly + 2;
            let clipped = !(ly + 17 > cy && ly + 8 < cy + ch);
            match kind {
                m::Row::Toggle(i) => {
                    let on = s.get(i);
                    let by0 = ly + CHECKBOX_OFFSET;
                    checkbox(batch, atlas, &sprites::CHECKBOX, ox + x, oy + by0);
                    if on {
                        checkbox(batch, atlas, &sprites::CHECKBOX_SELECTED, ox + x, oy + by0);
                    }
                    if self.focus == Some(Widget::Row(i)) {
                        checkbox(batch, atlas, &sprites::CHECKBOX_HIGHLIGHTED, ox + x, oy + by0);
                    }
                    if !clipped {
                        let name = Text::literal(m::name(i, lang));
                        let nw = width(atlas, &name);
                        text(batch, atlas, &name, ox + x + 26, oy + ly + 8, 0xFFBC_BCBC, false);
                        // The tooltip follows the name only, not the whole row.
                        if within(mouse, [x + 26, ly + 6, nw + 2, 15]) {
                            self.tooltip(row, m::toggle_tooltip(i, lang));
                        }
                    }
                }
                m::Row::Slider(i) => {
                    let (min, max, _) = m::slider_range(i);
                    let v = s.slider(i);
                    let value = if max > min {
                        ((v - min) / (max - min)).clamp(0.0, 1.0)
                    } else {
                        0.0
                    };
                    let dragging = self.slider_drag == Some(i);
                    let hovered = within(mouse, [x, wy, WIDGET_WIDTH, WIDGET_HEIGHT]);
                    let focused = self.focus == Some(Widget::Slider(i));
                    let track = if dragging || focused {
                        "gui/sprites/widget/slider_highlighted"
                    } else {
                        "gui/sprites/widget/slider"
                    };
                    let handle = if dragging || hovered || focused {
                        "gui/sprites/widget/slider_handle_highlighted"
                    } else {
                        "gui/sprites/widget/slider_handle"
                    };
                    ui::nine_slice(
                        batch,
                        atlas,
                        track,
                        (ox + x) as f32,
                        (oy + wy) as f32,
                        WIDGET_WIDTH as f32,
                        WIDGET_HEIGHT as f32,
                        u32::MAX,
                    );
                    ui::nine_slice(
                        batch,
                        atlas,
                        handle,
                        (ox + x) as f32 + (value * (WIDGET_WIDTH - THUMB_WIDTH) as f32).floor(),
                        (oy + wy) as f32,
                        THUMB_WIDTH as f32,
                        WIDGET_HEIGHT as f32,
                        u32::MAX,
                    );
                    if !clipped {
                        let label = Text::literal(m::slider_text(i, v, lang));
                        let nw = width(atlas, &Text::literal(m::slider_name(i, lang)));
                        let tx = x + (WIDGET_WIDTH - width(atlas, &label)) / 2;
                        text(batch, atlas, &label, ox + tx, oy + wy + 5, 0xFFFF_FFFF, false);
                        // The name is the label's first part: the tooltip
                        // follows it only.
                        if within(mouse, [tx, wy, nw + 2, WIDGET_HEIGHT]) {
                            self.tooltip(row, m::slider_tooltip(i, lang));
                        }
                    }
                }
                m::Row::Choice(i) => {
                    let name = Text::literal(m::choice_name(i, lang));
                    let bw = width(atlas, &name);
                    let bx = x + (bw as f32 / 4.7).floor() as i32 * 5;
                    self.choice_rects[i] = [bx, wy, CHOICE_WIDTH, WIDGET_HEIGHT];
                    if !clipped {
                        text(batch, atlas, &name, ox + x, oy + ly + 5, 0xFFBC_BCBC, false);
                        // The tooltip follows the name only, not the button.
                        if within(mouse, [x, ly + 3, bw + 2, 14]) {
                            self.tooltip(row, m::choice_tooltip(i, lang));
                        }
                    }
                    let value = Text::literal(m::choice_value(i, s.choice(i), lang));
                    let hovered = within(mouse, self.choice_rects[i]);
                    let focused = self.focus == Some(Widget::Choice(i));
                    ui::button(
                        batch,
                        atlas,
                        (ox + bx) as f32,
                        (oy + wy) as f32,
                        CHOICE_WIDTH as f32,
                        WIDGET_HEIGHT as f32,
                        &value,
                        true,
                        hovered || focused,
                    );
                }
                m::Row::Disabled(i) => {
                    if !clipped {
                        text(
                            batch,
                            atlas,
                            &Text::literal(m::disabled(i, lang)),
                            ox + x + 3,
                            oy + ly + 8,
                            0xFF80_8080,
                            false,
                        );
                    }
                }
                m::Row::Heading(i) => {
                    if !clipped {
                        text(
                            batch,
                            atlas,
                            &Text::literal(m::heading(i, lang)),
                            ox + x + 3,
                            oy + ly + 8,
                            0xFFBC_BCBC,
                            false,
                        );
                    }
                }
                m::Row::Blank => {}
            }
        }
        let cnt = DRAW_CNT.with(|c| c.replace(c.get() + 1));
        if cnt % 120 == 0 {
            let mut parts: Vec<String> = Vec::new();
            for (_, kind, ly) in self.rows(ch) {
                if let m::Row::Toggle(i) = kind {
                    parts.push(format!("{i}:{ly}:{}", s.get(i)));
                }
            }
            dbg(&format!("vis tg [{}]", parts.join(" ")));
        }
        batch.clip_since(
            start,
            (ox + cx) as f32,
            (oy + cy) as f32,
            (ox + cx + cw) as f32,
            (oy + cy + ch) as f32,
        );
    }

    fn draw_item_rows(
        &mut self,
        game: &Game<'_>,
        batch: &mut HudBatch,
        atlas: &HudAtlas,
        mouse: (i32, i32),
        cy: i32,
        ch: i32,
        visible: &[String],
        scroll: i32,
    ) {
        let (ox, oy) = self.origin;
        let [cx, _ccy, cw, _] = CARD;
        let s = glue::settings();
        let _ = mouse;
        let start = batch.vertices.len();
        // `scroll` skips the first rows; the window's rows start at the top.
        for (vi, id) in visible.iter().skip(scroll.max(0) as usize).enumerate() {
            let ly = cy + 2 + ROW_STEP * vi as i32;
            if ly >= cy + ch {
                break;
            }
            let x = cx + 4;
            let by0 = ly + CHECKBOX_OFFSET;
            // Item icon, then the checkbox, then the item's translated name.
            let on = glue::item_on(&s, self.item_tab, id);
            game.draw_item(batch, &stack(id), (ox + x) as f32 + 2.0, (oy + by0) as f32);
            checkbox(batch, atlas, &sprites::CHECKBOX, ox + x, oy + by0);
            if on {
                checkbox(batch, atlas, &sprites::CHECKBOX_SELECTED, ox + x, oy + by0);
            }
            if self.focus == Some(Widget::Item((scroll.max(0) as usize) + vi)) {
                checkbox(batch, atlas, &sprites::CHECKBOX_HIGHLIGHTED, ox + x, oy + by0);
            }
            if ly + 17 > cy && ly + 8 < cy + ch {
                let name = game.item_display_name(id).resolved().plain();
                text(batch, atlas, &Text::literal(name), ox + x + 26, oy + ly + 8, 0xFFBC_BCBC, false);
            }
        }
        batch.clip_since(
            start,
            (ox + cx) as f32,
            (oy + cy) as f32,
            (ox + cx + cw) as f32,
            (oy + cy + ch) as f32,
        );
    }

    /// Every item the game's registry knows, sorted by id (the mod's
    /// `createItemTab`), for either item tab.
    fn tab_items(&self, game: &Game<'_>) -> Vec<String> {
        let mut items = game.ui.registries.items.clone();
        if items.is_empty() {
            // Outside a world the registry is not built yet (`build_registries`
            // runs on world entry). The resource root's item defaults hold the
            // same ids, and are cached by `item_defaults::shared`.
            if let Ok(defaults) = horizon_world::item_defaults::shared(&game.options.resources) {
                items.extend(defaults.entries.keys().cloned());
            }
        }
        items.sort();
        items.dedup();
        items
    }

    fn draw_search_field(
        &mut self,
        game: &Game<'_>,
        batch: &mut HudBatch,
        atlas: &HudAtlas,
        cx: i32,
        cy: i32,
        cw: i32,
        ch: i32,
        active: &mut String,
    ) {
        let (ox, oy) = self.origin;
        let focused = self.item_search_focused;
        // Search panel background (vanilla edit box).
        fill(batch, atlas, ox + cx, oy + cy, cw, ch, 0xFF55_5555);
        fill(batch, atlas, ox + cx + 1, oy + cy + 1, cw - 2, ch - 2, 0xFF00_0000);
        if active.is_empty() {
            // The placeholder, only when there is nothing typed.
            let hint = if m::russian(&glue::language()) { "Поиск..." } else { "Search..." };
            text(batch, atlas, &Text::literal(hint), ox + cx + 6, oy + cy + 6, 0xFFAAAAAA, false);
        } else {
            text(batch, atlas, &Text::literal(active.clone()), ox + cx + 6, oy + cy + 6, 0xFFEEEEEE, false);
            if focused {
                text(
                    batch,
                    atlas,
                    &Text::literal("|"),
                    ox + cx + 6 + width(atlas, &Text::literal(active.clone())),
                    oy + cy + 6,
                    0xFFFFFFFF,
                    false,
                );
            }
        }
    }

    // ---- input --------------------------------------------------------------

    fn click(&mut self) {
        self.actions.push(Action::Click);
    }

    pub(super) fn press(&mut self, game: &Game<'_>, p: (f32, f32), button: i32) {
        let _ = button;
        let p = self.local(p);
        let hit = self.hit(p);
        dbg(&format!("gui.press p=({},{}) hit={:?}", p.0, p.1, hit));
        // `MouseInputHandler.checkFocus`: a click outside the focused widget
        // releases the focus. Only the scroll bar takes it on mouse down.
        if let Some(f) = self.focus
            && !self.focus_contains(f, p)
        {
            self.focus = None;
        }
        self.pressed = Some(hit);
        match hit {
            Hit::ScrollBar => {
                self.focus = Some(Widget::ScrollBar);
                let value = if self.item_tab == 0 {
                    self.scrollbar_value
                } else {
                    self.item_scroll[self.item_tab]
                };
                self.slide = Some(Slide { anchor: p.1, value });
            }
            Hit::Slider(i) => {
                self.focus = Some(Widget::Slider(i));
                self.slider_drag = Some(i);
                self.slider_to(game, i, p.0);
            }
            Hit::Choice(i) => self.focus = Some(Widget::Choice(i)),
            Hit::Row(i) => self.focus = Some(Widget::Row(i)),
            Hit::Done => self.focus = Some(Widget::Done),
            Hit::Reset => self.focus = Some(Widget::Reset),
            _ => {}
        }
        // The search field takes the focus on mouse down.
        if self.item_tab != 0 {
            let list_h = self.card_h() - ITEM_SEARCH_HEIGHT;
            let field = [CARD[0], CARD[1] + list_h, CARD[2], ITEM_SEARCH_HEIGHT];
            if within(p, field) {
                self.focus = Some(Widget::ItemSearch);
                self.item_search_focused = true;
            } else if self.focus == Some(Widget::ItemSearch) {
                self.item_search_focused = false;
            }
        }
    }

    /// `WAbstractDoubleSlider.setValueFromMouse`: the pointer's x sets the
    /// slider's value along its 8-px-inset track.
    fn slider_to(&self, game: &Game<'_>, i: usize, x: i32) {
        let (min, max, _) = m::slider_range(i);
        let t = ui::slider_value(
            (CARD[0] + 4) as f32,
            WIDGET_WIDTH as f32,
            x as f32,
        );
        glue::set_slider_local(i, min + t * (max - min));
    }

    pub(super) fn drag(&mut self, game: &Game<'_>, p: (f32, f32)) {
        let p = self.local(p);
        if let Some(i) = self.slider_drag {
            self.slider_to(game, i, p.0);
            return;
        }
        if let Some(hit) = self.pressed
            && hit == Hit::ScrollBar
            && self.focus == Some(Widget::ScrollBar)
            && let Some(slide) = self.slide
        {
            if self.item_tab == 0 {
                let ch = self.card_h();
                let count = m::ROW_COUNT as i32;
                let window = self.window(ch);
                let bar = ch - 2;
                let ratio = if window >= count { 1.0 } else { window as f32 / count as f32 };
                let movable = bar - ((ratio * bar as f32) as i32).max(6);
                let delta = ((p.1 - slide.anchor) as f32 / movable as f32 * (count - window) as f32) as i32;
                self.scrollbar_value = (slide.value + delta).clamp(0, self.max_scroll(ch));
            } else {
                let list_h = self.card_h() - ITEM_SEARCH_HEIGHT;
                let count = self.item_visible.len() as i32;
                let window = ((list_h + ROW_STEP) / ROW_STEP).max(1);
                let bar = list_h - 2;
                let ratio = if window >= count { 1.0 } else { window as f32 / count as f32 };
                let movable = bar - ((ratio * bar as f32) as i32).max(6);
                let delta = ((p.1 - slide.anchor) as f32 / movable as f32 * (count - window) as f32) as i32;
                self.item_scroll[self.item_tab] =
                    (slide.value + delta).clamp(0, (count - window).max(0));
            }
        }
    }

    pub(super) fn release(&mut self, game: &Game<'_>, p: (f32, f32), button: i32) {
        let _ = button;
        let p = self.local(p);
        let hit = self.pressed.take();
        dbg(&format!("gui.release p=({},{}) hit={:?}", p.0, p.1, hit));
        self.slide = None;
        self.slider_drag = None;
        match hit {
            Some(Hit::Row(i)) => {
                // `WToggleButton.onClick`: the toggle flips on release.
                self.click();
                glue::toggle_local(i);
            }
            Some(Hit::Choice(i)) => {
                if within(p, self.choice_rects[i]) {
                    self.click();
                    glue::cycle_choice_local(i);
                }
            }
            Some(Hit::Done) => {
                if within(p, [DONE[0], self.button_y(), DONE[2], DONE[3]]) {
                    self.click();
                    self.done(game);
                }
            }
            Some(Hit::Reset) => {
                if within(p, [RESET[0], self.button_y(), RESET[2], RESET[3]]) {
                    self.click();
                    self.reset(game);
                }
            }
            Some(Hit::Tab(i)) => {
                self.click();
                self.item_tab = i;
            }
            Some(Hit::Item(i)) => {
                self.click();
                if let Some(id) = self.item_visible.get(i).cloned() {
                    glue::toggle_item_local(self.item_tab, &id);
                }
            }
            _ => {}
        }
    }

    fn done(&mut self, game: &Game<'_>) {
        glue::mark_dirty();
        self.closed = true;
    }

    fn reset(&mut self, game: &Game<'_>) {
        glue::reset_local();
    }

    pub(super) fn scroll(&mut self, game: &Game<'_>, p: (f32, f32), scroll: [f64; 2]) {
        let _ = game;
        let ch = self.card_h();
        if self.item_tab == 0 {
            if !self.needs_scroll(ch) {
                return;
            }
            let p = self.local(p);
            if !within(p, [CARD[0], CARD[1], CARD[2], ch]) {
                return;
            }
            let delta = ((scroll[0] - scroll[1]) as i32) * 4;
            self.scrollbar_value = (self.scrollbar_value + delta).clamp(0, self.max_scroll(ch));
        } else {
            let count = self.item_visible.len() as i32;
            let window = ((ch - ITEM_SEARCH_HEIGHT + ROW_STEP) / ROW_STEP).max(1);
            let max_scroll = (count - window).max(0);
            let delta = ((scroll[0] - scroll[1]) as i32) * 4;
            self.item_scroll[self.item_tab] = (self.item_scroll[self.item_tab] + delta).clamp(0, max_scroll);
        }
    }

    pub(super) fn key(&mut self, game: &Game<'_>, key: &str, modifiers: u32) {
        if key == "key.keyboard.escape" {
            self.done(game);
            return;
        }
        // Backspace edits the focused search field.
        if key == "key.keyboard.backspace"
            && self.focus == Some(Widget::ItemSearch)
            && self.item_tab != 0
        {
            self.item_search[self.item_tab].pop();
            return;
        }
        if key == "key.keyboard.tab" {
            let order = self.focus_order();
            if order.is_empty() {
                return;
            }
            let at = self.focus.and_then(|f| order.iter().position(|o| *o == f));
            let next = match (at, modifiers & 1 != 0) {
                (None, false) => 0,
                (None, true) => order.len() - 1,
                (Some(i), false) => (i + 1) % order.len(),
                (Some(i), true) => (i + order.len() - 1) % order.len(),
            };
            self.focus = Some(order[next]);
            return;
        }
        // `WScrollBar.onKeyPressed`: the arrows move the scroll bar.
        if self.focus == Some(Widget::ScrollBar) {
            let ch = self.card_h();
            let delta = match key {
                "key.keyboard.down" | "key.keyboard.right" => 1,
                "key.keyboard.up" | "key.keyboard.left" => -1,
                _ => 0,
            };
            if delta != 0 {
                self.scrollbar_value = (self.scrollbar_value + delta).clamp(0, self.max_scroll(ch));
                return;
            }
        }
        // `WAbstractDoubleSlider`'s arrows move the focused slider by its step.
        if let Some(Widget::Slider(i)) = self.focus {
            let delta = match key {
                "key.keyboard.right" | "key.keyboard.up" => 1.0,
                "key.keyboard.left" | "key.keyboard.down" => -1.0,
                _ => 0.0,
            };
            if delta != 0.0 {
                let step = m::slider_range(i).2;
                let v = glue::settings().slider(i) + delta * step;
                glue::set_slider_local(i, v);
                return;
            }
        }
        let enter = matches!(key, "key.keyboard.enter" | "key.keyboard.keypad.enter" | "key.keyboard.space");
        if let Some(f) = self.focus
            && enter
        {
            match f {
                Widget::Tab => self.click(),
                Widget::Done => {
                    self.click();
                    self.done(game);
                }
                Widget::Reset => {
                    self.click();
                    self.reset(game);
                }
                Widget::Row(index) => {
                    self.click();
                    glue::toggle_local(index);
                }
                Widget::Choice(index) => {
                    self.click();
                    glue::cycle_choice_local(index);
                }
                Widget::Item(idx) => {
                    self.click();
                    if let Some(id) = self.item_visible.get(idx).cloned() {
                        glue::toggle_item_local(self.item_tab, &id);
                    }
                }
                Widget::ItemSearch => {}
                Widget::ScrollBar => {}
                Widget::Slider(_) => {}
            }
        }
    }

    /// A character typed: into the focused search field.
    pub(super) fn char_typed(&mut self, c: char) {
        if self.focus == Some(Widget::ItemSearch) && self.item_tab != 0 {
            self.item_search[self.item_tab].push(c);
        }
    }

    fn focus_order(&self) -> Vec<Widget> {
        let mut out = vec![Widget::Tab];
        if self.item_tab == 0 {
            let ch = self.card_h();
            if self.needs_scroll(ch) {
                out.push(Widget::ScrollBar);
            }
            for (_, kind, _) in self.rows(ch) {
                match kind {
                    m::Row::Toggle(i) => out.push(Widget::Row(i)),
                    m::Row::Slider(i) => out.push(Widget::Slider(i)),
                    m::Row::Choice(i) => out.push(Widget::Choice(i)),
                    _ => {}
                }
            }
        } else {
            out.push(Widget::ItemSearch);
            let count = self.item_visible.len() as i32;
            let window = ((self.card_h() - ITEM_SEARCH_HEIGHT + ROW_STEP) / ROW_STEP).max(1);
            let max_scroll = (count - window).max(0);
            let scroll = self.item_scroll[self.item_tab].clamp(0, max_scroll);
            for (vi, _) in self.item_visible.iter().skip(scroll as usize).enumerate() {
                let ly = CARD[1] + 2 + ROW_STEP * vi as i32;
                if ly < CARD[1] + (self.card_h() - ITEM_SEARCH_HEIGHT) {
                    out.push(Widget::Item((scroll + vi as i32) as usize));
                }
            }
        }
        out.push(Widget::Done);
        out.push(Widget::Reset);
        out
    }
}

/// The checkbox texture (the sprite's native `CHECKBOX` side).
fn checkbox(batch: &mut HudBatch, atlas: &HudAtlas, s: &sprites::Sprite, x: i32, y: i32) {
    sprite(batch, atlas, s, x, y, CHECKBOX, CHECKBOX);
}

/// The item icon the tab shows.
fn stack(item: &str) -> ItemStack {
    ItemStack { item: item.into(), count: 1, damage: 0, extra: None }
}

/// `isWithinBounds`: the point inside the rectangle (right and bottom excluded).
fn within(p: (i32, i32), r: [i32; 4]) -> bool {
    p.0 >= r[0] && p.0 < r[0] + r[2] && p.1 >= r[1] && p.1 < r[1] + r[3]
}

/// A tooltip's text at most this many pixels wide before it wraps.
const TOOLTIP_WIDTH: i32 = 200;
/// How long the pointer must rest on a row before its tooltip shows.
const TOOLTIP_DELAY: std::time::Duration = std::time::Duration::from_millis(600);

impl Gui {
    /// Remembers the hovered row's tooltip and restarts the delay when the
    /// pointer moves to another row, unless the language has none.
    fn tooltip(&mut self, id: usize, text: String) {
        if text.is_empty() {
            return;
        }
        let since = match &self.tooltip_hover {
            Some((prev, t)) if *prev == id => *t,
            _ => Instant::now(),
        };
        self.tooltip_hover = Some((id, since));
        self.tooltip = Some(text);
    }
}

/// The vanilla-style tooltip: the dark box with a blue frame, at the pointer,
/// clamped to the window (`GuiGraphics.renderComponentTooltip`).
fn draw_tooltip(batch: &mut HudBatch, atlas: &HudAtlas, tip: &str, cursor: (f32, f32), w: i32, h: i32) {
    let lines = wrap(atlas, tip, TOOLTIP_WIDTH);
    if lines.is_empty() {
        return;
    }
    let tw = lines.iter().map(|l| width(atlas, &Text::literal(l))).max().unwrap_or(0) + 8;
    let th = lines.len() as i32 * 10 + 6;
    let x = (cursor.0 as i32 + 12).clamp(0, (w - tw).max(0));
    let y = (cursor.1 as i32 - 14).clamp(0, (h - th).max(0));
    fill(batch, atlas, x + 2, y + 2, tw - 4, th - 4, 0xF010_0010);
    fill(batch, atlas, x + 1, y + 1, tw - 2, 1, 0x5050_00FF);
    fill(batch, atlas, x + 1, y + th - 2, tw - 2, 1, 0x5050_00FF);
    fill(batch, atlas, x + 1, y + 2, 1, th - 4, 0x5050_00FF);
    fill(batch, atlas, x + tw - 2, y + 2, 1, th - 4, 0x5050_00FF);
    for (li, line) in lines.iter().enumerate() {
        text(batch, atlas, &Text::literal(line), x + 4, y + 5 + li as i32 * 10, 0xFFFF_FFFF, false);
    }
}

/// Greedy word wrap to `max` px; a word wider than the whole line is
/// hard-broken (CJK has no spaces).
fn wrap(atlas: &HudAtlas, s: &str, max: i32) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    for word in s.split_whitespace() {
        let mut word = word;
        while width(atlas, &Text::literal(word)) > max {
            let mut n = word.chars().count().max(1);
            while n > 1
                && width(atlas, &Text::literal(&word.chars().take(n).collect::<String>())) > max
            {
                n -= 1;
            }
            let head: String = word.chars().take(n).collect();
            lines.push(head.clone());
            word = &word[head.len()..];
        }
        let fits = match lines.last() {
            Some(l) => width(atlas, &Text::literal(&format!("{l} {word}"))) <= max,
            None => false,
        };
        if fits {
            let l = lines.last_mut().unwrap();
            l.push(' ');
            l.push_str(word);
        } else {
            lines.push(word.to_string());
        }
    }
    lines
}