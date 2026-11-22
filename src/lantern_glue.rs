// The per-entity lantern sway state: a soft, inertia-free pendulum that eases
// toward an offset proportional to the hand's motion. The lantern stays near
// its resting center below the hand and leans only as much as the motion
// dictates.

use crate::mobs::Mob;
use std::cell::RefCell;

/// The lantern's sway state.
#[derive(Clone, Copy)]
pub struct LanternState {
    pub last_tick: i64,
    pub last_hand: [f64; 3],
    pub offset: [f64; 3],
    pub prev_offset: [f64; 3],
}

thread_local! {
    static LANTERNS: RefCell<crate::fast_hash::FxHashMap<u32, LanternState>> =
        RefCell::new(crate::fast_hash::FxHashMap::default());
}

/// Forget every lantern (left the world).
pub fn reset() {
    LANTERNS.with(|c| c.borrow_mut().clear());
}

/// Forget lanterns held by entities that left the world this frame.
pub fn prune_stale(alive: &crate::fast_hash::FxHashSet<u32>) {
    LANTERNS.with(|c| c.borrow_mut().retain(|id, _| alive.contains(id)));
}

/// Update the lantern's sway. Returns the sway offset in world space,
/// interpolated by the partial tick. `hand` is the hand pivot's world
/// position, `yaw` the player's view yaw in degrees, `delta` the partial tick
/// between two ticks.
pub fn spring(
    mob: &Mob,
    game_time: i64,
    hand: [f64; 3],
    delta: f32,
    yaw: f64,
) -> [f64; 3] {
    let _ = yaw;
    LANTERNS.with(|c| {
        let map = &mut *c.borrow_mut();
        let state = map.entry(mob.id).or_insert(LanternState {
            last_tick: game_time,
            last_hand: hand,
            offset: [0.0; 3],
            prev_offset: [0.0; 3],
        });
        // A teleport (more than 10 blocks of movement) resets the swing.
        let dist = ((hand[0] - state.last_hand[0]).powi(2)
            + (hand[1] - state.last_hand[1]).powi(2)
            + (hand[2] - state.last_hand[2]).powi(2))
        .sqrt();
        if dist > 10.0 {
            state.last_hand = hand;
            state.offset = [0.0; 3];
            state.prev_offset = [0.0; 3];
            return [0.0; 3];
        }
        if state.last_tick != game_time {
            state.prev_offset = state.offset;
            let hand_dx = [
                hand[0] - state.last_hand[0],
                hand[1] - state.last_hand[1],
                hand[2] - state.last_hand[2],
            ];
            // The target offset is proportional to the hand's motion (lagging
            // it), so the lantern leans gently and stays anywhere between the
            // center and the limit, depending on how fast the hand moves.
            for i in 0..3 {
                let target = -hand_dx[i] * 0.5;
                state.offset[i] += (target - state.offset[i]) * 0.25;
            }
            // Gravity: the lantern rests a little below the hand.
            state.offset[1] += (-0.22 - state.offset[1]) * 0.1;
            for i in 0..3 {
                state.offset[i] = state.offset[i].clamp(-0.8, 0.8);
            }
            state.last_tick = game_time;
            state.last_hand = hand;
        }
        let d = delta as f64;
        [
            state.prev_offset[0] + (state.offset[0] - state.prev_offset[0]) * d,
            state.prev_offset[1] + (state.offset[1] - state.prev_offset[1]) * d,
            state.prev_offset[2] + (state.offset[2] - state.prev_offset[2]) * d,
        ]
    })
}
