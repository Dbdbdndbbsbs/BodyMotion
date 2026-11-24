// The per-entity lantern swing: a two-axis angular pendulum hanging from the
// holding hand. Gravity pulls the angles back to vertical, damping bleeds the
// swing, and the hand's own acceleration pushes the lantern the other way, so
// it lags behind the hand and settles on its own. The suspension length is
// fixed, so the lantern can never rise above its pivot or leave the hand.

use crate::mobs::Mob;
use std::cell::RefCell;

/// Gravity, in blocks per second squared.
const GRAVITY: f32 = 9.81;
/// Suspension length in blocks: about a 1.5 s period.
const LENGTH: f32 = 0.55;
/// Damping ratio: a few visible swings, settled in two or three seconds.
const DAMPING_RATIO: f32 = 0.22;
/// The swing limit, 50 degrees, keeps the lantern clear of the body.
const MAX_ANGLE: f32 = 0.872_664_6;
/// The hand's acceleration is clamped so a lag spike cannot fling the lantern.
const MAX_DRIVE: f32 = 30.0;
/// Integration steps per game tick.
const SUBSTEPS: u32 = 4;
/// One game tick, in seconds.
const TICK: f32 = 0.05;

/// The lantern's swing state: two angles (body right, body forward) and their
/// speeds, plus the hand's last position and speed for the drive.
#[derive(Clone, Copy)]
pub struct LanternState {
    pub last_tick: i64,
    pub last_hand: [f64; 3],
    pub hand_vel: [f64; 3],
    pub angle: [f32; 2],
    pub prev_angle: [f32; 2],
    pub vel: [f32; 2],
}

thread_local! {
    /// Keyed by entity id and hand, so two lanterns swing independently.
    static LANTERNS: RefCell<crate::fast_hash::FxHashMap<(u32, bool), LanternState>> =
        RefCell::new(crate::fast_hash::FxHashMap::default());
}

/// Forget every lantern (left the world).
pub fn reset() {
    LANTERNS.with(|c| c.borrow_mut().clear());
}

/// Forget lanterns held by entities that left the world this frame.
pub fn prune_stale(alive: &crate::fast_hash::FxHashSet<u32>) {
    LANTERNS.with(|c| c.borrow_mut().retain(|(id, _), _| alive.contains(id)));
}

/// Advance the lantern's swing. `hand` is the pivot in the body's own frame
/// (right, up, forward), `delta` the partial tick, `off_hand` the hand holding
/// it. Motion is measured in that frame, so turning the head or the body moves
/// the hand and the lantern together and never feeds the pendulum: only the
/// arm's own motion does. Returns the two swing angles (body right, body
/// forward) in radians, interpolated by the partial tick.
pub fn pendulum(
    mob: &Mob,
    game_time: i64,
    hand: [f64; 3],
    delta: f32,
    off_hand: bool,
) -> [f32; 2] {
    LANTERNS.with(|c| {
        let map = &mut *c.borrow_mut();
        let state = map.entry((mob.id, off_hand)).or_insert(LanternState {
            last_tick: game_time,
            last_hand: hand,
            hand_vel: [0.0; 3],
            angle: [0.0; 2],
            prev_angle: [0.0; 2],
            vel: [0.0; 2],
        });
        let dist = ((hand[0] - state.last_hand[0]).powi(2)
            + (hand[1] - state.last_hand[1]).powi(2)
            + (hand[2] - state.last_hand[2]).powi(2))
        .sqrt();
        // A teleport (more than 10 blocks in a tick) or a gap in the ticks
        // (a culled or paused entity) leaves the lantern hanging still instead
        // of flinging it with a velocity built up over several ticks.
        if dist > 10.0 || (game_time - state.last_tick).abs() > 1 {
            state.last_tick = game_time;
            state.last_hand = hand;
            state.hand_vel = [0.0; 3];
            state.angle = [0.0; 2];
            state.prev_angle = [0.0; 2];
            state.vel = [0.0; 2];
            return [0.0; 2];
        }
        if state.last_tick != game_time {
            state.prev_angle = state.angle;
            // The hand's acceleration this tick, in the body's own frame. A
            // body-fixed frame keeps head and body turns out of the pendulum.
            let mut accel = [0.0f64; 3];
            for i in 0..3 {
                let vel = (hand[i] - state.last_hand[i]) / TICK as f64;
                accel[i] = (vel - state.hand_vel[i]) / TICK as f64;
                state.hand_vel[i] = vel;
            }
            let clamp_drive = |a: f64| a.clamp(-MAX_DRIVE as f64, MAX_DRIVE as f64) as f32;
            let drive = [-clamp_drive(accel[0]) / LENGTH, -clamp_drive(accel[2]) / LENGTH];
            let omega = (GRAVITY / LENGTH).sqrt();
            let damping = 2.0 * DAMPING_RATIO * omega;
            let h = TICK / SUBSTEPS as f32;
            for _ in 0..SUBSTEPS {
                for i in 0..2 {
                    let acc = -omega * omega * state.angle[i].sin()
                        - damping * state.vel[i]
                        + drive[i];
                    state.vel[i] += acc * h;
                    state.angle[i] += state.vel[i] * h;
                    if state.angle[i] > MAX_ANGLE {
                        state.angle[i] = MAX_ANGLE;
                        state.vel[i] = state.vel[i].min(0.0);
                    } else if state.angle[i] < -MAX_ANGLE {
                        state.angle[i] = -MAX_ANGLE;
                        state.vel[i] = state.vel[i].max(0.0);
                    }
                }
            }
            debug_assert!(state.angle.iter().all(|a| a.is_finite()));
            debug_assert!(state.vel.iter().all(|v| v.is_finite()));
            state.last_tick = game_time;
            state.last_hand = hand;
        }
        let d = (delta as f32).clamp(0.0, 1.0);
        [
            state.prev_angle[0] + (state.angle[0] - state.prev_angle[0]) * d,
            state.prev_angle[1] + (state.angle[1] - state.prev_angle[1]) * d,
        ]
    })
}
