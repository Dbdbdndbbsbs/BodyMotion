//! Body Motion native mod: the launcher compiles this crate against the pinned game and calls `hooks()`.
//! The settings live in `runtime` (pure, tested); the per-frame settings cache in `glue` is
//! refreshed here and written back when the settings screen changes it; the game-side mixins
//! read the cache. The settings screen is the custom GUI over own page in
//! Options > Mods.

pub mod glue;
mod pet;
pub mod runtime;

use mod_api::{
    ClientHost, Flow, Hooks, Registrar, RegistrationContext, RenderFramePreEvent,
    ScreenCharTypedPreEvent, ScreenRenderPostEvent, SettingSpec,
};

#[derive(Default)]
struct Mod;

thread_local! {
    static LOADED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

#[mod_api::horizon_mod]
impl Hooks for Mod {
    fn register(&self, r: &mut dyn Registrar, _cx: &RegistrationContext) {
        // The settings are registered so page exists in Options >
        // Mods (the page is the custom screen's host); the rows themselves are
        // always hidden by `scr_cover`, so the player only ever sees the
        // custom screen. Values persist through own config file
        // (`glue::save_settings`).
        for (key, label, default) in TOGGLES {
            let _ = r.register_setting(SettingSpec::toggle(key, label, default));
        }
        for (key, label, min, max, step, default) in SLIDERS {
            let _ = r.register_setting(SettingSpec::range(key, label, min, max, step, default));
        }
        let _ = r.register_setting(SettingSpec::choice(
            "bodymotion.body_lock_mode",
            "text.bodymotion.rotationlock",
            [
                "text.bodymotion.rotationlock.NONE",
                "text.bodymotion.rotationlock.FIXED",
                "text.bodymotion.rotationlock.SMOOTH",
            ],
            0,
        ));
        let _ = r.register_setting(SettingSpec::choice(
            "bodymotion.hold_mode",
            "text.bodymotion.holdUpItemsMode",
            [
                "text.bodymotion.holdUpItemsMode.CONFIG",
                "text.bodymotion.holdUpItemsMode.ALL",
                "text.bodymotion.holdUpItemsMode.NONE",
                "text.bodymotion.holdUpItemsMode.CONFIG_INVERTED",
            ],
            0,
        ));
        let _ = r.register_setting(SettingSpec::choice(
            "bodymotion.hold_target",
            "text.bodymotion.holdUpTarget",
            ["text.bodymotion.holdUpTarget.NONE", "text.bodymotion.holdUpTarget.CAMERA"],
            1,
        ));
    }

    /// Once a frame: refresh the settings cache and flush edits. `client_tick_pre`
    /// does not run outside a world, so the settings screen writes here too.
    fn render_frame_pre(&self, host: &mut dyn ClientHost, _: &mut RenderFramePreEvent) {
        // Load the saved settings into the cache once, at the first frame; from
        // then on the cache is authoritative (the host's read snapshot can lag
        // behind writes, so re-reading it every frame would revert edits).
        if !LOADED.with(|c| c.replace(true)) {
            glue::update_from_host(host);
        }
        if glue::take_dirty() {
            let pairs = glue::settings().saved();
            glue::save_settings(host, &pairs);
            glue::save_lists(host);
        }
    }

    // These two hooks are subscribed for the host's listener table alone: the
    // game only runs the screen render pass and offers typed characters when a
    // mod listens, and the drawing and the typing themselves arrive through
    // the patched native hooks (`native_bm_screen`). Without the subscription
    // the config screen is never drawn.
    fn screen_render_post(&self, _host: &dyn ClientHost, _event: &mut ScreenRenderPostEvent<'_>) {}

    fn screen_char_typed_pre(
        &self,
        _host: &dyn ClientHost,
        _event: &mut ScreenCharTypedPreEvent<'_>,
    ) -> Flow {
        Flow::Continue
    }

}

/// The toggles, in the screen's order: storage key, translation key, default.
const TOGGLES: [(&str, &str, bool); 27] = [
    (
        "bodymotion.map3d",
        "text.bodymotion.enable.inworldmaprendering",
        true,
    ),
    ("bodymotion.climb", "text.bodymotion.enable.ladderanimation", true),
    ("bodymotion.align_ladder", "text.bodymotion.enable.rotatetoladder", true),
    ("bodymotion.belly", "text.bodymotion.enable.crawling", true),
    ("bodymotion.consume", "text.bodymotion.enable.eatdrinkanimation", true),
    ("bodymotion.row", "text.bodymotion.enable.rowboatanimation", true),
    (
        "bodymotion.ride_horse",
        "text.bodymotion.enable.horseanimation",
        true,
    ),
    ("bodymotion.handoff", "text.bodymotion.enable.itemSwapAnimation", true),
    ("bodymotion.stroke", "text.bodymotion.enable.petAnimation", true),
    ("bodymotion.shiver", "text.bodymotion.enable.freezingAnimation", true),
    (
        "bodymotion.bow_clamp",
        "text.bodymotion.enable.clampCrossbowAnimations",
        false,
    ),
    ("bodymotion.alight", "text.bodymotion.enable.burningAnimation", true),
    (
        "bodymotion.smooth",
        "text.bodymotion.enable.animationsmoothing",
        true,
    ),
    (
        "bodymotion.smooth_legs",
        "text.bodymotion.disableLegSmoothing",
        false,
    ),
    (
        "bodymotion.lock_first_person",
        "text.bodymotion.enable.limitRotationLockToFP",
        true,
    ),
    (
        "bodymotion.lock_remote",
        "text.bodymotion.enable.applyRotationLockToEveryone",
        false,
    ),
    (
        "bodymotion.body_lock",
        "text.bodymotion.enable.rotationlocking",
        true,
    ),
    ("bodymotion.sway_lamps", "text.bodymotion.animateLanterns", true),
    ("bodymotion.hold_self_only", "text.bodymotion.enable.holdUpOnlySelf", false),
    (
        "bodymotion.glide_tweak",
        "text.bodymotion.enable.tweakElytraAnimation",
        true,
    ),
    (
        "bodymotion.bed_hands",
        "text.bodymotion.enable.dontholditemsinbed",
        true,
    ),
    ("bodymotion.rest", "text.bodymotion.enable.freezearmsinbed", true),
    ("bodymotion.bow_offhand", "text.bodymotion.enable.offhandhiding", true),
    ("bodymotion.dash", "text.bodymotion.enable.narutoRunning", false),
    (
        "bodymotion.embrace",
        "text.bodymotion.enable.huggingAnimation",
        false,
    ),
    (
        "bodymotion.tumble",
        "text.bodymotion.enable.fallingAnimation",
        false,
    ),
    (
        "bodymotion.horse_stance",
        "text.bodymotion.enable.enableHorseLegAnimation",
        false,
    ),
];

/// The sliders: storage key, translation key, min, max, step, default.
const SLIDERS: [(&str, &str, f64, f64, f64, f64); 8] = [
    (
        "bodymotion.climb_swing",
        "text.bodymotion.ladderAnimationAmplifier",
        0.05,
        0.6,
        0.02,
        0.28,
    ),
    (
        "bodymotion.climb_reach",
        "text.bodymotion.ladderAnimationArmHeight",
        0.8,
        3.2,
        0.2,
        1.55,
    ),
    (
        "bodymotion.climb_pace",
        "text.bodymotion.ladderAnimationArmSpeed",
        0.5,
        4.5,
        0.25,
        2.2,
    ),
    (
        "bodymotion.smooth_speed",
        "text.bodymotion.smoothingSpeed",
        0.05,
        1.0,
        0.05,
        0.25,
    ),
    (
        "bodymotion.head_limit",
        "text.bodymotion.maxNormalAngle",
        5.0,
        60.0,
        1.0,
        50.0,
    ),
    (
        "bodymotion.block_head_limit",
        "text.bodymotion.maxBlockingAngle",
        5.0,
        60.0,
        1.0,
        15.0,
    ),
    (
        "bodymotion.hold_offset_item",
        "text.bodymotion.holdUpItemOffset",
        -0.6,
        0.5,
        0.05,
        0.0,
    ),
    (
        "bodymotion.hold_offset_camera",
        "text.bodymotion.holdUpCameraOffset",
        -0.5,
        0.7,
        0.05,
        0.1,
    ),
];


