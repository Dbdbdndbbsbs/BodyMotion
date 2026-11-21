//! Body Motion native mod: the launcher compiles this crate against the pinned game and calls `hooks()`.
//! The settings live in `runtime` (pure, tested); the per-frame settings cache in `glue` is
//! refreshed here and written back when the settings screen changes it; the game-side mixins
//! read the cache. The settings screen is the custom GUI over the mod's own page in
//! Options > Mods.

pub mod glue;
mod pet;
pub mod runtime;

use mod_api::{
    ArmPose, CalculatePlayerTurnEvent, ClientHost, CustomArmPose, Flow, GetArmPoseEvent,
    ArmPoseTransformEvent, Hooks, Registrar, RegistrationContext, RenderFramePreEvent,
    ScreenCharTypedPreEvent, ScreenRenderPostEvent, SettingSpec, UseAnimation,
};

#[derive(Default)]
struct Mod;

/// The registered custom bow-draw pose (`get_arm_pose` returns it for a bow in
/// the custom mode; `arm_pose_transform` writes its arm angles).
static BOW_POSE: std::sync::OnceLock<CustomArmPose> = std::sync::OnceLock::new();

thread_local! {
    static LOADED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

#[mod_api::horizon_mod]
impl Hooks for Mod {
    fn register(&self, r: &mut dyn Registrar, _cx: &RegistrationContext) {
        // The settings are registered so the mod's page exists in Options >
        // Mods (the page is the custom screen's host); the rows themselves are
        // always hidden by `scr_cover`, so the player only ever sees the
        // custom screen. Values persist through the mod's own config file
        // (`glue::save_settings`).
        for (key, label, default) in TOGGLES {
            let _ = r.register_setting(SettingSpec::toggle(key, label, default));
        }
        for (key, label, min, max, step, default) in SLIDERS {
            let _ = r.register_setting(SettingSpec::range(key, label, min, max, step, default));
        }
        let _ = r.register_setting(SettingSpec::choice(
            "bm.bowAnimation",
            "text.bm.enable.bowAnimation",
            [
                "text.bm.enable.bowAnimation.VANILLA",
                "text.bm.enable.bowAnimation.CUSTOM_V1",
            ],
            0,
        ));
        let _ = r.register_setting(SettingSpec::choice(
            "bm.rotationLock",
            "text.bm.rotationlock",
            [
                "text.bm.rotationlock.NONE",
                "text.bm.rotationlock.FIXED",
                "text.bm.rotationlock.SMOOTH",
            ],
            0,
        ));
        let _ = r.register_setting(SettingSpec::choice(
            "bm.holdUpItemsMode",
            "text.bm.holdUpItemsMode",
            [
                "text.bm.holdUpItemsMode.CONFIG",
                "text.bm.holdUpItemsMode.ALL",
                "text.bm.holdUpItemsMode.NONE",
                "text.bm.holdUpItemsMode.CONFIG_INVERTED",
            ],
            0,
        ));
        let _ = r.register_setting(SettingSpec::choice(
            "bm.holdUpTarget",
            "text.bm.holdUpTarget",
            ["text.bm.holdUpTarget.NONE", "text.bm.holdUpTarget.CAMERA"],
            1,
        ));
        let pose = r.register_arm_pose("bodymotion:bow_custom_v1", true, true);
        let _ = BOW_POSE.set(pose);
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

    /// Slightly damp the mouse turn while the custom bow draw is on, for a
    /// steadier aim (one frame at a time; options are unchanged).
    fn calculate_player_turn(
        &self,
        _host: &dyn ClientHost,
        event: &mut CalculatePlayerTurnEvent,
    ) {
        let s = glue::settings();
        if s.enabled && s.bow_mode == runtime::BowAnimation::CustomV1 {
            event.mouse_sensitivity *= 0.85;
        }
    }

    fn screen_char_typed_pre(
        &self,
        _host: &dyn ClientHost,
        _event: &mut ScreenCharTypedPreEvent<'_>,
    ) -> Flow {
        Flow::Continue
    }

    /// Replace the arm pose for a drawn bow when the custom bow animation is
    /// on. Asked for local players, remote players and humanoid mobs alike.
    fn get_arm_pose(
        &self,
        _host: &dyn ClientHost,
        event: &mut GetArmPoseEvent<'_>,
    ) -> Option<ArmPose> {
        let s = glue::settings();
        if !s.enabled || s.bow_mode != runtime::BowAnimation::CustomV1 {
            return None;
        }
        if event.humanoid.use_animation != UseAnimation::Bow {
            return None;
        }
        BOW_POSE.get().copied().map(ArmPose::Custom)
    }

    /// Write the custom bow pose's arm angles. The old pose came from the
    /// vanilla bow base plus the mod's deltas in `bm_anim`; that mixin still
    /// runs afterwards, so this custom pose repeats the vanilla base exactly
    /// (a level aim: the hook has no head pitch/yaw) and the old look returns.
    fn arm_pose_transform(
        &self,
        _host: &dyn ClientHost,
        event: &mut ArmPoseTransformEvent<'_>,
    ) {
        let Some(token) = BOW_POSE.get() else {
            return;
        };
        if &event.pose != token {
            return;
        }
        let x = -std::f32::consts::FRAC_PI_2;
        if event.right_arm {
            event.angles.right = [x, -0.1, 0.0];
        } else {
            event.angles.left = [x, 0.1, 0.0];
        }
    }
}

/// The toggles, in the screen's order: storage key, translation key, default.
const TOGGLES: [(&str, &str, bool); 28] = [
    (
        "bm.inWorldMapRendering",
        "text.bm.enable.inworldmaprendering",
        true,
    ),
    ("bm.ladder", "text.bm.enable.ladderanimation", true),
    ("bm.rotateToLadder", "text.bm.enable.rotatetoladder", true),
    ("bm.crawling", "text.bm.enable.crawling", true),
    ("bm.eatDrink", "text.bm.enable.eatdrinkanimation", true),
    ("bm.boat", "text.bm.enable.rowboatanimation", true),
    (
        "bm.enableHorseAnimation",
        "text.bm.enable.horseanimation",
        true,
    ),
    ("bm.itemSwap", "text.bm.enable.itemSwapAnimation", true),
    ("bm.petAnimation", "text.bm.enable.petAnimation", true),
    ("bm.freezing", "text.bm.enable.freezingAnimation", true),
    (
        "bm.customBowRotationLock",
        "text.bm.enable.customBowRotationLock",
        false,
    ),
    (
        "bm.clampCrossbowAnimations",
        "text.bm.enable.clampCrossbowAnimations",
        false,
    ),
    ("bm.burning", "text.bm.enable.burningAnimation", true),
    (
        "bm.enableAnimationSmoothing",
        "text.bm.enable.animationsmoothing",
        true,
    ),
    (
        "bm.disableLegSmoothing",
        "text.bm.disableLegSmoothing",
        false,
    ),
    (
        "bm.limitRotationLockToFP",
        "text.bm.enable.limitRotationLockToFP",
        true,
    ),
    (
        "bm.applyRotationLockToEveryone",
        "text.bm.enable.applyRotationLockToEveryone",
        false,
    ),
    (
        "bm.enableRotationLocking",
        "text.bm.enable.rotationlocking",
        true,
    ),
    ("bm.animateLanterns", "text.bm.animateLanterns", true),
    ("bm.holdUpOnlySelf", "text.bm.enable.holdUpOnlySelf", false),
    (
        "bm.tweakElytraAnimation",
        "text.bm.enable.tweakElytraAnimation",
        true,
    ),
    (
        "bm.dontHoldItemsInBed",
        "text.bm.enable.dontholditemsinbed",
        true,
    ),
    ("bm.sleep", "text.bm.enable.freezearmsinbed", true),
    ("bm.offhandHiding", "text.bm.enable.offhandhiding", true),
    ("bm.narutoRunning", "text.bm.enable.narutoRunning", false),
    (
        "bm.huggingAnimation",
        "text.bm.enable.huggingAnimation",
        false,
    ),
    (
        "bm.fallingAnimation",
        "text.bm.enable.fallingAnimation",
        false,
    ),
    (
        "bm.enableHorseLegAnimation",
        "text.bm.enable.enableHorseLegAnimation",
        false,
    ),
];

/// The sliders: storage key, translation key, min, max, step, default.
const SLIDERS: [(&str, &str, f64, f64, f64, f64); 8] = [
    (
        "bm.ladderAnimationAmplifier",
        "text.bm.ladderAnimationAmplifier",
        0.1,
        0.5,
        0.01,
        0.35,
    ),
    (
        "bm.ladderAnimationArmHeight",
        "text.bm.ladderAnimationArmHeight",
        1.0,
        3.0,
        0.1,
        1.7,
    ),
    (
        "bm.ladderAnimationArmSpeed",
        "text.bm.ladderAnimationArmSpeed",
        1.0,
        4.0,
        0.1,
        2.0,
    ),
    (
        "bm.smoothingSpeed",
        "text.bm.smoothingSpeed",
        0.1,
        1.0,
        0.1,
        0.2,
    ),
    (
        "bm.maxNormalAngle",
        "text.bm.maxNormalAngle",
        0.0,
        50.0,
        0.5,
        50.0,
    ),
    (
        "bm.maxBlockingAngle",
        "text.bm.maxBlockingAngle",
        0.0,
        50.0,
        0.5,
        15.0,
    ),
    (
        "bm.holdUpItemOffset",
        "text.bm.holdUpItemOffset",
        -0.5,
        0.4,
        0.1,
        0.0,
    ),
    (
        "bm.holdUpCameraOffset",
        "text.bm.holdUpCameraOffset",
        -0.3,
        0.6,
        0.1,
        0.1,
    ),
];


