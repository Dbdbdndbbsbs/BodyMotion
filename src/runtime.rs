// The mod's own half: the mod's settings (one per animation, the
// sliders and the enum choices of the mod's config screen), translated
// into every language the mod ships. No game types here; `glue.rs`
// applies it. This file is also the bundle's Cargo library, so its tests run
// without a Horizon build.

pub const ID: &str = "bodymotion";
pub const NAME: &str = "Body Motion";
pub const VERSION: &str = "0.2.78";
pub const DESCRIPTION: &str = "Smoother and more expressive third-person animations: eating, drinking, climbing, riding, maps, lanterns and more.";

const KEY_ON: &str = "bodymotion.enabled";
const KEY_EAT: &str = "bodymotion.eatDrink";
const KEY_LADDER: &str = "bodymotion.ladder";
const KEY_ROTATE_LADDER: &str = "bodymotion.rotateToLadder";
const KEY_LADDER_AMP: &str = "bodymotion.ladderAnimationAmplifier";
const KEY_LADDER_HEIGHT: &str = "bodymotion.ladderAnimationArmHeight";
const KEY_LADDER_SPEED: &str = "bodymotion.ladderAnimationArmSpeed";
const KEY_BOAT: &str = "bodymotion.boat";
const KEY_SWAP: &str = "bodymotion.itemSwap";
const KEY_BURN: &str = "bodymotion.burning";
const KEY_FREEZE: &str = "bodymotion.freezing";
const KEY_CRAWL: &str = "bodymotion.crawling";
const KEY_SLEEP: &str = "bodymotion.sleep";
const KEY_MAPS: &str = "bodymotion.inWorldMapRendering";
const KEY_BOW_FIXES: &str = "bodymotion.offhandHiding";
const KEY_HOLDING: &str = "bodymotion.holdingItems";
const KEY_LANTERNS: &str = "bodymotion.lanternItems";
const KEY_ANIMATE_LANTERNS: &str = "bodymotion.animateLanterns";
const KEY_HUG: &str = "bodymotion.huggingAnimation";
const KEY_PET: &str = "bodymotion.petAnimation";
const KEY_NARUTO: &str = "bodymotion.narutoRunning";
const KEY_FALLING: &str = "bodymotion.fallingAnimation";
const KEY_HORSE: &str = "bodymotion.enableHorseAnimation";
const KEY_HORSE_LEG: &str = "bodymotion.enableHorseLegAnimation";
const KEY_ROTATION: &str = "bodymotion.enableRotationLocking";
const KEY_ROTATION_MODE: &str = "bodymotion.rotationLock";
const KEY_LIMIT_FP: &str = "bodymotion.limitRotationLockToFP";
const KEY_ROTATION_EVERYONE: &str = "bodymotion.applyRotationLockToEveryone";
const KEY_MAX_NORMAL: &str = "bodymotion.maxNormalAngle";
const KEY_MAX_BLOCKING: &str = "bodymotion.maxBlockingAngle";
const KEY_BOW: &str = "bodymotion.bowAnimation";
const KEY_CUSTOM_BOW_ROTATION: &str = "bodymotion.customBowRotationLock";
const KEY_CLAMP: &str = "bodymotion.clampCrossbowAnimations";
const KEY_ELYTRA: &str = "bodymotion.tweakElytraAnimation";
const KEY_SMOOTHING: &str = "bodymotion.enableAnimationSmoothing";
const KEY_DISABLE_LEG_SMOOTHING: &str = "bodymotion.disableLegSmoothing";
const KEY_SMOOTHING_SPEED: &str = "bodymotion.smoothingSpeed";
const KEY_HOLD_MODE: &str = "bodymotion.holdUpItemsMode";
const KEY_HOLD_TARGET: &str = "bodymotion.holdUpTarget";
const KEY_HOLD_ITEM_OFFSET: &str = "bodymotion.holdUpItemOffset";
const KEY_HOLD_CAMERA_OFFSET: &str = "bodymotion.holdUpCameraOffset";
const KEY_HOLD_ONLY_SELF: &str = "bodymotion.holdUpOnlySelf";
const KEY_DONT_HOLD_BED: &str = "bodymotion.dontHoldItemsInBed";

/// Tab identifiers used by the item lists: 0 = Settings, 1 = Hold Up Items,
/// 2 = Lanterns (the screen's tab order).
pub(crate) const ITEM_TAB_HOLDING: usize = 1;

/// Default holding-items list (stored under `holdingItems`).
pub const DEFAULT_HOLDING: &[&str] = &[    "minecraft:clock",
    "minecraft:compass",
    "minecraft:torch",
    "minecraft:lantern",
    "minecraft:soul_torch",
    "minecraft:soul_lantern",
    "minecraft:recovery_compass",
    "quad-mstv-mtv:acacia_torch",
    "quad-mstv-mtv:bamboo_torch",
    "quad-mstv-mtv:birch_torch",
    "quad-mstv-mtv:cherry_torch",
    "quad-mstv-mtv:crimson_torch",
    "quad-mstv-mtv:dark_oak_torch",
    "quad-mstv-mtv:pale_oak_torch",
    "quad-mstv-mtv:jungle_torch",
    "quad-mstv-mtv:mangrove_torch",
    "quad-mstv-mtv:spruce_torch",
    "quad-mstv-mtv:warped_torch",
    "quad-mstv-mtv:acacia_soul_torch",
    "quad-mstv-mtv:bamboo_soul_torch",
    "quad-mstv-mtv:birch_soul_torch",
    "quad-mstv-mtv:cherry_soul_torch",
    "quad-mstv-mtv:crimson_soul_torch",
    "quad-mstv-mtv:dark_oak_soul_torch",
    "quad-mstv-mtv:pale_oak_soul_torch",
    "quad-mstv-mtv:jungle_soul_torch",
    "quad-mstv-mtv:mangrove_soul_torch",
    "quad-mstv-mtv:spruce_soul_torch",
    "quad-mstv-mtv:warped_soul_torch",
    "minecraft:copper_lantern",
    "minecraft:exposed_copper_lantern",
    "minecraft:waxed_copper_lantern",
    "minecraft:waxed_exposed_copper_lantern",
    "minecraft:waxed_oxidized_copper_lantern",
    "minecraft:weathered_copper_lantern",
    "minecraft:waxed_weathered_copper_lantern",
];

/// Default lantern-items list (stored under `lanternItems`).
pub const DEFAULT_LANTERNS: &[&str] = &[
    "minecraft:waxed_copper_lantern",
    "minecraft:waxed_exposed_copper_lantern",
    "minecraft:soul_lantern",
    "minecraft:waxed_oxidized_copper_lantern",
    "minecraft:exposed_copper_lantern",
    "minecraft:lantern",
    "minecraft:waxed_weathered_copper_lantern",
    "minecraft:weathered_copper_lantern",
    "minecraft:oxidized_copper_lantern",
    "minecraft:copper_lantern",
];

/// `RotationLock` (the mod's enum).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RotationLock {
    None,
    Fixed,
    Smooth,
}

/// `HoldUpModes` (the mod's enum).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HoldUpMode {
    Config,
    All,
    None,
    ConfigInverted,
}

/// `HoldUpTarget` (the mod's enum).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HoldUpTarget {
    None,
    Camera,
}

/// `BowAnimation` (the mod's enum).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BowAnimation {
    Vanilla,
    CustomV1,
}

/// One row of the Settings tab, in the mod's order. Headings and the
/// disabled rows carry an index into their own label tables.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Row {
    Heading(usize),
    Toggle(usize),
    Slider(usize),
    Choice(usize),
    Disabled(usize),
    Blank,
}

pub const ROW_COUNT: usize = 53;

const ROWS: [Row; ROW_COUNT] = [
    Row::Heading(0),
    Row::Toggle(0),
    Row::Toggle(1),
    Row::Toggle(2),
    Row::Slider(0),
    Row::Slider(1),
    Row::Slider(2),
    Row::Toggle(3),
    Row::Toggle(4),
    Row::Toggle(5),
    Row::Toggle(6),
    Row::Toggle(27),
    Row::Toggle(7),
    Row::Toggle(8),
    Row::Toggle(9),
    Row::Choice(0),
    Row::Toggle(10),
    Row::Toggle(11),
    Row::Toggle(12),
    Row::Blank,
    Row::Heading(1),
    Row::Toggle(13),
    Row::Toggle(14),
    Row::Slider(3),
    Row::Blank,
    Row::Heading(2),
    Row::Choice(1),
    Row::Toggle(15),
    Row::Toggle(16),
    Row::Toggle(17),
    Row::Blank,
    Row::Heading(3),
    Row::Slider(4),
    Row::Slider(5),
    Row::Blank,
    Row::Heading(4),
    Row::Choice(2),
    Row::Toggle(18),
    Row::Slider(6),
    Row::Choice(3),
    Row::Slider(7),
    Row::Toggle(19),
    Row::Blank,
    Row::Heading(5),
    Row::Toggle(20),
    Row::Toggle(21),
    Row::Toggle(22),
    Row::Toggle(23),
    Row::Blank,
    Row::Heading(6),
    Row::Toggle(24),
    Row::Toggle(25),
    Row::Toggle(26),
];

pub fn row(i: usize) -> Row {
    ROWS.get(i).copied().unwrap_or(Row::Blank)
}

/// The number of toggle settings (the same order as `get`/`click`).
pub const TOGGLE_COUNT: usize = 28;
/// The number of slider settings (the same order as `slider`).
pub const SLIDER_COUNT: usize = 8;
/// The number of enum choices (the same order as `choice`).
pub const CHOICE_COUNT: usize = 4;

#[derive(Clone, Debug, PartialEq)]
pub struct Settings {
    /// Master switch: `false` turns every animation off.
    pub enabled: bool,
    pub eat_drink: bool,
    pub ladder: bool,
    /// The body snaps to face the ladder while climbing.
    pub rotate_to_ladder: bool,
    pub ladder_amplifier: f32,
    pub ladder_arm_height: f32,
    pub ladder_arm_speed: f32,
    pub boat: bool,
    pub item_swap: bool,
    pub burning: bool,
    pub freezing: bool,
    pub crawling: bool,
    /// Arms lie still in bed (`freezeArmsInBed`).
    pub sleep: bool,
    /// In-world map rendering: the held map is drawn as a 3D map.
    pub maps: bool,
    /// Bow animation fixes: the other hand's item hides while a bow or
    /// crossbow is drawn or charged.
    pub offhand_hiding: bool,
    /// Items that are "held up" (the Hold Up Items tab).
    pub holding_items: Vec<String>,
    /// Items that are animated lanterns (the Lanterns tab).
    pub lantern_items: Vec<String>,
    /// Whether lantern animations are active.
    pub animate_lanterns: bool,
    /// Simple hugging animation (`huggingAnimation`).
    pub hug: bool,
    /// Petting wolves and cats (`petAnimation`).
    pub pet: bool,
    /// Arms back while sprinting (`narutoRunning`).
    pub naruto: bool,
    /// Arms flail while falling (`fallingAnimation`).
    pub falling: bool,
    /// Horse riding arm pose (`enableHorseAnimation`).
    pub horse: bool,
    /// Horse riding leg pose (`enableHorseLegAnimation`); the screen shows it
    /// greyed out, as the port does not pose the legs separately.
    pub horse_leg: bool,
    /// The body turns to the head while eating/drinking/blocking
    /// (`enableRotationLocking`).
    pub rotation_lock: bool,
    /// `RotationLock`: None/Fixed/Smooth.
    pub rotation_mode: RotationLock,
    /// Only lock the rotation in first person.
    pub limit_rotation_fp: bool,
    /// Apply the rotation lock to every player, not just the camera entity.
    pub rotation_everyone: bool,
    /// `BowAnimation`: Vanilla or CustomV1 (the custom draw pose).
    pub bow_mode: BowAnimation,
    /// The custom bow's body turn (`customBowRotationLock`).
    pub custom_bow_rotation: bool,
    /// Crossbow poses clamp their arm angles (`clampCrossbowAnimations`).
    pub clamp_crossbow: bool,
    /// Elytra limbs react to speed (`tweakElytraAnimation`).
    pub tweak_elytra: bool,
    /// Animation smoothing (`enableAnimationSmoothing`).
    pub smoothing: bool,
    /// Legs keep their unsmoothed pose (`disableLegSmoothing`).
    pub disable_leg_smoothing: bool,
    /// `animationSmoothingSpeed` (0.01..1).
    pub smoothing_speed: f32,
    /// `HoldUpModes`.
    pub hold_up_mode: HoldUpMode,
    /// `HoldUpTarget`.
    pub hold_up_target: HoldUpTarget,
    /// `holdUpItemOffset` (-0.5..0.4).
    pub hold_up_item_offset: f32,
    /// `holdUpCameraOffset` (-0.3..0.6).
    pub hold_up_camera_offset: f32,
    /// Only the local player holds items up (`holdUpOnlySelf`).
    pub hold_up_only_self: bool,
    /// No items in the hands while in bed (`dontHoldItemsInBed`).
    pub dont_hold_items_in_bed: bool,
    /// `maxNormalAngle` (0..50).
    pub max_normal_angle: f32,
    /// `maxBlockingAngle` (0..50).
    pub max_blocking_angle: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            enabled: true,
            eat_drink: true,
            ladder: true,
            rotate_to_ladder: true,
            ladder_amplifier: 0.35,
            ladder_arm_height: 1.7,
            ladder_arm_speed: 2.0,
            boat: true,
            item_swap: true,
            burning: true,
            freezing: true,
            crawling: true,
            sleep: true,
            maps: true,
            offhand_hiding: true,
            holding_items: DEFAULT_HOLDING.iter().map(|s| s.to_string()).collect(),
            lantern_items: DEFAULT_LANTERNS.iter().map(|s| s.to_string()).collect(),
            animate_lanterns: true,
            hug: false,
            pet: true,
            naruto: false,
            falling: false,
            horse: true,
            horse_leg: false,
            rotation_lock: true,
            rotation_mode: RotationLock::None,
            limit_rotation_fp: true,
            rotation_everyone: false,
            bow_mode: BowAnimation::Vanilla,
            custom_bow_rotation: false,
            clamp_crossbow: false,
            tweak_elytra: true,
            smoothing: true,
            disable_leg_smoothing: false,
            smoothing_speed: 0.2,
            hold_up_mode: HoldUpMode::Config,
            hold_up_target: HoldUpTarget::Camera,
            hold_up_item_offset: 0.0,
            hold_up_camera_offset: 0.1,
            hold_up_only_self: false,
            dont_hold_items_in_bed: true,
            max_normal_angle: 50.0,
            max_blocking_angle: 15.0,
        }
    }
}

fn is_on(read: &impl Fn(&str) -> Option<String>, key: &str, def: bool) -> bool {
    read(key).map_or(def, |v| v != "false")
}
fn list(read: &impl Fn(&str) -> Option<String>, key: &str, def: &[&str]) -> Vec<String> {
    match read(key) {
        Some(v) => v
            .split(',')
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .collect(),
        None => def.iter().map(|s| s.to_string()).collect(),
    }
}
fn number(read: &impl Fn(&str) -> Option<String>, key: &str, def: f32) -> f32 {
    read(key).and_then(|v| v.parse().ok()).unwrap_or(def)
}
fn round_step(v: f32, min: f32, max: f32, step: f32) -> f32 {
    let v = v.clamp(min, max);
    let steps = ((v - min) / step).round();
    ((min + steps * step).clamp(min, max) * 100.0).round() / 100.0
}

impl Settings {
    /// `false` turns the animation off; anything else (or nothing) keeps it.
    pub fn load(read: impl Fn(&str) -> Option<String>) -> Self {
        let d = Self::default();
        Self {
            enabled: is_on(&read, KEY_ON, d.enabled),
            eat_drink: is_on(&read, KEY_EAT, d.eat_drink),
            ladder: is_on(&read, KEY_LADDER, d.ladder),
            rotate_to_ladder: is_on(&read, KEY_ROTATE_LADDER, d.rotate_to_ladder),
            ladder_amplifier: round_step(
                number(&read, KEY_LADDER_AMP, d.ladder_amplifier),
                0.1,
                0.5,
                0.01,
            ),
            ladder_arm_height: round_step(
                number(&read, KEY_LADDER_HEIGHT, d.ladder_arm_height),
                1.0,
                3.0,
                0.1,
            ),
            ladder_arm_speed: round_step(
                number(&read, KEY_LADDER_SPEED, d.ladder_arm_speed),
                1.0,
                4.0,
                0.1,
            ),
            boat: is_on(&read, KEY_BOAT, d.boat),
            item_swap: is_on(&read, KEY_SWAP, d.item_swap),
            burning: is_on(&read, KEY_BURN, d.burning),
            freezing: is_on(&read, KEY_FREEZE, d.freezing),
            crawling: is_on(&read, KEY_CRAWL, d.crawling),
            sleep: is_on(&read, KEY_SLEEP, d.sleep),
            maps: is_on(&read, KEY_MAPS, d.maps),
            offhand_hiding: is_on(&read, KEY_BOW_FIXES, d.offhand_hiding),
            holding_items: list(&read, KEY_HOLDING, DEFAULT_HOLDING),
            lantern_items: list(&read, KEY_LANTERNS, DEFAULT_LANTERNS),
            animate_lanterns: is_on(&read, KEY_ANIMATE_LANTERNS, d.animate_lanterns),
            hug: is_on(&read, KEY_HUG, d.hug),
            pet: is_on(&read, KEY_PET, d.pet),
            naruto: is_on(&read, KEY_NARUTO, d.naruto),
            falling: is_on(&read, KEY_FALLING, d.falling),
            horse: is_on(&read, KEY_HORSE, d.horse),
            horse_leg: is_on(&read, KEY_HORSE_LEG, d.horse_leg),
            rotation_lock: is_on(&read, KEY_ROTATION, d.rotation_lock),
            rotation_mode: match read(KEY_ROTATION_MODE).as_deref() {
                Some("FIXED") => RotationLock::Fixed,
                Some("SMOOTH") => RotationLock::Smooth,
                _ => RotationLock::None,
            },
            limit_rotation_fp: is_on(&read, KEY_LIMIT_FP, d.limit_rotation_fp),
            rotation_everyone: is_on(&read, KEY_ROTATION_EVERYONE, d.rotation_everyone),
            // the mod stores the enum; an older boolean `true` maps to
            // the custom pose.
            bow_mode: match read(KEY_BOW).as_deref() {
                Some("CUSTOM_V1") | Some("true") => BowAnimation::CustomV1,
                _ => BowAnimation::Vanilla,
            },
            custom_bow_rotation: is_on(&read, KEY_CUSTOM_BOW_ROTATION, d.custom_bow_rotation),
            clamp_crossbow: is_on(&read, KEY_CLAMP, d.clamp_crossbow),
            tweak_elytra: is_on(&read, KEY_ELYTRA, d.tweak_elytra),
            smoothing: is_on(&read, KEY_SMOOTHING, d.smoothing),
            disable_leg_smoothing: is_on(&read, KEY_DISABLE_LEG_SMOOTHING, d.disable_leg_smoothing),
            smoothing_speed: round_step(
                number(&read, KEY_SMOOTHING_SPEED, d.smoothing_speed),
                0.1,
                1.0,
                0.1,
            ),
            hold_up_mode: match read(KEY_HOLD_MODE).as_deref() {
                Some("ALL") => HoldUpMode::All,
                Some("NONE") => HoldUpMode::None,
                Some("CONFIG_INVERTED") => HoldUpMode::ConfigInverted,
                _ => HoldUpMode::Config,
            },
            hold_up_target: match read(KEY_HOLD_TARGET).as_deref() {
                Some("NONE") => HoldUpTarget::None,
                _ => HoldUpTarget::Camera,
            },
            hold_up_item_offset: round_step(
                number(&read, KEY_HOLD_ITEM_OFFSET, d.hold_up_item_offset),
                -0.5,
                0.4,
                0.1,
            ),
            hold_up_camera_offset: round_step(
                number(&read, KEY_HOLD_CAMERA_OFFSET, d.hold_up_camera_offset),
                -0.3,
                0.6,
                0.1,
            ),
            hold_up_only_self: is_on(&read, KEY_HOLD_ONLY_SELF, d.hold_up_only_self),
            dont_hold_items_in_bed: is_on(&read, KEY_DONT_HOLD_BED, d.dont_hold_items_in_bed),
            max_normal_angle: round_step(
                number(&read, KEY_MAX_NORMAL, d.max_normal_angle),
                0.0,
                50.0,
                0.5,
            ),
            max_blocking_angle: round_step(
                number(&read, KEY_MAX_BLOCKING, d.max_blocking_angle),
                0.0,
                50.0,
                0.5,
            ),
        }
    }

    pub fn saved(&self) -> Vec<(String, String)> {
        let f = |on: bool| if on { "true" } else { "false" }.into();
        let n = |v: f32| format!("{v}");
        let holding = self.holding_items.join(",");
        let lantern = self.lantern_items.join(",");
        vec![
            (KEY_ON.into(), f(self.enabled)),
            (KEY_EAT.into(), f(self.eat_drink)),
            (KEY_LADDER.into(), f(self.ladder)),
            (KEY_ROTATE_LADDER.into(), f(self.rotate_to_ladder)),
            (KEY_LADDER_AMP.into(), n(self.ladder_amplifier)),
            (KEY_LADDER_HEIGHT.into(), n(self.ladder_arm_height)),
            (KEY_LADDER_SPEED.into(), n(self.ladder_arm_speed)),
            (KEY_BOAT.into(), f(self.boat)),
            (KEY_SWAP.into(), f(self.item_swap)),
            (KEY_BURN.into(), f(self.burning)),
            (KEY_FREEZE.into(), f(self.freezing)),
            (KEY_CRAWL.into(), f(self.crawling)),
            (KEY_SLEEP.into(), f(self.sleep)),
            (KEY_MAPS.into(), f(self.maps)),
            (KEY_BOW_FIXES.into(), f(self.offhand_hiding)),
            (KEY_HOLDING.into(), holding),
            (KEY_LANTERNS.into(), lantern),
            (KEY_ANIMATE_LANTERNS.into(), f(self.animate_lanterns)),
            (KEY_HUG.into(), f(self.hug)),
            (KEY_PET.into(), f(self.pet)),
            (KEY_NARUTO.into(), f(self.naruto)),
            (KEY_FALLING.into(), f(self.falling)),
            (KEY_HORSE.into(), f(self.horse)),
            (KEY_HORSE_LEG.into(), f(self.horse_leg)),
            (KEY_ROTATION.into(), f(self.rotation_lock)),
            (
                KEY_ROTATION_MODE.into(),
                match self.rotation_mode {
                    RotationLock::None => "NONE",
                    RotationLock::Fixed => "FIXED",
                    RotationLock::Smooth => "SMOOTH",
                }
                .into(),
            ),
            (KEY_LIMIT_FP.into(), f(self.limit_rotation_fp)),
            (KEY_ROTATION_EVERYONE.into(), f(self.rotation_everyone)),
            (
                KEY_BOW.into(),
                match self.bow_mode {
                    BowAnimation::Vanilla => "VANILLA",
                    BowAnimation::CustomV1 => "CUSTOM_V1",
                }
                .into(),
            ),
            (KEY_CUSTOM_BOW_ROTATION.into(), f(self.custom_bow_rotation)),
            (KEY_CLAMP.into(), f(self.clamp_crossbow)),
            (KEY_ELYTRA.into(), f(self.tweak_elytra)),
            (KEY_SMOOTHING.into(), f(self.smoothing)),
            (
                KEY_DISABLE_LEG_SMOOTHING.into(),
                f(self.disable_leg_smoothing),
            ),
            (KEY_SMOOTHING_SPEED.into(), n(self.smoothing_speed)),
            (
                KEY_HOLD_MODE.into(),
                match self.hold_up_mode {
                    HoldUpMode::Config => "CONFIG",
                    HoldUpMode::All => "ALL",
                    HoldUpMode::None => "NONE",
                    HoldUpMode::ConfigInverted => "CONFIG_INVERTED",
                }
                .into(),
            ),
            (
                KEY_HOLD_TARGET.into(),
                match self.hold_up_target {
                    HoldUpTarget::None => "NONE",
                    HoldUpTarget::Camera => "CAMERA",
                }
                .into(),
            ),
            (KEY_HOLD_ITEM_OFFSET.into(), n(self.hold_up_item_offset)),
            (KEY_HOLD_CAMERA_OFFSET.into(), n(self.hold_up_camera_offset)),
            (KEY_HOLD_ONLY_SELF.into(), f(self.hold_up_only_self)),
            (KEY_DONT_HOLD_BED.into(), f(self.dont_hold_items_in_bed)),
            (KEY_MAX_NORMAL.into(), n(self.max_normal_angle)),
            (KEY_MAX_BLOCKING.into(), n(self.max_blocking_angle)),
        ]
    }

    /// The value of the toggle `i` (the same order as `click`).
    pub fn get(&self, i: usize) -> bool {
        match i {
            0 => self.maps,
            1 => self.ladder,
            2 => self.rotate_to_ladder,
            3 => self.crawling,
            4 => self.eat_drink,
            5 => self.boat,
            6 => self.horse,
            7 => self.item_swap,
            8 => self.pet,
            9 => self.freezing,
            10 => self.custom_bow_rotation,
            11 => self.clamp_crossbow,
            12 => self.burning,
            13 => self.smoothing,
            14 => self.disable_leg_smoothing,
            15 => self.limit_rotation_fp,
            16 => self.rotation_everyone,
            17 => self.rotation_lock,
            18 => self.animate_lanterns,
            19 => self.hold_up_only_self,
            20 => self.tweak_elytra,
            21 => self.dont_hold_items_in_bed,
            22 => self.sleep,
            23 => self.offhand_hiding,
            24 => self.naruto,
            25 => self.hug,
            26 => self.falling,
            27 => self.horse_leg,
            _ => false,
        }
    }

    /// Sets the toggle `i` (the same order as `click`).
    pub fn set(&mut self, i: usize, v: bool) {
        match i {
            0 => self.maps = v,
            1 => self.ladder = v,
            2 => self.rotate_to_ladder = v,
            3 => self.crawling = v,
            4 => self.eat_drink = v,
            5 => self.boat = v,
            6 => self.horse = v,
            7 => self.item_swap = v,
            8 => self.pet = v,
            9 => self.freezing = v,
            10 => self.custom_bow_rotation = v,
            11 => self.clamp_crossbow = v,
            12 => self.burning = v,
            13 => self.smoothing = v,
            14 => self.disable_leg_smoothing = v,
            15 => self.limit_rotation_fp = v,
            16 => self.rotation_everyone = v,
            17 => self.rotation_lock = v,
            18 => self.animate_lanterns = v,
            19 => self.hold_up_only_self = v,
            20 => self.tweak_elytra = v,
            21 => self.dont_hold_items_in_bed = v,
            22 => self.sleep = v,
            23 => self.offhand_hiding = v,
            24 => self.naruto = v,
            25 => self.hug = v,
            26 => self.falling = v,
            27 => self.horse_leg = v,
            _ => {}
        }
    }

    /// Flips the toggle `i`.
    pub fn click(&mut self, i: usize) {
        let v = !self.get(i);
        self.set(i, v);
    }

    /// The slider `i`'s current value.
    pub fn slider(&self, i: usize) -> f32 {
        match i {
            0 => self.ladder_amplifier,
            1 => self.ladder_arm_height,
            2 => self.ladder_arm_speed,
            3 => self.smoothing_speed,
            4 => self.max_normal_angle,
            5 => self.max_blocking_angle,
            6 => self.hold_up_item_offset,
            7 => self.hold_up_camera_offset,
            _ => 0.0,
        }
    }

    /// Sets the slider `i`, snapped to its step.
    pub fn set_slider(&mut self, i: usize, v: f32) {
        let (min, max, step) = slider_range(i);
        let v = round_step(v, min, max, step);
        match i {
            0 => self.ladder_amplifier = v,
            1 => self.ladder_arm_height = v,
            2 => self.ladder_arm_speed = v,
            3 => self.smoothing_speed = v,
            4 => self.max_normal_angle = v,
            5 => self.max_blocking_angle = v,
            6 => self.hold_up_item_offset = v,
            7 => self.hold_up_camera_offset = v,
            _ => {}
        }
    }

    /// The choice `i`'s selected index.
    pub fn choice(&self, i: usize) -> usize {
        match i {
            0 => match self.bow_mode {
                BowAnimation::Vanilla => 0,
                BowAnimation::CustomV1 => 1,
            },
            1 => match self.rotation_mode {
                RotationLock::None => 0,
                RotationLock::Fixed => 1,
                RotationLock::Smooth => 2,
            },
            2 => match self.hold_up_mode {
                HoldUpMode::Config => 0,
                HoldUpMode::All => 1,
                HoldUpMode::None => 2,
                HoldUpMode::ConfigInverted => 3,
            },
            3 => match self.hold_up_target {
                HoldUpTarget::None => 0,
                HoldUpTarget::Camera => 1,
            },
            _ => 0,
        }
    }

    /// Selects the choice `i`'s value `n`.
    pub fn set_choice(&mut self, i: usize, n: usize) {
        match (i, n) {
            (0, 0) => self.bow_mode = BowAnimation::Vanilla,
            (0, 1) => self.bow_mode = BowAnimation::CustomV1,
            (1, 0) => self.rotation_mode = RotationLock::None,
            (1, 1) => self.rotation_mode = RotationLock::Fixed,
            (1, 2) => self.rotation_mode = RotationLock::Smooth,
            (2, 0) => self.hold_up_mode = HoldUpMode::Config,
            (2, 1) => self.hold_up_mode = HoldUpMode::All,
            (2, 2) => self.hold_up_mode = HoldUpMode::None,
            (2, 3) => self.hold_up_mode = HoldUpMode::ConfigInverted,
            (3, 0) => self.hold_up_target = HoldUpTarget::None,
            (3, 1) => self.hold_up_target = HoldUpTarget::Camera,
            _ => {}
        }
    }

    /// The next value of the choice `i`.
    pub fn cycle_choice(&mut self, i: usize) {
        let n = (self.choice(i) + 1) % choice_len(i);
        self.set_choice(i, n);
    }

    /// Back to the defaults.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Toggle an item in the holding or lantern list.
    pub fn toggle_item(&mut self, lantern: bool, id: &str) {
        let list = if lantern {
            &mut self.lantern_items
        } else {
            &mut self.holding_items
        };
        if let Some(pos) = list.iter().position(|i| i == id) {
            list.remove(pos);
        } else {
            list.push(id.to_string());
        }
    }

    /// Check whether an item is selected in a given tab.
    pub fn item_on(&self, lantern: bool, id: &str) -> bool {
        let list = if lantern {
            &self.lantern_items
        } else {
            &self.holding_items
        };
        list.iter().any(|i| i == id)
    }

    /// The toggles' labels for the mod's page (`text.bm.enable.*`).
    pub fn labels(&self, lang: &str) -> Vec<String> {
        let (on, off) = if russian(lang) {
            ("Вкл.", "Выкл.")
        } else {
            ("On", "Off")
        };
        (0..TOGGLE_COUNT)
            .map(|i| format!("{}: {}", name(i, lang), if self.get(i) { on } else { off }))
            .collect()
    }
}

/// The slider `i`'s (min, max, step), from the slider specs.
pub fn slider_range(i: usize) -> (f32, f32, f32) {
    match i {
        0 => (0.1, 0.5, 0.01),
        1 => (1.0, 3.0, 0.1),
        2 => (1.0, 4.0, 0.1),
        3 => (0.1, 1.0, 0.1),
        4 => (0.0, 50.0, 0.5),
        5 => (0.0, 50.0, 0.5),
        6 => (-0.5, 0.4, 0.1),
        7 => (-0.3, 0.6, 0.1),
        _ => (0.0, 1.0, 0.1),
    }
}

/// The number of values of the choice `i`.
pub fn choice_len(i: usize) -> usize {
    match i {
        0 => 2,
        1 => 3,
        2 => 4,
        3 => 2,
        _ => 1,
    }
}

/// A slider value rounded to two decimals: no
/// trailing zeros, comma in Russian.
pub fn number_text(v: f32, ru: bool) -> String {
    let mut s = format!("{v:.2}");
    while s.ends_with('0') {
        s.pop();
    }
    if s.ends_with('.') {
        s.pop();
    }
    if s == "-0" {
        s = "0".into();
    }
    if ru { s.replace('.', ",") } else { s }
}

/// The settings screen's strings: our `data/lang/<code>.json` files, compiled
/// in. A language we don't ship, or a key a file lacks, falls back to en_us.
pub fn russian(lang: &str) -> bool {
    lang.starts_with("ru")
}

/// Each language's file, byte for byte.
const LANG_FILES: &[(&str, &[u8])] = &[
    (
        "de_at",
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/bodymotion/lang/de_at.json")),
    ),
    (
        "de_ch",
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/bodymotion/lang/de_ch.json")),
    ),
    (
        "de_de",
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/bodymotion/lang/de_de.json")),
    ),
    (
        "en_us",
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/bodymotion/lang/en_us.json")),
    ),
    (
        "fr_fr",
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/bodymotion/lang/fr_fr.json")),
    ),
    (
        "pl_pl",
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/bodymotion/lang/pl_pl.json")),
    ),
    (
        "pt_br",
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/bodymotion/lang/pt_br.json")),
    ),
    (
        "pt_pt",
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/bodymotion/lang/pt_pt.json")),
    ),
    (
        "ru_ru",
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/bodymotion/lang/ru_ru.json")),
    ),
    (
        "sv_se",
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/bodymotion/lang/sv_se.json")),
    ),
    (
        "tr_tr",
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/bodymotion/lang/tr_tr.json")),
    ),
    (
        "uk_ua",
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/bodymotion/lang/uk_ua.json")),
    ),
    (
        "zh_cn",
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/bodymotion/lang/zh_cn.json")),
    ),
    (
        "zh_tw",
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/bodymotion/lang/zh_tw.json")),
    ),
];

thread_local! {
    /// Each language's parsed table, parsed once per run.
    static LANG_TABLES: std::cell::RefCell<
        std::collections::HashMap<&'static str, std::collections::HashMap<String, String>>,
    > = std::cell::RefCell::new(std::collections::HashMap::new());
}

/// The mod's translated string `key` for `lang`: its own file, else en_us.
/// Every file we ship is complete, and en_us is the net for languages we
/// don't ship at all.
pub fn lang_text(lang: &str, key: &str) -> Option<String> {
    let code = LANG_FILES
        .iter()
        .find(|(l, _)| lang.eq_ignore_ascii_case(l))
        .map(|(l, _)| *l)
        .unwrap_or("en_us");
    LANG_TABLES.with(|t| {
        let mut tables = t.borrow_mut();
        let mut get = |code: &'static str| {
            tables
                .entry(code)
                .or_insert_with(|| {
                    let bytes = LANG_FILES
                        .iter()
                        .find(|(l, _)| *l == code)
                        .map(|(_, b)| *b)
                        .unwrap_or(b"{}");
                    serde_json::from_slice(bytes).unwrap_or_default()
                })
                .get(key)
                .filter(|s| !s.is_empty())
                .cloned()
        };
        get(code).or_else(|| if code == "en_us" { None } else { get("en_us") })
    })
}

/// Each toggle's translation key (the same order as `get`/`click`).
const TOGGLE_KEYS: [&str; 28] = [
    "text.bm.enable.inworldmaprendering",
    "text.bm.enable.ladderanimation",
    "text.bm.enable.rotatetoladder",
    "text.bm.enable.crawling",
    "text.bm.enable.eatdrinkanimation",
    "text.bm.enable.rowboatanimation",
    "text.bm.enable.horseanimation",
    "text.bm.enable.itemSwapAnimation",
    "text.bm.enable.petAnimation",
    "text.bm.enable.freezingAnimation",
    "text.bm.enable.customBowRotationLock",
    "text.bm.enable.clampCrossbowAnimations",
    "text.bm.enable.burningAnimation",
    "text.bm.enable.animationsmoothing",
    "text.bm.disableLegSmoothing",
    "text.bm.enable.limitRotationLockToFP",
    "text.bm.enable.applyRotationLockToEveryone",
    "text.bm.enable.rotationlocking",
    "text.bm.animateLanterns",
    "text.bm.enable.holdUpOnlySelf",
    "text.bm.enable.tweakElytraAnimation",
    "text.bm.enable.dontholditemsinbed",
    "text.bm.enable.freezearmsinbed",
    "text.bm.enable.offhandhiding",
    "text.bm.enable.narutoRunning",
    "text.bm.enable.huggingAnimation",
    "text.bm.enable.fallingAnimation",
    "text.bm.enable.enableHorseLegAnimation",
];

/// Each slider's translation key (the same order as `slider`).
const SLIDER_KEYS: [&str; 8] = [
    "text.bm.ladderAnimationAmplifier",
    "text.bm.ladderAnimationArmHeight",
    "text.bm.ladderAnimationArmSpeed",
    "text.bm.smoothingSpeed",
    "text.bm.maxNormalAngle",
    "text.bm.maxBlockingAngle",
    "text.bm.holdUpItemOffset",
    "text.bm.holdUpCameraOffset",
];

/// Each choice's translation key (the same order as `choice`).
const CHOICE_KEYS: [&str; 4] = [
    "text.bm.enable.bowAnimation",
    "text.bm.rotationlock",
    "text.bm.holdUpItemsMode",
    "text.bm.holdUpTarget",
];

/// The enum constant name per choice value (the mod's enum variants).
const CHOICE_VALUES: &[&[&str]] = &[
    &["VANILLA", "CUSTOM_V1"],
    &["NONE", "FIXED", "SMOOTH"],
    &["CONFIG", "ALL", "NONE", "CONFIG_INVERTED"],
    &["NONE", "CAMERA"],
];

/// The headings' translation keys (`text.bm.line.*`). the mod's
/// "Unfinished/Deprecated" heading is gone: its falling animation moved into
/// Fun, finished.
const HEADING_KEYS: [&str; 7] = [
    "text.bm.line.animations",
    "text.bm.line.smoothing",
    "text.bm.line.rotation",
    "text.bm.line.rotationAngle",
    "text.bm.line.holdup",
    "text.bm.line.fixes",
    "text.bm.line.fun",
];

/// The tabs' translation keys (`text.bm.tab.*`).
const TAB_KEYS: [&str; 3] = [
    "text.bm.tab.settings",
    "text.bm.tab.holdup",
    "text.bm.tab.lantern",
];

/// The screen's title: the mod's own name, not a translated key.
pub fn title(lang: &str) -> &'static str {
    if russian(lang) {
        "Настройки Body Motion"
    } else {
        "Body Motion Settings"
    }
}

/// `text.bm.tab.settings` for the Settings tab.
pub fn tab_label(lang: &str) -> String {
    tab_label_by_tab(0, lang)
}

/// `text.bm.tab.settings` for Settings tab, `text.bm.tab.holdup` for Hold Up
/// Items tab, `text.bm.tab.lantern` for Lanterns tab.
pub fn tab_label_by_tab(tab: usize, lang: &str) -> String {
    let key = TAB_KEYS.get(tab).copied().unwrap_or("");
    lang_text(lang, key).unwrap_or_default()
}

/// The headings of the Settings tab (`text.bm.line.*`).
pub fn heading(i: usize, lang: &str) -> String {
    let key = HEADING_KEYS.get(i).copied().unwrap_or("");
    lang_text(lang, key).unwrap_or_default()
}

/// The label of the greyed-out row `i` (a row whose animation is not ported).
pub fn disabled(i: usize, lang: &str) -> String {
    match i {
        0 => lang_text(lang, TOGGLE_KEYS[27]).unwrap_or_default(),
        _ => String::new(),
    }
}

/// The settings' descriptions (`key.tooltip` strings), for the screen's hover
/// boxes.
pub fn toggle_tooltip(i: usize, lang: &str) -> String {
    lang_text(
        lang,
        &format!("{}.tooltip", TOGGLE_KEYS.get(i).copied().unwrap_or("")),
    )
    .unwrap_or_default()
}

pub fn slider_tooltip(i: usize, lang: &str) -> String {
    lang_text(
        lang,
        &format!("{}.tooltip", SLIDER_KEYS.get(i).copied().unwrap_or("")),
    )
    .unwrap_or_default()
}

pub fn choice_tooltip(i: usize, lang: &str) -> String {
    lang_text(
        lang,
        &format!("{}.tooltip", CHOICE_KEYS.get(i).copied().unwrap_or("")),
    )
    .unwrap_or_default()
}

/// The toggle `i`'s name (the same order as `get`/`click`).
pub fn name(i: usize, lang: &str) -> String {
    let key = TOGGLE_KEYS.get(i).copied().unwrap_or("");
    lang_text(lang, key).unwrap_or_default()
}

/// The slider `i`'s name (the same order as `slider`).
pub fn slider_name(i: usize, lang: &str) -> String {
    let key = SLIDER_KEYS.get(i).copied().unwrap_or("");
    lang_text(lang, key).unwrap_or_default()
}

/// A slider's whole button label: `Name: value`.
pub fn slider_text(i: usize, v: f32, lang: &str) -> String {
    format!(
        "{}: {}",
        slider_name(i, lang),
        number_text(v, russian(lang))
    )
}

/// The choice `i`'s name (the same order as `choice`).
pub fn choice_name(i: usize, lang: &str) -> String {
    let key = CHOICE_KEYS.get(i).copied().unwrap_or("");
    lang_text(lang, key).unwrap_or_default()
}

/// The choice `i`'s value `n` as its button shows it (the
/// `text.bm.<key>.<ENUM>` strings).
pub fn choice_value(i: usize, n: usize, lang: &str) -> String {
    let key = CHOICE_KEYS.get(i).copied().unwrap_or("");
    let value = CHOICE_VALUES
        .get(i)
        .and_then(|v| v.get(n))
        .copied()
        .unwrap_or("");
    lang_text(lang, &format!("{}.{}", key, value)).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn on_by_default() {
        let s = Settings::load(|_| None);
        assert!(s.enabled);
        assert!(s.eat_drink);
        assert!(s.sleep);
        assert!(s.maps);
        assert!(s.offhand_hiding);
        assert_eq!(s.labels("en_us").len(), TOGGLE_COUNT);
        assert_eq!(s.rotation_mode, RotationLock::None);
        assert_eq!(s.hold_up_mode, HoldUpMode::Config);
        assert_eq!(s.hold_up_target, HoldUpTarget::Camera);
        assert_eq!(s.bow_mode, BowAnimation::Vanilla);
    }

    #[test]
    fn false_turns_each_off_and_back() {
        let mut s = Settings::load(|_| Some("false".into()));
        assert!(!s.enabled);
        assert!(!s.eat_drink);
        s.click(4);
        assert!(s.eat_drink);
        s.click(4);
        assert!(!s.eat_drink);
    }

    #[test]
    fn labels_follow_language_and_state() {
        let s = Settings::default();
        assert_eq!(s.labels("en_us")[4], "Eating And Drinking: On");
        assert_eq!(s.labels("ru_ru")[3], "Ползание: Вкл.");
        let mut off = Settings::default();
        off.click(3);
        assert_eq!(off.labels("ru_ru")[3], "Ползание: Выкл.");
        // Unknown language falls back to English.
        assert_eq!(s.labels("xx_xx")[12], "Burning: On");
        assert_eq!(s.labels("en_us")[0], "Maps In The World: On");
        assert_eq!(
            s.labels("en_us")[23],
            "Hide Offhand While Drawing A Bow: On"
        );
    }

    #[test]
    fn saved_round_trips() {
        let s = Settings {
            enabled: false,
            crawling: false,
            maps: false,
            rotation_mode: RotationLock::Smooth,
            hold_up_mode: HoldUpMode::All,
            bow_mode: BowAnimation::CustomV1,
            ladder_arm_height: 2.3,
            smoothing_speed: 0.5,
            ..Settings::default()
        };
        let saved = s.saved();
        let loaded = Settings::load(|k| saved.iter().find(|(a, _)| a == k).map(|(_, v)| v.clone()));
        assert!(!loaded.enabled);
        assert!(!loaded.crawling);
        assert!(!loaded.maps);
        assert_eq!(loaded.rotation_mode, RotationLock::Smooth);
        assert_eq!(loaded.hold_up_mode, HoldUpMode::All);
        assert_eq!(loaded.bow_mode, BowAnimation::CustomV1);
        assert_eq!(loaded.ladder_arm_height, 2.3);
        assert_eq!(loaded.smoothing_speed, 0.5);
        assert!(loaded.burning);
    }

    #[test]
    fn get_set_reset_round_trip() {
        let mut s = Settings::default();
        assert!(s.get(0));
        s.set(0, false);
        assert!(!s.get(0));
        s.click(0);
        assert!(s.get(0));
        s.reset();
        assert!(s.get(0));
        assert!(s.get(23));
        assert!(!s.get(99));
        s.set(99, false);
    }

    #[test]
    fn sliders_snap_and_format() {
        let mut s = Settings::default();
        assert_eq!(s.slider(0), 0.35);
        s.set_slider(0, 0.333);
        assert_eq!(s.slider(0), 0.33);
        s.set_slider(0, 9.0);
        assert_eq!(s.slider(0), 0.5);
        s.set_slider(6, -0.5);
        assert_eq!(s.slider(6), -0.5);
        assert_eq!(
            slider_text(0, 0.35, "ru_ru"),
            "Размах рук на лестнице: 0,35"
        );
        assert_eq!(slider_text(4, 50.0, "en_us"), "Normal Angle: 50");
        assert_eq!(slider_text(3, 0.2, "ru_ru"), "Сила сглаживания: 0,2");
        assert_eq!(slider_text(6, 0.0, "ru_ru"), "Смещение при показе: 0");
    }

    #[test]
    fn choices_cycle() {
        let mut s = Settings::default();
        assert_eq!(s.choice(0), 0);
        s.cycle_choice(0);
        assert_eq!(s.bow_mode, BowAnimation::CustomV1);
        s.cycle_choice(0);
        assert_eq!(s.bow_mode, BowAnimation::Vanilla);
        assert_eq!(choice_len(1), 3);
        s.cycle_choice(1);
        assert_eq!(s.rotation_mode, RotationLock::Fixed);
        s.cycle_choice(2);
        assert_eq!(s.hold_up_mode, HoldUpMode::All);
        s.cycle_choice(3);
        assert_eq!(s.hold_up_target, HoldUpTarget::None);
    }

    #[test]
    fn rows_match_the_screen_order() {
        assert_eq!(row(0), Row::Heading(0));
        assert_eq!(row(1), Row::Toggle(0));
        assert_eq!(row(4), Row::Slider(0));
        assert_eq!(row(11), Row::Toggle(27));
        assert_eq!(row(15), Row::Choice(0));
        assert_eq!(row(26), Row::Choice(1));
        assert_eq!(row(31), Row::Heading(3));
        assert_eq!(row(35), Row::Heading(4));
        assert_eq!(row(36), Row::Choice(2));
        assert_eq!(row(39), Row::Choice(3));
        assert_eq!(row(43), Row::Heading(5));
        assert_eq!(row(49), Row::Heading(6));
        assert_eq!(row(52), Row::Toggle(26));
        assert_eq!(row(ROW_COUNT), Row::Blank);
    }

    #[test]
    fn screen_texts() {
        assert_eq!(title("en_us"), "Body Motion Settings");
        assert_eq!(title("ru_ru"), "Настройки Body Motion");
        assert_eq!(tab_label("ru_ru"), "Настройки");
        assert_eq!(tab_label_by_tab(1, "ru_ru"), "Показ предметов");
        assert_eq!(heading(0, "en_us"), "Animations");
        assert_eq!(heading(5, "ru_ru"), "Исправления");
        assert_eq!(heading(3, "ru_ru"), "Углы наклона");
        assert_eq!(tab_label_by_tab(2, "ru_ru"), "Фонари");
        assert_eq!(disabled(0, "ru_ru"), "Позы ног на лошади");
        assert_eq!(name(8, "ru_ru"), "Поглаживание");
        assert_eq!(name(10, "ru_ru"), "Наклон тела с луком");
        assert_eq!(name(18, "ru_ru"), "Качающиеся фонари");
        assert_eq!(name(26, "en_us"), "Falling");
        assert_eq!(name(26, "ru_ru"), "Падение");
        assert_eq!(choice_value(2, 3, "ru_ru"), "Всё, кроме выбранных");
        assert!(!toggle_tooltip(0, "ru_ru").is_empty());
        assert!(!toggle_tooltip(18, "ru_ru").is_empty());
        assert!(russian("ru_ru"));
        assert!(!russian("en_us"));
        assert_eq!(name(23, "en_us"), "Hide Offhand While Drawing A Bow");
        assert_eq!(choice_name(1, "ru_ru"), "Поворот тела");
        assert_eq!(choice_value(1, 0, "ru_ru"), "Выкл.");
    }

    #[test]
    fn lang_files_parse_and_en_us_has_all_tooltips() {
        for (lang, bytes) in LANG_FILES {
            let _: serde_json::Value = serde_json::from_slice(bytes).expect(lang);
            let text = std::str::from_utf8(bytes).expect(lang);
            assert!(
                !text.contains("text.nea"),
                "{lang} still carries text.nea keys"
            );
        }
        let mut tips = 0;
        for keys in [
            TOGGLE_KEYS.as_slice(),
            SLIDER_KEYS.as_slice(),
            CHOICE_KEYS.as_slice(),
        ] {
            for k in keys {
                let tip = lang_text("en_us", &format!("{k}.tooltip")).expect(k);
                assert!(!tip.is_empty(), "{k}");
                tips += 1;
            }
        }
        assert_eq!(
            tips,
            TOGGLE_KEYS.len() + SLIDER_KEYS.len() + CHOICE_KEYS.len()
        );
    }

    /// Every language we ship must resolve a non-empty string for every name,
    /// tooltip, choice value, heading and tab on the screen, and no value may
    /// carry a "[WIP] ..."-style marker.
    #[test]
    fn every_language_is_fully_translated() {
        let langs = [
            "de_at", "de_ch", "de_de", "en_us", "fr_fr", "pl_pl", "pt_br", "pt_pt", "ru_ru",
            "sv_se", "tr_tr", "uk_ua", "zh_cn", "zh_tw",
        ];
        let mut keys: Vec<String> = TOGGLE_KEYS
            .iter()
            .chain(&SLIDER_KEYS)
            .chain(&CHOICE_KEYS)
            .flat_map(|k| [k.to_string(), format!("{k}.tooltip")])
            .collect();
        for (values, key) in CHOICE_VALUES.iter().zip(&CHOICE_KEYS) {
            for v in *values {
                keys.push(format!("{key}.{v}"));
            }
        }
        keys.extend(HEADING_KEYS.iter().map(|k| k.to_string()));
        keys.extend(TAB_KEYS.iter().map(|k| k.to_string()));
        for lang in langs {
            for key in &keys {
                let text = lang_text(lang, key).unwrap_or_else(|| panic!("{lang} misses {key}"));
                assert!(!text.trim().is_empty(), "{lang} has empty {key}");
                assert!(
                    !text.starts_with('[') && !text.starts_with('【'),
                    "{lang} {key} carries a WIP marker: {text}"
                );
            }
        }
    }
}
