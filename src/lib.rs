//! Body Motion native mod: the launcher compiles this crate against the pinned game and calls `hooks()`.
//! The settings live in `runtime` (pure, tested); the per-frame settings cache in `glue` is
//! refreshed here and written back when the settings screen changes it; the game-side mixins
//! read the cache. The settings screen is the custom GUI over own page in
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
            "bodymotion.bow_pose",
            "text.bodymotion.enable.bowAnimation",
            [
                "text.bodymotion.enable.bowAnimation.VANILLA",
                "text.bodymotion.enable.bowAnimation.CUSTOM_V1",
            ],
            0,
        ));
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
    /// vanilla bow base plus deltas in `bm_anim`; that mixin still
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
        "bodymotion.bow_lock",
        "text.bodymotion.enable.customBowRotationLock",
        false,
    ),
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
        0.1,
        0.5,
        0.01,
        0.28,
    ),
    (
        "bodymotion.climb_reach",
        "text.bodymotion.ladderAnimationArmHeight",
        1.0,
        3.0,
        0.1,
        1.55,
    ),
    (
        "bodymotion.climb_pace",
        "text.bodymotion.ladderAnimationArmSpeed",
        1.0,
        4.0,
        0.1,
        2.2,
    ),
    (
        "bodymotion.smooth_speed",
        "text.bodymotion.smoothingSpeed",
        0.1,
        1.0,
        0.1,
        0.25,
    ),
    (
        "bodymotion.head_limit",
        "text.bodymotion.maxNormalAngle",
        0.0,
        50.0,
        0.5,
        50.0,
    ),
    (
        "bodymotion.block_head_limit",
        "text.bodymotion.maxBlockingAngle",
        0.0,
        50.0,
        0.5,
        15.0,
    ),
    (
        "bodymotion.hold_offset_item",
        "text.bodymotion.holdUpItemOffset",
        -0.5,
        0.4,
        0.1,
        0.0,
    ),
    (
        "bodymotion.hold_offset_camera",
        "text.bodymotion.holdUpCameraOffset",
        -0.3,
        0.6,
        0.1,
        0.1,
    ),
];


