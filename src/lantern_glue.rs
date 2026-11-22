// the mod's LanternAnimation: the per-entity spring state of a held lantern. The
// lantern hangs from a pivot above the hand and chases it (a held-item spring):
// stiffness 0.15, damping 0.95, one full physics tick per game tick, snapped
// when it is more than 10 blocks from the hand.

use crate::mobs::Mob;
use std::cell::RefCell;

/// The lantern's position, velocity and the
/// velocity of the previous tick (the swing lerps between them).
#[derive(Clone, Copy)]
pub struct LanternState {
    pub last_tick: i64,
    pub pos: [f64; 3],
    pub velocity: [f64; 3],
    pub last_velocity: [f64; 3],
    /// Smoothed view yaw (degrees) the swing axes measure against, so a sharp
    /// turn does not yank the lantern.
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

/// `LanternAnimation.updateData` plus the swing's lerped velocity. `hand` is
/// the hand pivot's world position, `yaw` the player's view yaw in degrees,
/// `delta` the partial tick. Returns the lerped velocity.
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
            pos: [mob.position.x, mob.position.y, mob.position.z],
            velocity: [0.0; 3],
            last_velocity: [0.0; 3],
            smooth_yaw: yaw,
        });
        // Low-pass the yaw: the lantern sways against the smoothed direction.
        state.smooth_yaw += (yaw - state.smooth_yaw) * 0.25;
        let yaw = state.smooth_yaw.to_radians();
        let forward = [-yaw.sin(), 0.0, yaw.cos()];
        let right = [forward[2], 0.0, -forward[0]];
        update_data(state, game_time, hand, forward, right);
        // `state.lastLanternVelocity.add(state.lanternVelocity
        //     .subtract(state.lastLanternVelocity).multiply(delta, delta, delta))`.
        (0..3)
            .map(|i| {
                state.last_velocity[i]
                    + (state.velocity[i] - state.last_velocity[i]) * delta as f64
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or([0.0; 3])
    })
}

fn update_data(state: &mut LanternState, game_time: i64, hand: [f64; 3], forward: [f64; 3], right: [f64; 3]) {
    // Reset if too far (a teleport).
    let dist = ((hand[0] - state.pos[0]).powi(2)
        + (hand[1] - state.pos[1]).powi(2)
        + (hand[2] - state.pos[2]).powi(2))
    .sqrt();
    if dist > 10.0 {
        state.pos = hand;
    }
    let mut displacement: [f64; 3] =
        (0..3).map(|i| state.pos[i] - hand[i]).collect::<Vec<_>>().try_into().unwrap_or([0.0; 3]);
    // Add Y movement influence, so the lantern moves when jumping or
    // falling straight, rotated to the player.
    let dy = displacement[1];
    displacement[0] += dy * 0.1 * right[0];
    displacement[2] += dy * 0.1 * right[2];
    let len = (displacement[0] * displacement[0]
        + displacement[1] * displacement[1]
        + displacement[2] * displacement[2]) as f32;
    let mut acceleration = [0.0f64; 3];
    for (i, d) in displacement.iter().enumerate() {
        acceleration[i] = -d * 0.15;
    }
    if game_time == state.last_tick {
        return;
    }
    state.last_tick = game_time;
    if len > 10.0 {
        state.last_velocity = state.velocity;
        state.velocity = acceleration;
        state.pos = hand;
    } else {
        // Standard physics update (full tick, no delta), with a velocity cap
        // so a sharp turn overshoots softly instead of jerking.
        state.last_velocity = state.velocity;
        for i in 0..3 {
            state.velocity[i] = (state.velocity[i] + acceleration[i]) * 0.95;
            state.velocity[i] = state.velocity[i].clamp(-0.35, 0.35);
            state.pos[i] += state.velocity[i];
        }
    }
}
