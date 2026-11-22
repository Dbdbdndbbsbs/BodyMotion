//! The mod's game-facing half: a per-frame cache of the parsed settings, the
//! current language, the off-hand hiding flags the first-person/item mixins read,
//! and the mutators the settings screen uses (the screen runs in the game's
//! scope, so it cannot call `Host` itself; it marks the cache dirty and
//! `Hooks::client_tick_pre` writes it out).

use std::cell::{Cell, RefCell};

use mod_api::{ClientHost, ConfigPath};

use crate::runtime::{self, Settings};

/// The bulk file holding the two item lists (they have no `SettingKind`, so
/// they cannot be registered settings). The path is owned by this mod
/// (`config_files` refuses anything outside `bodymotion/...`).
const LIST_FILE: &str = "bodymotion/item-lists.json";
/// The bulk file holding the settings themselves: none are registered as game
/// settings, so the game's settings page shows only the custom screen; the
/// values live in this file instead.
const SETTINGS_FILE: &str = "bodymotion/settings.json";

thread_local! {
    static SETTINGS: RefCell<Option<Settings>> = const { RefCell::new(None) };
    static LANGUAGE: RefCell<String> = const { RefCell::new(String::new()) };
    static DIRTY: Cell<bool> = const { Cell::new(false) };
    static HIDE_OFFHAND: Cell<bool> = const { Cell::new(false) };
    static HIDE_MAIN: Cell<bool> = const { Cell::new(false) };
}

/// The current settings (defaults until the frame hook has loaded them).
pub fn settings() -> Settings {
    SETTINGS.with(|c| c.borrow().clone().unwrap_or_default())
}

pub fn set_settings(s: Settings) {
    SETTINGS.with(|c| *c.borrow_mut() = Some(s));
}

/// A bit mask of the 28 toggles (bit i = toggle i) for diagnostics.
pub fn mask() -> u32 {
    let s = settings();
    (0..28).fold(0u32, |m, i| m | ((s.get(i) as u32) << i))
}

/// The current language (from `Host::client_language`, cached each frame).
pub fn language() -> String {
    let lang = LANGUAGE.with(|c| c.borrow().clone());
    if lang.is_empty() {
        "en_us".into()
    } else {
        lang
    }
}

pub fn set_language(lang: String) {
    LANGUAGE.with(|c| *c.borrow_mut() = lang);
}

/// The settings changed; `Hooks::client_tick_pre` writes them to the host.
pub fn mark_dirty() {
    DIRTY.with(|c| c.set(true));
}

fn dbg_res(_msg: &str) {}

pub fn take_dirty() -> bool {
    DIRTY.with(|c| c.replace(false))
}

fn settings_path() -> Option<ConfigPath> {
    ConfigPath::new(SETTINGS_FILE).ok()
}

/// The earlier names of each setting, for migrating saved files.
fn legacy_names(key: &str) -> &'static [&'static str] {
    match key {
        "bodymotion.master" => &["bodymotion.enabled", "bm.enabled"],
        "bodymotion.consume" => &["bodymotion.eatDrink", "bm.eatDrink"],
        "bodymotion.climb" => &["bodymotion.ladder", "bm.ladder"],
        "bodymotion.align_ladder" => &["bodymotion.rotateToLadder", "bm.rotateToLadder"],
        "bodymotion.climb_swing" => &["bodymotion.ladderAnimationAmplifier", "bm.ladderAnimationAmplifier"],
        "bodymotion.climb_reach" => &["bodymotion.ladderAnimationArmHeight", "bm.ladderAnimationArmHeight"],
        "bodymotion.climb_pace" => &["bodymotion.ladderAnimationArmSpeed", "bm.ladderAnimationArmSpeed"],
        "bodymotion.row" => &["bodymotion.boat", "bm.boat"],
        "bodymotion.handoff" => &["bodymotion.itemSwap", "bm.itemSwap"],
        "bodymotion.alight" => &["bodymotion.burning", "bm.burning"],
        "bodymotion.shiver" => &["bodymotion.freezing", "bm.freezing"],
        "bodymotion.belly" => &["bodymotion.crawling", "bm.crawling"],
        "bodymotion.rest" => &["bodymotion.sleep", "bm.sleep"],
        "bodymotion.map3d" => &["bodymotion.inWorldMapRendering", "bm.inWorldMapRendering"],
        "bodymotion.bow_offhand" => &["bodymotion.offhandHiding", "bm.offhandHiding"],
        "bodymotion.hold_list" => &["bodymotion.holdingItems", "bm.holdingItems"],
        "bodymotion.lamp_list" => &["bodymotion.lanternItems", "bm.lanternItems"],
        "bodymotion.sway_lamps" => &["bodymotion.animateLanterns", "bm.animateLanterns"],
        "bodymotion.embrace" => &["bodymotion.huggingAnimation", "bm.huggingAnimation"],
        "bodymotion.stroke" => &["bodymotion.petAnimation", "bm.petAnimation"],
        "bodymotion.dash" => &["bodymotion.narutoRunning", "bm.narutoRunning"],
        "bodymotion.tumble" => &["bodymotion.fallingAnimation", "bm.fallingAnimation"],
        "bodymotion.ride_horse" => &["bodymotion.enableHorseAnimation", "bm.enableHorseAnimation"],
        "bodymotion.horse_stance" => &["bodymotion.enableHorseLegAnimation", "bm.enableHorseLegAnimation"],
        "bodymotion.body_lock" => &["bodymotion.enableRotationLocking", "bm.enableRotationLocking"],
        "bodymotion.body_lock_mode" => &["bodymotion.rotationLock", "bm.rotationLock"],
        "bodymotion.lock_first_person" => &["bodymotion.limitRotationLockToFP", "bm.limitRotationLockToFP"],
        "bodymotion.lock_remote" => &["bodymotion.applyRotationLockToEveryone", "bm.applyRotationLockToEveryone"],
        "bodymotion.head_limit" => &["bodymotion.maxNormalAngle", "bm.maxNormalAngle"],
        "bodymotion.block_head_limit" => &["bodymotion.maxBlockingAngle", "bm.maxBlockingAngle"],
        "bodymotion.bow_pose" => &["bodymotion.bowAnimation", "bm.bowAnimation"],
        "bodymotion.bow_lock" => &["bodymotion.customBowRotationLock", "bm.customBowRotationLock"],
        "bodymotion.bow_clamp" => &["bodymotion.clampCrossbowAnimations", "bm.clampCrossbowAnimations"],
        "bodymotion.glide_tweak" => &["bodymotion.tweakElytraAnimation", "bm.tweakElytraAnimation"],
        "bodymotion.smooth" => &["bodymotion.enableAnimationSmoothing", "bm.enableAnimationSmoothing"],
        "bodymotion.smooth_legs" => &["bodymotion.disableLegSmoothing", "bm.disableLegSmoothing"],
        "bodymotion.smooth_speed" => &["bodymotion.smoothingSpeed", "bm.smoothingSpeed"],
        "bodymotion.hold_mode" => &["bodymotion.holdUpItemsMode", "bm.holdUpItemsMode"],
        "bodymotion.hold_target" => &["bodymotion.holdUpTarget", "bm.holdUpTarget"],
        "bodymotion.hold_offset_item" => &["bodymotion.holdUpItemOffset", "bm.holdUpItemOffset"],
        "bodymotion.hold_offset_camera" => &["bodymotion.holdUpCameraOffset", "bm.holdUpCameraOffset"],
        "bodymotion.hold_self_only" => &["bodymotion.holdUpOnlySelf", "bm.holdUpOnlySelf"],
        "bodymotion.bed_hands" => &["bodymotion.dontHoldItemsInBed", "bm.dontHoldItemsInBed"],
        _ => &[],
    }
}

/// Read one saved setting from the mod's config file. Keys recorded under the
/// earlier `bodymotion.*` or `bm.*` schemas are still honoured.
fn read_settings(host: &dyn ClientHost, key: &str) -> Option<String> {
    let path = settings_path()?;
    let bytes = host.config_files().read(&path).ok()?;
    let v: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
    if let Some(s) = v.get(key).and_then(|x| x.as_str()) {
        return Some(s.to_owned());
    }
    for old in legacy_names(key) {
        if let Some(s) = v.get(old).and_then(|x| x.as_str()) {
            return Some(s.to_owned());
        }
    }
    None
}

/// Write the saved settings to the mod's config file.
pub fn save_settings(host: &dyn ClientHost, pairs: &[(String, String)]) {
    let Some(path) = settings_path() else {
        return;
    };
    let mut map = serde_json::Map::new();
    for (key, value) in pairs {
        map.insert(key.clone(), serde_json::Value::String(value.clone()));
    }
    let bytes = serde_json::to_vec(&serde_json::Value::Object(map)).unwrap_or_default();
    let _ = host.config_files().write(&path, &bytes);
}

/// Re-read the settings from the mod's config file (once a frame, from
/// `render_frame_pre`).
pub fn update_from_host(host: &dyn ClientHost) {
    let mut s = Settings::load(|key| read_settings(host, key));
    apply_lists(host, &mut s);
    set_settings(s);
    let lang = host
        .client_session()
        .client_language()
        .unwrap_or_else(|| "en_us".into());
    set_language(lang);
}

fn lists_path() -> Option<ConfigPath> {
    ConfigPath::new(LIST_FILE).ok()
}

fn apply_lists(host: &dyn ClientHost, s: &mut Settings) {
    let Some(path) = lists_path() else {
        return;
    };
    let Ok(bytes) = host.config_files().read(&path) else {
        return;
    };
    let Ok(v) = serde_json::from_slice::<serde_json::Value>(&bytes) else {
        return;
    };
    let list = |key: &str, fallback: &[String]| {
        v.get(key)
            .and_then(|x| x.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|e| e.as_str().map(str::to_owned))
                    .collect()
            })
            .unwrap_or_else(|| fallback.to_vec())
    };
    s.holding_items = list("holdingItems", &s.holding_items);
    s.lantern_items = list("lanternItems", &s.lantern_items);
}

/// Write the item lists to the host's bulk config (called by the dirty flush).
pub fn save_lists(host: &dyn ClientHost) {
    let Some(path) = lists_path() else {
        return;
    };
    let s = settings();
    let bytes = serde_json::to_vec(&serde_json::json!({
        "holdingItems": s.holding_items,
        "lanternItems": s.lantern_items,
    }))
    .unwrap_or_default();
    let _ = host.config_files().write(&path, &bytes);
}

/// The settings screen's mutators: change the cache and mark it dirty. The
/// writes happen when the tick hook flushes.
pub fn toggle_local(i: usize) {
    let mut s = settings();
    s.click(i);
    let en = s.enabled;
    let lad = s.ladder;
    let mask = tg_mask(&s);
    set_settings(s);
    mark_dirty();
    dbg_res(&format!("toggle_local i={i} mask=0x{mask:08x} enabled={en} ladder={lad}"));
}

fn tg_mask(s: &Settings) -> u32 {
    (0..28).fold(0u32, |m, i| m | ((s.get(i) as u32) << i))
}

pub fn set_slider_local(i: usize, v: f32) {
    let mut s = settings();
    s.set_slider(i, v);
    let amp = s.ladder_amplifier;
    let maxn = s.max_normal_angle;
    set_settings(s);
    mark_dirty();
    dbg_res(&format!("set_slider_local i={i} v={v} amp={amp} maxN={maxn}"));
}

pub fn cycle_choice_local(i: usize) {
    let mut s = settings();
    s.cycle_choice(i);
    set_settings(s);
    mark_dirty();
}

/// Whether the item with the id `id` is in the given tab's list.
pub fn item_on(s: &Settings, tab: usize, id: &str) -> bool {
    s.item_on(tab != runtime::ITEM_TAB_HOLDING, id)
}

/// Toggle the item with the id `id` in the given tab's list.
pub fn toggle_item_local(tab: usize, id: &str) {
    let mut s = settings();
    s.toggle_item(tab != runtime::ITEM_TAB_HOLDING, id);
    set_settings(s);
    mark_dirty();
}

pub fn reset_local() {
    set_settings(Settings::default());
    mark_dirty();
}

/// While the main hand holds a bow or crossbow, the off hand's item hides
/// (the mod's OffhandHidingAnimation); `hide_offhand_bm` suppresses its
/// vanilla render for the frame. A shield never hides.
pub fn set_hide_offhand(v: bool) {
    HIDE_OFFHAND.with(|c| c.set(v));
}

pub fn hide_offhand() -> bool {
    HIDE_OFFHAND.with(|c| c.get())
}

pub fn set_hide_main(v: bool) {
    HIDE_MAIN.with(|c| c.set(v));
}

pub fn hide_main() -> bool {
    HIDE_MAIN.with(|c| c.get())
}
