// The per-entity lantern sway state: a held lantern trails behind the hand
// with inertia, so it swings on quick turns and settles when the hand rests.

use crate::mobs::Mob;
use std::cell::RefCell;

/// The lantern's trail state: where the hand was last tick, the current sway
/// offset, and a smoothed view yaw for the swing axes.
#[derive(Clone, Copy)]
pub struct LanternState {
    pub last_tick: i64,
    pub last_hand: [f64; 3],
    pub sway: [f64; 3],
    pub smooth_yaw: f64,
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

/// Update the lantern's trailing sway. Returns the sway offset to draw the
/// lantern at. `hand` is the hand pivot's world position, `yaw` the player's
/// view yaw in degrees, `delta` the partial tick.
pub fn spring(
    mob: &Mob,
    game_time: i64,
    hand: [f64; 3],
    delta: f32,
    yaw: f64,
) -> [f64; 3] {
    LANTERNS.with(|c| {
        let map = &mut *c.borrow_mut();
        let state = map.entry(mob.id).or_insert(LanternState {
            last_tick: game_time,
            last_hand: hand,
            sway: [0.0; 3],
            smooth_yaw: yaw,
        });
        state.smooth_yaw += (yaw - state.smooth_yaw) * 0.2;
        // A teleport (more than 10 blocks of movement) resets the trail.
        let dist = ((hand[0] - state.last_hand[0]).powi(2)
            + (hand[1] - state.last_hand[1]).powi(2)
            + (hand[2] - state.last_hand[2]).powi(2))
        .sqrt();
        if dist > 10.0 {
            state.last_hand = hand;
            state.sway = [0.0; 3];
            return [0.0; 3];
        }
        if state.last_tick != game_time {
            for i in 0..3 {
                state.sway[i] = state.sway[i] * 0.8 + (hand[i] - state.last_hand[i]) * 0.9;
            }
            state.last_tick = game_time;
            state.last_hand = hand;
        }
        // Scale by the partial tick so frames between ticks stay smooth.
        [
            state.sway[0] * delta as f64,
            state.sway[1] * delta as f64,
            state.sway[2] * delta as f64,
        ]
    })
}
