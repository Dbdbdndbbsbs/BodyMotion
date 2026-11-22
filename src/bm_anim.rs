// animation half, inserted into `game/client_ambient.rs`: the settings
// and the ported animations. The frame hook captures each player's state (`BmState`)
// and hands it to the model as its `BodyPose`; the game applies it right after the
// vanilla `humanoid` pose.
use crate::mobs::anim::{Anim, State};
use crate::mobs::Mob;
use horizon_core::geometry::trig::Mth;
use horizon_core::use_animation::UseAnimation;
use std::cell::{Cell, RefCell};
use std::sync::Arc;

/// `RotationLock` (enum). Defined here rather than in the
/// settings module: this file is shared with the library crate, which has no
/// `game` module.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RotationLock {
    None,
    Fixed,
    Smooth,
}

/// `HoldUpModes` (enum).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HoldUpMode {
    Config,
    All,
    None,
    ConfigInverted,
}

/// `HoldUpTarget` (enum).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HoldUpTarget {
    None,
    Camera,
}

/// `BowAnimation` (enum).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BowAnimation {
    Vanilla,
    CustomV1,
}

/// settings, switched each frame from settings.
#[derive(Clone, Copy, Debug)]
pub struct BmSettings {
    pub master: bool,
    pub eat_drink: bool,
    pub ladder: bool,
    pub rotate_to_ladder: bool,
    pub ladder_amplifier: f32,
    pub ladder_arm_height: f32,
    pub ladder_arm_speed: f32,
    pub boat: bool,
    pub item_swap: bool,
    pub burning: bool,
    pub freezing: bool,
    pub crawling: bool,
    pub sleep: bool,
    pub maps: bool,
    pub offhand_hiding: bool,
    pub hug: bool,
    pub pet: bool,
    pub naruto: bool,
    pub falling: bool,
    pub horse: bool,
    pub horse_leg: bool,
    pub rotation_lock: bool,
    pub rotation_mode: RotationLock,
    pub limit_rotation_fp: bool,
    pub rotation_everyone: bool,
    pub max_normal_angle: f32,
    pub max_blocking_angle: f32,
    pub bow_mode: BowAnimation,
    pub custom_bow_rotation: bool,
    pub clamp_crossbow: bool,
    pub tweak_elytra: bool,
    pub smoothing: bool,
    pub disable_leg_smoothing: bool,
    pub smoothing_speed: f32,
    pub hold_up_mode: HoldUpMode,
    pub hold_up_target: HoldUpTarget,
    pub hold_up_item_offset: f32,
    pub hold_up_camera_offset: f32,
    pub hold_up_only_self: bool,
}
impl BmSettings {
    pub const DEFAULT: Self = Self {
        master: true,
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
        max_normal_angle: 50.0,
        max_blocking_angle: 15.0,
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
    };
}
impl Default for BmSettings {
    fn default() -> Self {
        Self::DEFAULT
    }
}
thread_local! {
    static BM: Cell<BmSettings> = const { Cell::new(BmSettings::DEFAULT) };
    /// FreezingAnimation: the local player's box touches powder snow
    /// (computed by the frame hook).
    static POWDER_SNOW: Cell<bool> = const { Cell::new(false) };
    /// BurningAnimation: the local player's remaining fire ticks (the
    /// model's own `fire_ticks` is only synced for remote players).
    static FIRE_TICKS: Cell<i32> = const { Cell::new(0) };
    /// LadderAnimation: the local player is on a climbable block that is
    /// a ladder or trapdoor (computed by the frame hook).
    static CLIMBING: Cell<bool> = const { Cell::new(false) };
    /// LadderAnimation: the body yaw (radians) that faces the ladder,
    /// read from the ladder block's `facing` by the frame hook.
    static LADDER_FACE: Cell<f32> = const { Cell::new(0.0) };
    /// LadderAnimation: the climbable block has a `facing` (ladder or
    /// trapdoor), so the body is locked and the head is clamped to it.
    static LADDER_LOCK: Cell<bool> = const { Cell::new(false) };
    /// BurningAnimation: the local player is on fire without fire
    /// resistance (computed by the frame hook).
    static BURNING: Cell<bool> = const { Cell::new(false) };
    /// FreezingAnimation: the local player is freezing in powder snow
    /// (computed by the frame hook).
    static FREEZING: Cell<bool> = const { Cell::new(false) };
    /// ItemSwapAnimation: the model age when the hands last traded items.
    static SWAP_AT: Cell<f32> = const { Cell::new(0.0) };
    /// BoatAnimation: riding a boat, and each paddle's state and time.
    static BOAT: Cell<(bool, [bool; 2], [f32; 2])> =
        const { Cell::new((false, [false; 2], [0.0; 2])) };
    /// The hands the frame hook saw last, to spot a trade.
    static HANDS: RefCell<(Option<Arc<str>>, Option<Arc<str>>)> =
        const { RefCell::new((None, None)) };
    /// The local player's flags the synced extras cannot say (the frame hook
    /// computes them from PlayerMotion): sprinting, Riptide's spin, dying, and
    /// the FallingAnimation's trigger, vertical speed and fall-start age.
    static LOCAL_SPRINT: Cell<bool> = const { Cell::new(false) };
    static LOCAL_SPIN: Cell<bool> = const { Cell::new(false) };
    static LOCAL_DYING: Cell<bool> = const { Cell::new(false) };
    static LOCAL_FALLING: Cell<bool> = const { Cell::new(false) };
    static LOCAL_FALL_SPEED: Cell<f32> = const { Cell::new(0.0) };
    static LOCAL_FALL_START: Cell<f32> = const { Cell::new(-1.0) };
    static LOCAL_FALL_DURATION: Cell<f32> = const { Cell::new(0.0) };
    /// The tweak-elytra speed scale, from the local player's velocity.
    static LOCAL_K: Cell<f32> = const { Cell::new(1.0) };
    /// The local player's off-hand crossbow, from the frame hook: loaded, and
    /// its charge time. The game tracks both for the main hand only
    /// (`Perspective::update_model`), and a remote player's off-hand stack data
    /// is not synced, so remote players keep the unloaded defaults.
    static OFF_CHARGED: Cell<bool> = const { Cell::new(false) };
    static OFF_CHARGE_MAX: Cell<f32> = const { Cell::new(25.0) };
    /// The social poses' raycast, the frame hook's (the local player crouches
    /// with another player (hug) or a wolf/cat (pet) in view).
    static LOCAL_HUG: Cell<bool> = const { Cell::new(false) };
    static LOCAL_PET: Cell<bool> = const { Cell::new(false) };
    /// The pet target offset from the player's model origin, in model space.
    static LOCAL_PET_TARGET: RefCell<Option<[f32; 3]>> =
        const { RefCell::new(None) };
    /// The mob ids riding a saddled horse-family mount, from the frame hook.
    static HORSE_RIDERS: RefCell<crate::fast_hash::FxHashSet<u32>> =
        RefCell::new(crate::fast_hash::FxHashSet::default());
    /// The mount's walk animation position (`AbstractHorse.walkAnimation.position`),
    /// from the frame hook, for the arms' swing.
    static HORSE_WALK: Cell<f32> = const { Cell::new(0.0) };
    /// The mount is up on its hind legs (`AbstractHorse.isStanding`), from the
    /// frame hook, for the legs' pose.
    static HORSE_STANDING: Cell<bool> = const { Cell::new(false) };
    /// The rider's pump strength (0..1) from the mount's horizontal speed, for
    /// the legs' and arms' swing; 0 when the horse is idle or mid-jump.
    static HORSE_PUMP: Cell<f32> = const { Cell::new(0.0) };
    /// The mount's stand (rear) animation 0..1 (`AbstractHorse.standAnim`):
    /// the horse's body pitches up by 45°, and the rider's legs follow it.
    static HORSE_STAND: Cell<f32> = const { Cell::new(0.0) };
    /// Remote players' FallingAnimation data, per mob id: the last y and the
    /// falling speed (the per-mob fall record).
    static FALL_MAP: RefCell<crate::fast_hash::FxHashMap<u32, (f64, f32, u32, u32)>> =
        RefCell::new(crate::fast_hash::FxHashMap::default());
    /// The Hold Up Items list (the Hold Up Items tab), from the frame hook.
    static HOLDING_LIST: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
    /// The last tick a remote player's body was turned to the head.
    static ROT_TICK: RefCell<crate::fast_hash::FxHashMap<u32, u32>> =
        RefCell::new(crate::fast_hash::FxHashMap::default());
    /// The last tick the local player's body was turned to the head.
    static LOCAL_TICK: Cell<u32> = const { Cell::new(u32::MAX) };
    /// Animation smoothing: per mob, the previous rendered and the current
    /// target arm/leg rotations (left arm, right arm, left leg, right leg;
    /// x, y, z), plus the partial tick they were captured at.
    static SMOOTH: RefCell<crate::fast_hash::FxHashMap<u32, SmoothEntry>> =
        RefCell::new(crate::fast_hash::FxHashMap::default());
}

/// Per-limb smoothing state: `prev` is the last rendered
/// rotation, `target` the rotation the next render interpolates to.
#[derive(Clone, Copy, Debug)]
struct SmoothEntry {
    prev: [[f32; 3]; 4],
    target: [[f32; 3]; 4],
    render: [[f32; 3]; 4],
    partial: f32,
    started: bool,
    /// The last tick and partial a render happened at, and how many renders in
    /// a row they have not changed: the smoothing interpolation is stuck then
    /// (a paused world, an open inventory, a frozen server tick), and the
    /// limbs are re-seeded to the pose they should be in instead.
    last_age: f32,
    stall: u32,
}
pub fn set_bm(s: BmSettings) {
    BM.with(|c| c.set(s));
}
pub fn set_powder_snow(on: bool) {
    POWDER_SNOW.with(|c| c.set(on));
}
pub fn set_fire_ticks(ticks: i32) {
    FIRE_TICKS.with(|c| c.set(ticks));
}
pub fn set_climbing(on: bool) {
    CLIMBING.with(|c| c.set(on));
}
pub fn set_ladder_face(yaw: f32) {
    LADDER_FACE.with(|c| c.set(yaw));
}
pub fn set_ladder_lock(on: bool) {
    LADDER_LOCK.with(|c| c.set(on));
}
pub fn set_burning(on: bool) {
    BURNING.with(|c| c.set(on));
}
pub fn set_freezing(on: bool) {
    FREEZING.with(|c| c.set(on));
}
pub fn set_local_sprinting(on: bool) {
    LOCAL_SPRINT.with(|c| c.set(on));
}
pub fn set_local_spin(on: bool) {
    LOCAL_SPIN.with(|c| c.set(on));
}
pub fn set_local_dying(on: bool) {
    LOCAL_DYING.with(|c| c.set(on));
}
/// The FallingAnimation's trigger (`fallDistance > 4`) and speed for the
/// local player, plus the age the fall started at (for the buildup).
pub fn set_local_falling(on: bool, speed: f32, age: f32) {
    let dur = if on {
        let start = LOCAL_FALL_START.with(|c| {
            let s = c.get();
            if s < 0.0 {
                c.set(age);
                age
            } else {
                s
            }
        });
        (age - start).max(0.0)
    } else {
        LOCAL_FALL_START.with(|c| c.set(-1.0));
        0.0
    };
    LOCAL_FALLING.with(|c| c.set(on));
    LOCAL_FALL_SPEED.with(|c| c.set(speed));
    LOCAL_FALL_DURATION.with(|c| c.set(dur));
}
pub fn set_local_k(k: f32) {
    LOCAL_K.with(|c| c.set(k));
}
/// The local player's off-hand crossbow: loaded, and its charge time (ticks).
pub fn set_offhand_crossbow(charged: bool, max: f32) {
    OFF_CHARGED.with(|c| c.set(charged));
    OFF_CHARGE_MAX.with(|c| c.set(max));
}
pub fn set_hug(on: bool) {
    LOCAL_HUG.with(|c| c.set(on));
}
pub fn set_pet(on: bool, target: Option<[f32; 3]>) {
    LOCAL_PET.with(|c| c.set(on));
    LOCAL_PET_TARGET.with(|c| *c.borrow_mut() = if on { target } else { None });
}
pub fn set_horse_riders(ids: crate::fast_hash::FxHashSet<u32>) {
    HORSE_RIDERS.with(|c| *c.borrow_mut() = ids);
}
pub fn set_horse_walk(walk: f32) {
    HORSE_WALK.with(|c| c.set(walk));
}
pub fn set_horse_standing(on: bool) {
    HORSE_STANDING.with(|c| c.set(on));
}
/// The ridden horse's horizontal speed this frame (blocks/tick): the pump of
/// the rider's legs and arms follows it, so it stops when the horse is idle
/// or mid-jump.
pub fn set_horse_pump(pump: f32) {
    HORSE_PUMP.with(|c| c.set(pump));
}
pub fn set_horse_stand(stand: f32) {
    HORSE_STAND.with(|c| c.set(stand));
}
pub fn horse_stand() -> f32 {
    HORSE_STAND.with(|c| c.get())
}
pub fn horse_pump() -> f32 {
    HORSE_PUMP.with(|c| c.get())
}
pub fn set_holding_list(list: Vec<String>) {
    HOLDING_LIST.with(|c| *c.borrow_mut() = list);
}
/// Forget per-mob state for entities that left the world this frame.
pub fn prune_stale(alive: &crate::fast_hash::FxHashSet<u32>) {
    FALL_MAP.with(|c| c.borrow_mut().retain(|id, _| alive.contains(id)));
    ROT_TICK.with(|c| c.borrow_mut().retain(|id, _| alive.contains(id)));
    SMOOTH.with(|c| c.borrow_mut().retain(|id, _| alive.contains(id)));
}

/// Forget every per-mob state (left the world).
pub fn reset_animation_state() {
    SWAP_AT.with(|c| c.set(0.0));
    BOAT.with(|c| c.set((false, [false; 2], [0.0; 2])));
    HANDS.with(|h| *h.borrow_mut() = (None, None));
    FALL_MAP.with(|c| c.borrow_mut().clear());
    HORSE_RIDERS.with(|c| c.borrow_mut().clear());
    HORSE_WALK.with(|c| c.set(0.0));
    HORSE_STANDING.with(|c| c.set(false));
    HOLDING_LIST.with(|c| c.borrow_mut().clear());
    ROT_TICK.with(|c| c.borrow_mut().clear());
    LOCAL_TICK.with(|c| c.set(u32::MAX));
    LOCAL_FALLING.with(|c| c.set(false));
    LOCAL_FALL_SPEED.with(|c| c.set(0.0));
    LOCAL_FALL_START.with(|c| c.set(-1.0));
    LOCAL_FALL_DURATION.with(|c| c.set(0.0));
    OFF_CHARGED.with(|c| c.set(false));
    OFF_CHARGE_MAX.with(|c| c.set(25.0));
    SMOOTH.with(|c| c.borrow_mut().clear());
}
/// BoatAnimation: the boat the local player rides, and each paddle's
/// state and time (`AbstractBoat.rowingTimeLeft/Right`).
pub fn update_boat(boat: Option<([bool; 2], [f32; 2])>) {
    BOAT.with(|c| c.set(match boat {
        Some((paddling, paddles)) => (true, paddling, paddles),
        None => (false, [false; 2], [0.0; 2]),
    }));
}
/// `AbstractBoat.getRowingTime`: 0 when the oar is not paddling, else the
/// paddle time interpolated over the render frame.
fn rowing_time(paddling: bool, paddle: f32, partial: f32) -> f32 {
    if paddling {
        paddle - std::f32::consts::FRAC_PI_8 + partial * std::f32::consts::FRAC_PI_8
    } else {
        0.0
    }
}
/// ItemSwapAnimation: notice the hands trading their items, and remember
/// the age it happened at.
pub fn update_hands(age: f32, main: Option<Arc<str>>, off: Option<Arc<str>>) {
    HANDS.with(|h| {
        let (last_main, last_off) = h.borrow().clone();
        if last_main != last_off
            && main == last_off
            && off == last_main
            && (main.is_some() || off.is_some())
        {
            SWAP_AT.with(|c| c.set(age));
        }
        *h.borrow_mut() = (main, off);
    });
}
fn swap_at() -> f32 {
    SWAP_AT.with(|c| c.get())
}
fn boat() -> (bool, [bool; 2], [f32; 2]) {
    BOAT.with(|c| c.get())
}
fn bm() -> BmSettings {
    BM.with(|c| c.get())
}
fn bm_on(flag: bool) -> bool {
    let n = bm();
    n.master && flag
}
fn in_powder_snow() -> bool {
    POWDER_SNOW.with(|c| c.get())
}
fn fire_ticks() -> i32 {
    FIRE_TICKS.with(|c| c.get())
}

/// Everything the animations need from the mob and the render state, captured
/// before the vanilla pose runs.
#[derive(Clone, Copy, Debug)]
pub struct BmState {
    pub id: u32,
    pub is_player: bool,
    pub is_local: bool,
    pub partial: f32,
    pub pos: f32,
    pub y: f32,
    pub age: f32,
    pub pitch: f32,
    pub swing: f32,
    pub using_item: i32,
    /// The vanilla use animation (`UseAnimation`) of the used item: se
    /// eat/drink pose runs only for real food and drink, never for a spear or
    /// any other item in use.
    pub use_animation: UseAnimation,
    pub using_off_hand: bool,
    pub swap_at: f32,
    pub climbing: bool,
    pub ladder_face: f32,
    pub ladder_lock: bool,
    pub head_yaw: f32,
    pub boat: bool,
    pub boat_left: f32,
    pub boat_right: f32,
    pub swim: f32,
    pub swim_in_water: bool,
    pub main_arm_left: bool,
    pub swing_right_arm: Option<bool>,
    pub main_special: bool,
    pub off_special: bool,
    pub main_map: bool,
    pub off_map: bool,
    pub off_empty: bool,
    pub spear: bool,
    pub sleep_pose: bool,
    pub fire_resistance: bool,
    pub burning: bool,
    pub freezing: bool,
    pub sprinting: bool,
    pub crouching: bool,
    pub passenger: bool,
    pub fall_flying: bool,
    pub dying: bool,
    pub spin_attack: bool,
    pub blocking: bool,
    pub two_handed: bool,
    pub single_handed: bool,
    pub crossbow_charge: bool,
    pub crossbow_hold: bool,
    pub using_bow: bool,
    /// The main hand holds the two-handed pose: a bow or crossbow being used,
    /// or a loaded crossbow held up. The off-hand pose must not override it.
    pub main_two_handed: bool,
    /// The off hand holds a bow or a crossbow, and the crossbow is loaded.
    pub off_bow: bool,
    pub off_crossbow: bool,
    pub off_charged: bool,
    /// The off-hand crossbow's charge time (ticks); 25 when unknown.
    pub off_charge_max: f32,
    pub hug: bool,
    pub pet: bool,
    /// The pet target offset from the player's model origin, in model space.
    pub pet_target: Option<[f32; 3]>,
    pub horse: bool,
    pub horse_walk: f32,
    pub horse_standing: bool,
    pub falling: bool,
    pub fall_speed: f32,
    /// Ticks since the free fall started; the tumble pose builds up with it.
    pub fall_duration: f32,
    pub fly_k: f32,
    pub main_hold: bool,
    pub off_hold: bool,
    pub main_empty: bool,
    pub rotation_target: bool,
}
impl BmState {
    pub const NONE: Self = Self {
        id: u32::MAX,
        is_player: false,
        is_local: false,
        partial: 0.0,
        pos: 0.0,
        y: 0.0,
        age: 0.0,
        pitch: 0.0,
        swing: 0.0,
        using_item: 0,
        use_animation: UseAnimation::None,
        using_off_hand: false,
        swap_at: 0.0,
        climbing: false,
        ladder_face: 0.0,
        ladder_lock: false,
        head_yaw: 0.0,
        boat: false,
        boat_left: 0.0,
        boat_right: 0.0,
        swim: 0.0,
        swim_in_water: false,
        main_arm_left: false,
        swing_right_arm: None,
        main_special: false,
        off_special: false,
        main_map: false,
        off_map: false,
        off_empty: true,
        spear: false,
        sleep_pose: false,
        fire_resistance: false,
        burning: false,
        freezing: false,
        sprinting: false,
        crouching: false,
        passenger: false,
        fall_flying: false,
        dying: false,
        spin_attack: false,
        blocking: false,
        two_handed: false,
        single_handed: false,
        crossbow_charge: false,
        crossbow_hold: false,
        using_bow: false,
        main_two_handed: false,
        off_bow: false,
        off_crossbow: false,
        off_charged: false,
        off_charge_max: 25.0,
        hug: false,
        pet: false,
        pet_target: None,
        horse: false,
        horse_walk: 0.0,
        horse_standing: false,
        falling: false,
        fall_speed: 0.0,
        fall_duration: 0.0,
        fly_k: 1.0,
        main_hold: false,
        off_hold: false,
        main_empty: true,
        rotation_target: false,
    };
    pub fn capture(m: &Mob, s: &State) -> Self {
        let spear = m
            .extra
            .main_hand_id()
            .and_then(crate::spear::Kinetic::of)
            .is_some()
            || m
                .extra
                .off_hand_id()
                .and_then(crate::spear::Kinetic::of)
                .is_some();
        let special = |item: Option<&str>| {
            matches!(
                item,
                Some("minecraft:bow")
                    | Some("minecraft:trident")
                    | Some("minecraft:spyglass")
                    | Some("minecraft:goat_horn")
                    | Some("minecraft:crossbow")
                    | Some("minecraft:brush")
            ) || (m.extra.use_animation == UseAnimation::Block)
        };
        // Only a filled map gets the map-holding pose. An empty map stays a
        // plain item and is held like one. The explorer maps (treasure,
        // ocean, woodland) are filled maps too: separate item ids, the same
        // `map_id` component, the same pose.
        let is_map = |item: Option<&str>| {
            matches!(
                item,
                Some("minecraft:filled_map")
                    | Some("minecraft:buried_treasure_map")
                    | Some("minecraft:ocean_explorer_map")
                    | Some("minecraft:woodland_explorer_map")
            )
        };
        let (boat, paddling, paddles) = boat();
        let is_player = matches!(m.kind.anim, Anim::Humanoid { zombie_arms: false });
        let is_local = m.id == 0;
        // The local player's climbing/freezing/burning state comes from the
        // frame hook, which reads PlayerMotion (synced fields are unavailable
        // for the local model). Remote players and other mobs use their own
        // synced fields.
        let climbing = if is_local && is_player {
            CLIMBING.with(|c| c.get())
        } else {
            m.climbing
        };
        let burning = if is_local && is_player {
            BURNING.with(|c| c.get())
        } else {
            m.is_on_fire() && !m.has_effect("minecraft:fire_resistance")
        };
        let freezing = if is_local && is_player {
            FREEZING.with(|c| c.get())
        } else {
            m.ticks_frozen > 0
        };
        // The vanilla poses' guards (`isEnabled() == true`, empty apply): they
        // block the lower-priority animations. The local player's dying and
        // spin come from the frame hook (`death_time` is not synced for it).
        let dying = if is_local && is_player {
            LOCAL_DYING.with(|c| c.get())
        } else {
            m.death_time > 0
        };
        let spin_attack = if is_local && is_player {
            LOCAL_SPIN.with(|c| c.get())
        } else {
            m.extra.auto_spin_attack
        };
        let sprinting = if is_local && is_player {
            LOCAL_SPRINT.with(|c| c.get())
        } else {
            m.extra.sprinting
        };
        let using_hand: Option<&str> = if m.extra.using_off_hand {
            m.extra.off_hand_id()
        } else {
            m.extra.main_hand_id()
        };
        let using_main = m.extra.using_item > 0 && !m.extra.using_off_hand;
        let using_off = m.extra.using_item > 0 && m.extra.using_off_hand;
        let using_bow = m.extra.using_item > 0 && using_hand == Some("minecraft:bow");
        let using_crossbow = m.extra.using_item > 0 && using_hand == Some("minecraft:crossbow");
        // The game tracks the loaded crossbow in the main hand only
        // (`Perspective::update_model`); the frame hook feeds the local
        // player's off hand, and a remote off hand has no stack data.
        let off_bow = m.extra.off_hand_id() == Some("minecraft:bow");
        let off_crossbow = m.extra.off_hand_id() == Some("minecraft:crossbow");
        let off_charged = off_crossbow && is_local && OFF_CHARGED.with(|c| c.get());
        let main_charged =
            m.extra.main_hand_id() == Some("minecraft:crossbow") && m.extra.charged;
        let charged_using = if m.extra.using_off_hand {
            off_charged
        } else {
            m.extra.charged
        };
        let crossbow_charge = using_crossbow && !charged_using;
        let crossbow_hold = (main_charged || off_charged) && s.swing == 0.0;
        // VanillaTwoHandedAnimation: the vanilla bow/crossbow poses. The game
        // poses the main hand only; `apply_offhand_two_handed` mirrors the
        // pose onto the off hand.
        let main_two_handed =
            (using_main && (using_bow || using_crossbow)) || (main_charged && s.swing == 0.0);
        let off_two_handed =
            (using_off && (using_bow || using_crossbow)) || (off_charged && s.swing == 0.0);
        let two_handed = main_two_handed || off_two_handed;
        // VanillaSingleHandedAnimation: spyglass, trident, spear.
        let single_handed = spear
            || (m.extra.using_item > 0
                && matches!(using_hand, Some("minecraft:spyglass") | Some("minecraft:trident")));
        // Falling: the local player's trigger and speed come from the frame
        // hook; remote players estimate the speed from the per-tick drop.
        let mut falling = false;
        let mut fall_speed = 0.0;
        let mut fall_duration = 0.0;
        if is_player {
            if is_local {
                falling = LOCAL_FALLING.with(|c| c.get());
                fall_speed = LOCAL_FALL_SPEED.with(|c| c.get());
                fall_duration = LOCAL_FALL_DURATION.with(|c| c.get());
            } else if !m.extra.fall_flying
                && !m.on_ground
                && !climbing
                && !m.extra.swim.in_water
            {
                fall_duration = FALL_MAP.with(|c| {
                    let map = &mut *c.borrow_mut();
                    let data = map.entry(m.id).or_insert((m.position.y as f64, 0.0, m.age, m.age));
                    if data.2 == m.age {
                        // Rerender in the same tick: keep the speed.
                        fall_speed = data.1;
                    } else {
                        // The drop since the last sample, per tick; a sample can
                        // span several ticks on a laggy server.
                        let dropped = (data.0 - m.position.y as f64) as f32
                            / m.age.saturating_sub(data.2).max(1) as f32;
                        data.0 = m.position.y as f64;
                        data.1 = (dropped * 0.14).clamp(0.0, 1.0);
                        data.2 = m.age;
                        fall_speed = data.1;
                    }
                    falling = fall_speed > 0.08;
                    m.age.saturating_sub(data.3) as f32
                });
            }
        }
        // ElytraAnimation's speed scale, from the velocity (tweak).
        let fly_k = if is_local && is_player {
            LOCAL_K.with(|c| c.get())
        } else {
            let v = m.velocity;
            (((v.x * v.x + v.y * v.y + v.z * v.z) as f32 / 0.2).powi(3)).max(1.0)
        };
        // HorseAnimation: riding a saddled horse-family mount (the frame hook
        // resolves the mount's kind).
        let horse = HORSE_RIDERS.with(|c| c.borrow().contains(&m.id));
        let horse_walk = HORSE_WALK.with(|c| c.get());
        let horse_standing = HORSE_STANDING.with(|c| c.get());
        // The social poses' raycast is the frame hook's (local player only).
        let hug = LOCAL_HUG.with(|c| c.get());
        let pet = LOCAL_PET.with(|c| c.get());
        let pet_target = if is_local && is_player && pet {
            LOCAL_PET_TARGET.with(|c| {
                c.borrow().map(|target| {
                    // The model renders at `alpha` between the previous and the
                    // current tick, so the body yaw it sees is that
                    // interpolation; `target_in_model` then converts the world
                    // offset into the model's space.
                    let alpha = f32::from(s.partial);
                    let body = crate::game::native_bm_pet::lerp_degrees(
                        m.previous_body_yaw,
                        m.body_yaw,
                        alpha,
                    );
                    crate::game::native_bm_pet::target_in_model(target, body)
                })
            })
        } else {
            None
        };
        // LookAtItemAnimation: the hand's item is on the Hold Up Items list.
        let holds = |item: Option<&str>| {
            item.is_some_and(|i| {
                HOLDING_LIST.with(|c| c.borrow().iter().any(|l| l == i))
            })
        };
        let main_hold = holds(m.extra.main_hand_id());
        let off_hold = holds(m.extra.off_hand_id());
        // ActionRotationLockAnimation: eating, drinking or blocking (the
        // vanilla two-handed and single-handed poses keep the vanilla body).
        let rotation_target = (m.extra.use_animation == UseAnimation::Block)
            || (m.extra.using_item > 0
                && !matches!(
                    using_hand,
                    Some("minecraft:bow")
                        | Some("minecraft:crossbow")
                        | Some("minecraft:spyglass")
                        | Some("minecraft:goat_horn")
                        | Some("minecraft:brush")
                        | Some("minecraft:trident")
                )
                && !spear);
        Self {
            id: m.id,
            is_player,
            is_local,
            partial: s.partial,
            pos: s.pos,
            y: m.position.y as f32,
            age: s.age,
            pitch: s.x_rot,
            swing: s.swing,
            using_item: m.extra.using_item,
            use_animation: m.extra.use_animation,
            using_off_hand: m.extra.using_off_hand,
            swap_at: swap_at(),
            climbing,
            ladder_face: if climbing {
                LADDER_FACE.with(|c| c.get())
            } else {
                0.0
            },
            // Remote players have no local ladder block; keep their vanilla
            // body and head instead of using the local player's lock.
            ladder_lock: climbing && is_local && is_player && LADDER_LOCK.with(|c| c.get()),
            // `State.y_rot` is the head relative to the (locked) body, so it
            // is already the sideways look that the ladder animation clamps.
            head_yaw: s.y_rot.to_radians(),
            boat,
            boat_left: rowing_time(paddling[0], paddles[0], s.partial),
            boat_right: rowing_time(paddling[1], paddles[1], s.partial),
            swim: m.extra.swim.amount(s.partial),
            swim_in_water: m.extra.swim.in_water,
            main_arm_left: m.main_arm_left(),
            swing_right_arm: m.extra.swing_right_arm,
            main_special: special(m.extra.main_hand_id()),
            off_special: special(m.extra.off_hand_id()),
            main_map: is_map(m.extra.main_hand_id()),
            off_map: is_map(m.extra.off_hand_id()),
            off_empty: m.extra.off_hand_id().is_none(),
            spear,
            sleep_pose: crate::mobs::villager::sleep_pose(m).is_some(),
            fire_resistance: m.has_effect("minecraft:fire_resistance"),
            burning,
            freezing,
            sprinting,
            crouching: m.extra.crouching,
            passenger: m.extra.passenger,
            fall_flying: m.extra.fall_flying,
            dying,
            spin_attack,
            blocking: (m.extra.use_animation == UseAnimation::Block),
            two_handed,
            single_handed,
            crossbow_charge,
            crossbow_hold,
            using_bow,
            main_two_handed,
            off_bow,
            off_crossbow,
            off_charged,
            off_charge_max: if is_local {
                OFF_CHARGE_MAX.with(|c| c.get())
            } else {
                25.0
            },
            hug,
            pet,
            pet_target,
            horse,
            horse_walk,
            horse_standing,
            falling,
            fall_speed,
            fall_duration,
            fly_k,
            main_hold,
            off_hold,
            main_empty: m.extra.main_hand_id().is_none(),
            rotation_target,
        }
    }
}
/// The six body parts a `BodyPose` poses, under the accessors the animations use
/// (`get`/`xrot`/`yrot`/`zrot`, as `mobs::anim::Poses` has them).
pub struct Poses {
    parts: [[f32; 9]; 6],
    /// A part the model lacks: writes land here and are dropped.
    dummy: [f32; 9],
}

fn part_index(name: &str) -> Option<usize> {
    crate::mobs::anim::BODY_PARTS.iter().position(|p| *p == name)
}

impl Poses {
    fn get(&mut self, name: &str) -> &mut [f32; 9] {
        match part_index(name) {
            Some(i) => &mut self.parts[i],
            None => &mut self.dummy,
        }
    }
    fn xrot(&mut self, name: &str, v: f32) {
        self.get(name)[3] = v;
    }
    fn yrot(&mut self, name: &str, v: f32) {
        self.get(name)[4] = v;
    }
    fn zrot(&mut self, name: &str, v: f32) {
        self.get(name)[5] = v;
    }
}

/// A player model's Body Motion pose: the state the frame hook captured, laid
/// over the vanilla animation by the game.
#[derive(Debug)]
pub struct BmPose(pub BmState);

impl crate::mobs::anim::BodyPose for BmPose {
    fn pose(&self, _age: f32, _at_rest: &[[f32; 9]; 6], poses: &mut [[f32; 9]; 6]) -> bool {
        if !self.0.is_player {
            return false;
        }
        let mut p = Poses {
            parts: *poses,
            dummy: [0.0; 9],
        };
        apply_all(&mut p, &self.0);
        *poses = p.parts;
        true
    }
}

/// Capture `m`'s state at `alpha` and hand it to the model as its pose. Horizon's
/// own emote poses are left alone; own is dropped when it is off.
pub fn set_emote(m: &mut Mob, alpha: f32) {
    let is_player = matches!(m.kind.anim, Anim::Humanoid { zombie_arms: false });
    let ours = m
        .emote
        .as_ref()
        .is_some_and(|p| (&**p as &dyn std::any::Any).is::<BmPose>());
    if m.emote.is_some() && !ours {
        return;
    }
    if !is_player || !bm().master {
        if ours {
            m.emote = None;
        }
        return;
    }
    let st = BmState::capture(m, &crate::mobs::anim::state(m, alpha));
    m.emote = Some(Arc::new(BmPose(st)) as Arc<dyn crate::mobs::anim::BodyPose>);
}

fn rotlerp_rad(a: f32, from: f32, to: f32) -> f32 {
    let mut diff = to - from;
    while diff < -std::f32::consts::PI {
        diff += std::f32::consts::TAU;
    }
    while diff >= std::f32::consts::PI {
        diff -= std::f32::consts::TAU;
    }
    from + a * diff
}

/// `AnimationUtil.wrapDegrees` (radians in, radians out, [-pi, pi)).
fn wrap_deg(a: f32) -> f32 {
    let mut a = (a + std::f32::consts::PI) % std::f32::consts::TAU;
    if a < 0.0 {
        a += std::f32::consts::TAU;
    }
    a - std::f32::consts::PI
}

/// Apply every ported animation to a player model, after the vanilla pose.
pub fn apply_all(p: &mut Poses, st: &BmState) {
    if !st.is_player {
        return;
    }
    // The 3600-priority guards first: dying and Riptide's spin claim the whole
    // model and block every other animation.
    if st.dying || st.spin_attack {
        return;
    }
    // `maxNormalAngle` / `maxBlockingAngle`: vanilla's head limit relative to
    // the body (the mod mixin on `getMaxHeadRotationRelativeToBody`).
    let angles = bm();
    let limit = if st.blocking {
        angles.max_blocking_angle
    } else {
        angles.max_normal_angle
    };
    if angles.master && limit < 90.0 {
        let limit = limit.to_radians();
        p.yrot("head", st.head_yaw.clamp(-limit, limit));
    }
    // Lowest priority first: a later animation overwrites the arms it owns,
    // as priority order does (Map/Hold Up 300, Crawling 350,
    // Falling/Burning/Freezing 400, Naruto 500, Ladder 1000, Boat/Horse 1500,
    // Hug/Pet 2100, Eat/Drink 2500, Elytra 3000, guards 3100, Clamp/Custom
    // Bow 3200, Item Swap 3500). Eat/Drink is applied after Elytra and Item
    // Swap on purpose: the using arm must still reach the mouth while gliding,
    // swimming or trading items.
    // The vanilla poses' guards (3100) block everything below 3100.
    let guard = st.blocking || st.two_handed || st.single_handed;
    if st.fall_flying {
        // ElytraAnimation (3000) claims the model during the glide; the
        // tweak re-poses the limbs (or does nothing when off).
        apply_elytra(p, st);
    } else if !guard && !st.swim_in_water {
        apply_map_holding(p, st);
        apply_hold_up(p, st);
        apply_crawling(p, st);
        apply_falling(p, st);
        apply_burning(p, st);
        apply_freezing(p, st);
        apply_naruto(p, st);
        apply_ladder(p, st);
        apply_boat(p, st);
        apply_horse(p, st);
        apply_hug(p, st);
        apply_pet(p, st);
    }
    apply_offhand_two_handed(p, st);
    apply_clamp_crossbow(p, st);
    apply_custom_bow(p, st);
    apply_item_swap(p, st);
    // Eat/Drink wins the using arm over the glide, the swim pose and a fresh
    // item swap; special items and sleep still keep their own poses.
    apply_eat_drink(p, st);
    apply_sleep(p, st);
    apply_smoothing(p, st);
}

/// MapHoldingAnimation (priority 300): the arms hold the map in front,
/// both hands when the off hand is empty, otherwise only the hand with the
/// map. pitch-driven formulas.
fn apply_map_holding(p: &mut Poses, st: &BmState) {
    if !bm_on(bm().maps) {
        return;
    }
    let both = st.main_map && st.off_empty;
    let both_maps = st.main_map && st.off_map;
    let main_only = st.main_map && !st.off_empty && !st.off_map;
    let off_only = !st.main_map && st.off_map;
    if !(both || both_maps || main_only || off_only) {
        return;
    }
    // `EntityUtil.getXRot` is the head pitch in degrees.
    let t = (90.0 - st.pitch) / 180.0;
    let lerp = |a: f32, from: f32, to: f32| from + a * (to - from);
    // pitch-driven formulas swing the arms (and the maps in
    // them) as the head tilts, so two maps converge and diverge when looking
    // up and down. Pin the pose to level gaze (t = 0.5): the map stays fixed
    // in the hands no matter the head angle.
    let (x, y) = if both {
        (-lerp(0.5, 0.7, 0.9), lerp(0.5, -0.3, -0.2))
    } else {
        (-lerp(0.5, 0.5, 1.5), 0.0)
    };
    let arm = |p: &mut Poses, right: bool| {
        let name = if right { "right_arm" } else { "left_arm" };
        p.xrot(name, x);
        p.yrot(name, if right { y } else { -y });
        p.zrot(name, if right { 0.3 } else { -0.3 });
    };
    if both {
        arm(p, true);
        arm(p, false);
    } else if both_maps {
        // A map in each hand: the mod only poses the main arm, leaving
        // the off-hand map floating. Pose both arms with the one-handed
        // formula so each hand holds its own map.
        arm(p, true);
        arm(p, false);
    } else if main_only {
        arm(p, !st.main_arm_left);
    } else {
        // The off arm is the opposite of the main arm.
        arm(p, st.main_arm_left);
    }
}

/// LookAtItemAnimation (priority 300): the arms hold their hand's item up
/// to the face (the CONFIG/ALL/CONFIG_INVERTED modes) or level to the camera
/// (`HoldUpTarget`), with offsets. An arm busy with a swing
/// keeps its vanilla pose.
fn apply_hold_up(p: &mut Poses, st: &BmState) {
    let n = bm();
    if !n.master || st.spear || n.hold_up_mode == HoldUpMode::None {
        return;
    }
    if n.hold_up_only_self && !st.is_local {
        return;
    }
    let wants = |listed: bool, empty: bool| match n.hold_up_mode {
        HoldUpMode::Config => listed,
        HoldUpMode::All => !empty,
        HoldUpMode::ConfigInverted => !listed && !empty,
        HoldUpMode::None => false,
    };
    // The right hand is the main one unless the player is left-handed.
    let right = if st.main_arm_left {
        (st.off_hold, st.off_empty)
    } else {
        (st.main_hold, st.main_empty)
    };
    let left = if st.main_arm_left {
        (st.main_hold, st.main_empty)
    } else {
        (st.off_hold, st.off_empty)
    };
    let right = wants(right.0, right.1);
    let left = wants(left.0, left.1);
    let swinging = |left: bool| {
        st.swing > 0.0
            && match st.swing_right_arm {
                Some(right) => left == !right,
                None => left == st.main_arm_left,
            }
    };
    let right = right && !swinging(false);
    let left = left && !swinging(true);
    if !right && !left {
        return;
    }
    let arm = |p: &mut Poses, left: bool| {
        let name = if left { "left_arm" } else { "right_arm" };
        match n.hold_up_target {
            HoldUpTarget::None => {
                // `EntityUtil.getXRot` is the head pitch in degrees.
                let t = -1.0 * (st.pitch - 90.0) / 180.0;
                let x = -n.hold_up_item_offset - (1.0 + t * (1.5 - 1.0));
                p.xrot(name, x);
                p.yrot(name, if left { 0.2 } else { -0.2 });
                p.zrot(name, if left { -0.3 } else { 0.3 });
            }
            HoldUpTarget::Camera => {
                let invert = if left { -1.0 } else { 1.0 };
                let x = (-std::f32::consts::FRAC_PI_2 + st.pitch.to_radians()).clamp(-2.5, 0.0);
                let y = (n.hold_up_camera_offset + st.head_yaw * invert)
                    .clamp(-0.2, n.hold_up_camera_offset.max(0.2));
                p.xrot(name, x);
                p.yrot(name, y);
                p.zrot(name, if left { -0.1 } else { 0.1 });
            }
        }
    };
    if right && left {
        arm(p, false);
        arm(p, true);
    } else if right {
        arm(p, false);
    } else {
        arm(p, true);
    }
}

/// EatDrinkAnimation (priority 2500): the using arm lifts to the mouth
/// and chews. The classic port formula.
fn apply_eat_drink(p: &mut Poses, st: &BmState) {
    // Only real food and drink use this pose; any other item in use (a spear,
    // a shield, ...) keeps its vanilla arm.
    if !bm_on(bm().eat_drink)
        || st.using_item <= 0
        || !matches!(st.use_animation, UseAnimation::Eat | UseAnimation::Drink)
    {
        return;
    }
    let using_main = !st.using_off_hand;
    let using_off = st.using_off_hand;
    let (arm, using) = if using_main && !st.main_special {
        ("right_arm", true)
    } else if using_off && !st.off_special {
        ("left_arm", true)
    } else {
        return;
    };
    if !using {
        return;
    }
    let t = st.using_item as f32 + st.partial;
    let rise = (t / 5.0).clamp(0.0, 1.0);
    let smooth = rise * rise * (3.0 - 2.0 * rise);
    let phase = (t * 0.125).fract();
    let lift = (phase * std::f32::consts::TAU).msin().abs();
    let chew = lift * smooth;
    let low = -0.85;
    let mouth = -1.55;
    let right = arm == "right_arm";
    p.xrot(arm, low + (mouth - low) * smooth - chew * 0.20);
    p.yrot(arm, if right { -0.60 * smooth } else { 0.60 * smooth });
    p.zrot(
        arm,
        if right {
            0.15 * smooth + chew * 0.25
        } else {
            -(0.15 * smooth + chew * 0.25)
        },
    );
}

/// Ladder climb pose: reaching up hand over hand, knees tucked onto the rungs.
/// The sway runs on a sine of the climb phase; left and right limbs mirror.
fn apply_ladder(p: &mut Poses, st: &BmState) {
    let n = bm();
    // The Passenger guard (3500): riding anything but a boat, the vanilla
    // pose keeps the body, so the ladder must not take it.
    if !n.master || !n.ladder || !st.climbing || (st.passenger && !st.boat) {
        return;
    }
    let sway = (st.y * n.ladder_arm_speed * 2.0).sin() * n.ladder_amplifier;
    let base = n.ladder_arm_height;
    // Right arm pulls up while the left reaches down, then they swap.
    let r = sway * 0.5;
    p.xrot("right_arm", -base - r);
    p.yrot("right_arm", -0.12);
    p.zrot("right_arm", 0.22);
    p.xrot("left_arm", -base + r);
    p.yrot("left_arm", 0.12);
    p.zrot("left_arm", -0.22);
    // Both knees stay lifted; the hips sway slightly with the arms.
    let leg_r = sway * 0.35;
    p.xrot("right_leg", -0.95 - leg_r);
    p.yrot("right_leg", -0.12);
    p.zrot("right_leg", 0.22);
    p.xrot("left_leg", -0.95 + leg_r);
    p.yrot("left_leg", 0.12);
    p.zrot("left_leg", -0.22);
    // The body's world yaw is locked to the ladder by the frame hook. The
    // head follows the camera relative to that body, clamped to +-90 degrees
    // so a backwards climb turns the head sideways instead of spinning it.
    if st.ladder_lock {
        p.yrot(
            "head",
            st.head_yaw
                .clamp(-std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2),
        );
    }
}

/// BoatAnimation (priority 1000): each oar sweeps with its own rowing
/// time; idle holds the arms still.
fn apply_boat(p: &mut Poses, st: &BmState) {
    if !bm_on(bm().boat) || !st.boat {
        return;
    }
    let row = |t: f32| -1.1 - t.msin() * 0.3;
    p.xrot("right_arm", row(st.boat_right));
    p.yrot("right_arm", 0.2);
    p.zrot("right_arm", 0.3);
    p.xrot("left_arm", row(st.boat_left));
    p.yrot("left_arm", -0.2);
    p.zrot("left_arm", -0.3);
}

/// HorseAnimation (priority 1500): riding a saddled horse-family mount,
/// the arms reach for the reins, and with `enableHorseLegAnimation` the legs
/// spread out around the horse's flanks. The seat is assumed the first
/// (index 0).
fn apply_horse(p: &mut Poses, st: &BmState) {
    let n = bm();
    if !bm_on(n.horse) || !st.horse {
        return;
    }
    // The pump follows the mount's horizontal speed (0 when idle or mid-jump),
    // and the phase is the mount's walk position: legs and arms move together,
    // in the horse's step pace.
    let pump = if st.horse_standing { 0.0 } else { horse_pump() };
    let phase = st.horse_walk * 0.35;
    // Big amplitude at the mod pace: the limbs travel far, not fast.
    let swing = phase.msin() * pump * 0.7;
    // The torso keeps its straight seat (no lean).
    // Sitting in the saddle: the rider stays centered on the mount (no slide).
    let swinging = |left: bool| {
        st.swing > 0.0
            && match st.swing_right_arm {
                Some(right) => left == !right,
                None => left == st.main_arm_left,
            }
    };
    if !swinging(false) {
        p.xrot("right_arm", -0.9 - horse_stand() * 0.6 - swing * 0.3);
        p.yrot("right_arm", -0.1);
        p.zrot("right_arm", 0.1);
    }
    if !swinging(true) {
        p.xrot("left_arm", -0.9 - horse_stand() * 0.6 - swing * 0.3);
        p.yrot("left_arm", 0.1);
        p.zrot("left_arm", -0.1);
    }
    // The legs claim the full body. The thighs lie forward over the horse's
    // back, close to the body and clear of it (not sunk into the flanks). A
    // rearing horse pitches its body up by 45°; the legs and the arms rise
    // with it (`horse_stand`), so they keep clear of each other in the jump.
    // The legs keep their own small sway.
    if n.horse_leg {
        // The legs sway on their own, barely and slowly: their own slower
        // phase and a small amplitude, not tied to the arms' swing.
        let leg_swing = (st.horse_walk * 0.15 + 1.5).msin() * pump * 0.05;
        let x = -1.5 - horse_stand() * 0.7 + leg_swing;
        p.xrot("right_leg", x);
        p.yrot("right_leg", 0.3);
        p.zrot("right_leg", 0.12);
        p.xrot("left_leg", x);
        p.yrot("left_leg", -0.3);
        p.zrot("left_leg", -0.12);
    }
}

/// Hug pose: crouching face to face with another crouching player, the arms
/// reach out (the local player's raycast).
fn apply_hug(p: &mut Poses, st: &BmState) {
    if !bm_on(bm().hug) || !st.hug {
        return;
    }
    p.xrot("right_arm", -1.35);
    p.yrot("right_arm", -0.25);
    p.zrot("right_arm", 0.4);
    p.xrot("left_arm", -0.9);
    p.yrot("left_arm", 0.25);
    p.zrot("left_arm", -0.4);
}

/// The smoothed aim of the petting hand (yaw, pitch in the model's frame),
/// shared across frames so the hand glides instead of snapping.
thread_local! {
    static PET_AIM: RefCell<[f32; 2]> = RefCell::new([0.0, -1.2]);
}

/// Petting pose: the main hand reaches toward the animal's chest and keeps a
/// soft stroking sway; the arm eases to the target instead of snapping.
fn apply_pet(p: &mut Poses, st: &BmState) {
    if !bm_on(bm().pet) || !st.pet {
        return;
    }
    let right = !st.main_arm_left;
    let name = if right { "right_arm" } else { "left_arm" };
    if let Some([x, y, z]) = st.pet_target {
        let [yaw_target, pitch_target] =
            crate::game::native_bm_pet::aim_angles([x, y, z]);
        let [yaw, pitch] = PET_AIM.with(|c| {
            let mut c = c.borrow_mut();
            let k = 0.12;
            let yaw = c[0] + (yaw_target - c[0]) * k;
            let pitch = c[1] + (pitch_target - c[1]) * k;
            *c = [yaw, pitch];
            [yaw, pitch]
        });
        p.yrot(name, yaw);
        p.xrot(name, -1.05 - pitch * 0.7);
    } else {
        PET_AIM.with(|c| *c.borrow_mut() = [0.0, -1.2]);
        // A gentle forward lift tied to the camera pitch (degrees).
        let t = ((st.pitch + 90.0) / 180.0).clamp(0.0, 1.0);
        p.xrot(name, -(0.9 + t * 0.6));
        p.yrot(name, if right { -0.45 } else { 0.45 });
    }
    let wave = (st.age * 0.7).msin() * 0.15 + (st.age * 1.3).msin() * 0.05;
    p.zrot(name, if right { 0.25 + wave } else { -(0.25 + wave) });
}

/// Gliding pose: the limbs lie back like wing surfaces, with a slow rocking
/// that flattens as speed grows.
fn apply_elytra(p: &mut Poses, st: &BmState) {
    if !st.fall_flying {
        return;
    }
    if !bm().tweak_elytra {
        // The vanilla guard: the pose claims the model, nothing to add.
        return;
    }
    let k = st.fly_k.max(0.001);
    let out = (0.16 / k).clamp(0.08, 0.22);
    let phase = st.age * 0.55;
    let swing = 0.45 / k;
    p.xrot("left_arm", phase.mcos() * swing);
    p.zrot("left_arm", -out);
    p.xrot("right_arm", (phase + std::f32::consts::PI).mcos() * swing);
    p.zrot("right_arm", out);
    p.xrot("left_leg", (phase + std::f32::consts::PI).mcos() * 0.65 / k);
    p.zrot("left_leg", -out * 0.8);
    p.xrot("right_leg", phase.mcos() * 0.65 / k);
    p.zrot("right_leg", out * 0.8);
}

/// `AnimationUtils.animateCrossbowHold` (private in the game's `mobs::anim`).
fn crossbow_hold(p: &mut Poses, hx: f32, hy: f32, in_right: bool) {
    let (hold, shoot) = if in_right {
        ("right_arm", "left_arm")
    } else {
        ("left_arm", "right_arm")
    };
    p.yrot(hold, if in_right { -0.3 } else { 0.3 } + hy);
    p.yrot(shoot, if in_right { 0.6 } else { -0.6 } + hy);
    p.xrot(hold, -std::f32::consts::PI / 2.0 + hx + 0.1);
    p.xrot(shoot, -1.5 + hx);
}

/// `AnimationUtils.animateCrossbowCharge` (private in the game's `mobs::anim`).
fn crossbow_charge(p: &mut Poses, max: f32, ticks: f32, in_right: bool) {
    let (hold, pull) = if in_right {
        ("right_arm", "left_arm")
    } else {
        ("left_arm", "right_arm")
    };
    p.yrot(hold, if in_right { -0.8 } else { 0.8 });
    p.xrot(hold, -0.970_796_35);
    p.xrot(pull, -0.970_796_35);
    let use_ticks = ticks.clamp(0.0, max);
    let alpha = use_ticks / max;
    let lerp = |a: f32, p0: f32, p1: f32| p0 + a * (p1 - p0);
    p.yrot(
        pull,
        lerp(alpha, 0.4, 0.85) * if in_right { 1.0 } else { -1.0 },
    );
    let x = lerp(alpha, p.get(pull)[3], -std::f32::consts::PI / 2.0);
    p.xrot(pull, x);
}

/// `AnimationUtils.bobModelPart` (private in the game's `mobs::anim`).
fn bob(p: &mut Poses, n: &str, age: f32, scale: f32) {
    let q = p.get(n);
    q[5] += scale * ((age * 0.09).mcos() * 0.05 + 0.05);
    q[3] += scale * ((age * 0.067).msin() * 0.05);
}

/// The vanilla two-handed poses for a weapon in the off hand: the game gives
/// only the main hand the bow/crossbow pose (`AvatarRenderer.getArmPose`
/// leaves the off hand at `ArmPose::Item`), so the off hand holds the weapon
/// down. Mirror the game's formulas with the off hand as the holding hand; the
/// main hand's two-handed pose wins, as in vanilla.
fn apply_offhand_two_handed(p: &mut Poses, st: &BmState) {
    if !st.is_player || st.main_two_handed {
        return;
    }
    let hx = p.get("head")[3];
    let hy = p.get("head")[4];
    if st.using_off_hand && st.using_item > 0 && st.off_crossbow {
        crossbow_charge(p, st.off_charge_max, st.using_item as f32, false);
    } else if st.using_off_hand && st.using_item > 0 && st.off_bow {
        // `ArmPose::Bow`, the game's own formulas.
        p.yrot("right_arm", -0.1 + hy);
        p.yrot("left_arm", 0.1 + hy + 0.4);
        p.xrot("right_arm", -std::f32::consts::PI / 2.0 + hx);
        p.xrot("left_arm", -std::f32::consts::PI / 2.0 + hx);
    } else if st.off_charged && st.swing == 0.0 {
        crossbow_hold(p, hx, hy, false);
    } else {
        return;
    }
    // `humanoid` bobs the arms after posing them; the mod runs after it.
    bob(p, "right_arm", st.age, 1.0);
    bob(p, "left_arm", st.age, -1.0);
}

/// Crossbow pose clamp: the arms never swing past the shoulder or up behind
/// the head.
fn apply_clamp_crossbow(p: &mut Poses, st: &BmState) {
    if !bm_on(bm().clamp_crossbow) || !(st.crossbow_charge || st.crossbow_hold) {
        return;
    }
    for name in ["right_arm", "left_arm"] {
        let x = {
            let a = p.get(name);
            wrap_deg(a[3]).clamp(-1.85, 0.0)
        };
        p.xrot(name, x);
    }
}

/// CustomBowAnimation (priority 3200, `bowAnimation` = CUSTOM_V1): the
/// custom draw pose, the arms track the aim.
fn apply_custom_bow(p: &mut Poses, st: &BmState) {
    let n = bm();
    if !n.master || n.bow_mode != BowAnimation::CustomV1 || !st.using_bow {
        return;
    }
    // The bow is in the left hand when a right-handed player uses the off
    // hand or a left-handed player the main hand.
    let left = if st.main_arm_left { !st.using_off_hand } else { st.using_off_hand };
    let invert = if left { -1.0 } else { 1.0 };
    let (main, off) = if left { ("left_arm", "right_arm") } else { ("right_arm", "left_arm") };
    // `model.head.xRot` is the head pitch, `wrapDegrees` wraps radians.
    let pitch = wrap_deg(-st.pitch.to_radians());
    let yaw = wrap_deg(st.head_yaw);
    let main_z = p.get(main)[5];
    let off_z = p.get(off)[5];
    p.yrot(main, invert * (-0.1 + pitch).clamp(-1.1, 0.45));
    p.xrot(main, (-1.570_796_4 + invert * yaw).clamp(-1.85, 0.0));
    p.zrot(main, main_z + invert * 1.35);
    p.yrot(off, invert * (0.1 + pitch).clamp(-0.95, 0.65));
    p.xrot(off, (-1.570_796_4 + invert * yaw + 0.8).clamp(-0.95, -0.6));
    p.zrot(off, off_z + invert * 1.35);
}

/// Crawling pose: hands and knees pull the body along; each limb advances in
/// turn, mirrored left/right, driven by the crawl distance.
fn apply_crawling(p: &mut Poses, st: &BmState) {
    if !bm_on(bm().crawling) || st.swim_in_water {
        return;
    }
    let amount = st.swim;
    if amount <= 0.0 {
        return;
    }
    let step = st.pos * 2.4;
    let reach = (step.sin() * 0.9).clamp(-0.9, 0.9);
    let lift = (step.cos() * 0.5).clamp(-0.5, 0.5);
    let lerp = |a: f32, from: f32, to: f32| from + a * (to - from);
    let m_amount = if st.swing_right_arm == Some(true) { 0.0 } else { amount };
    let n_amount = if st.swing_right_arm == Some(false) { 0.0 } else { amount };
    let rrx = { let a = p.get("right_arm"); a[3] };
    let rrz = { let a = p.get("right_arm"); a[5] };
    let lrx = { let a = p.get("left_arm"); a[3] };
    let lrz = { let a = p.get("left_arm"); a[5] };
    let rlx = { let a = p.get("right_leg"); a[3] };
    let llx = { let a = p.get("left_leg"); a[3] };
    // The right arm reaches and sweeps with the right leg trailing; the left
    // pair mirrors it half a cycle later.
    p.xrot("right_arm", lerp(m_amount, rrx, -0.6 + reach * 0.8));
    p.zrot("right_arm", lerp(m_amount, rrz, 0.35));
    p.xrot("left_arm", lerp(n_amount, lrx, -0.6 - reach * 0.8));
    p.zrot("left_arm", lerp(n_amount, lrz, -0.35));
    p.xrot("right_leg", lerp(amount, rlx, -0.35 + lift * 0.7));
    p.zrot("right_leg", 0.18);
    p.xrot("left_leg", lerp(amount, llx, -0.35 - lift * 0.7));
    p.zrot("left_leg", -0.18);
}

/// Free-fall pose: the arms and legs rise out to the sides gradually, and the
/// windmilling builds up the longer the fall lasts.
fn apply_falling(p: &mut Poses, st: &BmState) {
    if !bm_on(bm().falling) || !st.falling {
        return;
    }
    // Buildup over roughly half a second of free fall.
    let t = (st.fall_duration / 12.0).clamp(0.0, 1.0);
    let speed = st.fall_speed.min(1.0);
    let spread = (0.4 + 1.2 * speed) * t;
    let wave = t * (0.6 + 0.6 * speed);
    let phase = st.age * 0.7;
    p.xrot("left_arm", phase.mcos() * wave);
    p.zrot("left_arm", -spread);
    p.xrot("right_arm", (phase + std::f32::consts::PI).mcos() * wave);
    p.zrot("right_arm", spread);
    p.xrot(
        "left_leg",
        (phase + std::f32::consts::PI * 0.75).mcos() * wave * 0.8,
    );
    p.zrot("left_leg", -(0.15 + 0.35 * speed) * t);
    p.xrot(
        "right_leg",
        (phase + std::f32::consts::PI * 0.25).mcos() * wave * 0.8,
    );
    p.zrot("right_leg", (0.15 + 0.35 * speed) * t);
}

/// NarutoRunningAnimation (priority 500): sprinting sweeps both arms
/// back, unless one is swinging.
fn apply_naruto(p: &mut Poses, st: &BmState) {
    if !bm_on(bm().naruto) || !st.sprinting {
        return;
    }
    let swinging = |left: bool| {
        st.swing > 0.0
            && match st.swing_right_arm {
                Some(right) => left == !right,
                None => left == st.main_arm_left,
            }
    };
    if !swinging(false) {
        p.xrot("right_arm", 1.0);
        p.yrot("right_arm", -0.2);
        p.zrot("right_arm", 0.3);
    }
    if !swinging(true) {
        p.xrot("left_arm", 1.0);
        p.yrot("left_arm", 0.2);
        p.zrot("left_arm", -0.3);
    }
}

/// Item-swap pose: both hands swing inward across the body and settle back,
/// over a 14-tick ease. While the freezing effect is active the hands stay put.
fn apply_item_swap(p: &mut Poses, st: &BmState) {
    if !bm_on(bm().item_swap) || st.swap_at <= 0.0 || st.freezing {
        return;
    }
    let elapsed = st.age - st.swap_at;
    if !(0.0..=14.0).contains(&elapsed) {
        return;
    }
    let u = elapsed / 14.0;
    // Fast at the start, settling at the end.
    let roll = (-1.35 * (1.0 - u) * (1.0 - u)).max(-1.35);
    p.xrot("right_arm", -0.45);
    p.yrot("right_arm", 0.18);
    p.zrot("right_arm", roll);
    p.xrot("left_arm", -0.45);
    p.yrot("left_arm", -0.18);
    p.zrot("left_arm", -roll);
}

/// Burning pose: a slight head roll and calm arm flaps, no flailing.
fn apply_burning(p: &mut Poses, st: &BmState) {
    if !bm_on(bm().burning) || !st.burning {
        return;
    }
    let t = st.age * 1.4;
    let head_roll = (t * 1.3).msin() * 0.055;
    p.get("head")[4] += head_roll;
    p.get("head")[5] += (t * 1.1).msin() * 0.035;
    let flap = (t.msin() * 0.5 + (t * 1.7).msin() * 0.3) * 0.16;
    bm_flail(p, st, |p| {
        p.xrot("right_arm", -2.2 + flap);
        p.yrot("right_arm", -0.15);
        p.zrot("right_arm", -0.25 - flap * 0.3);
    }, |p| {
        p.xrot("left_arm", -2.2 - flap);
        p.yrot("left_arm", 0.15);
        p.zrot("left_arm", 0.25 + flap * 0.3);
    });
}

/// Freezing pose: the arms fold inward and shake, with a light head shiver.
/// Hands holding an item keep their pose.
fn apply_freezing(p: &mut Poses, st: &BmState) {
    if !bm_on(bm().freezing) || !st.freezing {
        return;
    }
    let t = st.age * 2.0;
    let shake = (t.sin() * 0.5 + (t * 1.6).sin() * 0.5) * 0.035;
    let right_roll = -1.0 + shake;
    let left_roll = 1.25 + shake;
    p.get("head")[4] += shake * 0.5;
    p.get("head")[5] += (t * 1.2).sin() * 0.03;
    bm_flail(p, st, |p| {
        p.xrot("right_arm", -0.5);
        p.yrot("right_arm", 0.2);
        p.zrot("right_arm", right_roll);
    }, |p| {
        p.xrot("left_arm", -0.6);
        p.yrot("left_arm", -0.2);
        p.zrot("left_arm", left_roll);
    });
}

/// SleepAnimation (`freezeArmsInBed`): both arms lie still in the bed.
fn apply_sleep(p: &mut Poses, st: &BmState) {
    if !bm_on(bm().sleep) || !st.sleep_pose {
        return;
    }
    p.xrot("right_arm", 0.0);
    p.yrot("right_arm", 0.0);
    p.zrot("right_arm", 0.0);
    p.xrot("left_arm", 0.0);
    p.yrot("left_arm", 0.0);
    p.zrot("left_arm", 0.0);
}

/// Apply `right`/`left` to the free arms; an arm busy with higher-priority
/// work keeps its pose. Busy arms: swinging, using an item, the item-swap
/// timer, climbing, riding, the spear hand, and hands holding an item in a
/// hold-up pose (map, lantern, item).
fn bm_flail(
    p: &mut Poses,
    st: &BmState,
    right: impl Fn(&mut Poses),
    left: impl Fn(&mut Poses),
) {
    let swing_right = st.swing_right_arm;
    let swinging = |left: bool| {
        st.swing > 0.0
            && match swing_right {
                Some(right) => left == !right,
                None => left == st.main_arm_left,
            }
    };
    let using_left = st.using_item > 0 && (st.using_off_hand != st.main_arm_left);
    let using_right = st.using_item > 0 && !using_left;
    let swap_busy = st.swap_at > 0.0 && st.age - st.swap_at < 10.0;
    let busy = |left: bool| {
        let main = left == st.main_arm_left;
        swinging(left)
            || (left && using_left)
            || (!left && using_right)
            || swap_busy
            || st.climbing
            || st.boat
            || (main && (st.spear || st.main_hold))
            || (!main && st.off_hold)
    };
    if !busy(false) {
        right(p);
    }
    if !busy(true) {
        left(p);
    }
}

/// Animation smoothing (`enableAnimationSmoothing`): a new tick moves the
/// target 20%-ish toward the freshly animated pose (the configurable speed);
/// every render interpolates from the last rendered pose to that target by
/// the partial tick, so remote players' arms glide between updates. The legs
/// keep their vanilla pose when `disableLegSmoothing` is set.
fn apply_smoothing(p: &mut Poses, st: &BmState) {
    let n = bm();
    if !n.master || !n.smoothing || st.horse {
        return;
    }
    let speed = n.smoothing_speed.clamp(0.0, 1.0);
    let delta = st.partial.clamp(0.0, 1.0);
    let legs = !n.disable_leg_smoothing;
    SMOOTH.with(|cell| {
        let mut map = cell.borrow_mut();
        let e = map.entry(st.id).or_insert(SmoothEntry {
            prev: [[0.0; 3]; 4],
            target: [[0.0; 3]; 4],
            render: [[0.0; 3]; 4],
            partial: st.partial,
            started: false,
            last_age: st.age,
            stall: 0,
        });
        let first = !e.started;
        // A partial tick that went backwards (or the first sight of the mob)
        // starts a new tick.
        let new_tick = first || st.partial < e.partial;
        // The interpolation is stuck when renders keep coming with the same
        // tick and partial (a paused world, an open inventory, a frozen server
        // tick): re-seed the limbs to the pose they should be in, so the model
        // follows the game instead of holding its last smoothed pose.
        let stalled = e.last_age == st.age && e.partial == st.partial;
        e.stall = if stalled { e.stall.saturating_add(1) } else { 0 };
        e.last_age = st.age;
        e.partial = st.partial;
        e.started = true;
        let limbs = [
            ("left_arm", 0),
            ("right_arm", 1),
            ("left_leg", 2),
            ("right_leg", 3),
        ];
        for (name, i) in limbs {
            if i >= 2 && !legs {
                continue;
            }
            let cur = p.get(name);
            let cur = [cur[3], cur[4], cur[5]];
            if e.stall >= 2 {
                e.prev[i] = cur;
                e.target[i] = cur;
                e.render[i] = cur;
            } else {
                if new_tick {
                    // `last[3] = last[0] = last[6]`, then the target moves toward
                    // the new pose by `speed`.
                    for k in 0..3 {
                        e.prev[i][k] = if first { cur[k] } else { e.render[i][k] };
                        e.target[i][k] = if first {
                            cur[k]
                        } else {
                            rotlerp_rad(speed, cur[k], e.prev[i][k])
                        };
                    }
                }
                for k in 0..3 {
                    e.render[i][k] = rotlerp_rad(delta, e.prev[i][k], e.target[i][k]);
                }
            }
            p.xrot(name, e.render[i][0]);
            p.yrot(name, e.render[i][1]);
            p.zrot(name, e.render[i][2]);
        }
    });
}

/// `AnimationUtil.interpolateRotation` in degrees: the shortest turn from
/// `from` toward `to` by `t`.
fn lerp_deg(from: f32, to: f32, t: f32) -> f32 {
    let mut diff = to - from;
    while diff > 180.0 {
        diff -= 360.0;
    }
    while diff < -180.0 {
        diff += 360.0;
    }
    from + diff * t
}

/// The item the mob is using, main or off hand.
fn used_item(m: &Mob) -> Option<&str> {
    if m.extra.using_off_hand {
        m.extra.off_hand_id()
    } else {
        m.extra.main_hand_id()
    }
}

/// ActionRotationLockAnimation's target: eating, drinking or blocking. The
/// two-handed and single-handed vanilla poses (bow, crossbow, spyglass, goat
/// horn, brush, trident) keep the vanilla body.
fn rotation_target(m: &Mob) -> bool {
    if (m.extra.use_animation == UseAnimation::Block) {
        return true;
    }
    m.extra.using_item > 0
        && !matches!(
            used_item(m),
            Some("minecraft:bow")
                | Some("minecraft:crossbow")
                | Some("minecraft:spyglass")
                | Some("minecraft:goat_horn")
                | Some("minecraft:brush")
                | Some("minecraft:trident")
        )
        && m.extra.main_hand_id().and_then(crate::spear::Kinetic::of).is_none()
        && m.extra.off_hand_id().and_then(crate::spear::Kinetic::of).is_none()
}

/// CustomBowAnimation's body turn (`customBowRotationLock`): the body turns
/// 32 degrees away from the aim, mirrored for a bow in the left hand.
fn custom_bow_body(m: &Mob) -> Option<f32> {
    let n = bm();
    if n.bow_mode != BowAnimation::CustomV1
        || !n.custom_bow_rotation
        || m.extra.using_item <= 0
        || used_item(m) != Some("minecraft:bow")
    {
        return None;
    }
    let bow_left = if m.main_arm_left() {
        !m.extra.using_off_hand
    } else {
        m.extra.using_off_hand
    };
    Some(m.yaw + if bow_left { 32.0 } else { -32.0 })
}

/// Turn the local player's body to the head (the body-follows-head rotation
/// lock). The camera entity is limited to first person when the setting says
/// so; a ladder's body lock and a vehicle keep the vanilla body.
pub fn rotate_local_body(m: &mut Mob, first_person: bool) {
    let n = bm();
    if !n.master {
        return;
    }
    if let Some(deg) = custom_bow_body(m) {
        m.body_yaw = deg;
        m.previous_body_yaw = deg;
        return;
    }
    if m.extra.passenger || (CLIMBING.with(|c| c.get()) && LADDER_LOCK.with(|c| c.get())) {
        return;
    }
    if n.limit_rotation_fp && !first_person {
        return;
    }
    let rotate = match n.rotation_mode {
        RotationLock::None => n.rotation_lock && rotation_target(m),
        RotationLock::Fixed | RotationLock::Smooth => true,
    };
    if !rotate {
        return;
    }
    let tick = m.age;
    if LOCAL_TICK.with(|c| c.get()) == tick {
        return;
    }
    LOCAL_TICK.with(|c| c.set(tick));
    if n.rotation_mode == RotationLock::Fixed {
        m.body_yaw = m.head_yaw;
        m.previous_body_yaw = m.head_yaw;
    } else {
        let old = m.body_yaw;
        m.body_yaw = lerp_deg(old, m.head_yaw, 0.5);
        m.previous_body_yaw = old;
    }
}

/// `applyRotationLockToEveryone`: the same turn for every other player.
pub fn rotate_remote_bodies<'a>(mobs: impl IntoIterator<Item = &'a mut Mob>) {
    let n = bm();
    if !n.master {
        return;
    }
    ROT_TICK.with(|cell| {
        let mut ticks = cell.borrow_mut();
        for m in mobs {
            if m.id == 0 || m.kind.id != "minecraft:player" || m.extra.passenger {
                continue;
            }
            // CustomBowAnimation's body turn is part of the animation itself,
            // so every rendered player gets it, not only with the rotation lock.
            if let Some(deg) = custom_bow_body(m) {
                m.body_yaw = deg;
                m.previous_body_yaw = deg;
                continue;
            }
            if !n.rotation_everyone {
                continue;
            }
            let rotate = match n.rotation_mode {
                RotationLock::None => n.rotation_lock && rotation_target(m),
                RotationLock::Fixed | RotationLock::Smooth => true,
            };
            if !rotate {
                continue;
            }
            let tick = m.age;
            if ticks.get(&m.id).copied() == Some(tick) {
                continue;
            }
            ticks.insert(m.id, tick);
            if n.rotation_mode == RotationLock::Fixed {
                m.body_yaw = m.head_yaw;
                m.previous_body_yaw = m.head_yaw;
            } else {
                let old = m.body_yaw;
                m.body_yaw = lerp_deg(old, m.head_yaw, 0.5);
                m.previous_body_yaw = old;
            }
        }
    });
}
