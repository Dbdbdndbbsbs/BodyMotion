// The frame hook: refresh the mod's settings (and the local player's
// powder-snow flag) for the animation half, once a frame.
fn bm_frame(game: &mut crate::game::Game<'_>) {
    use crate::game::client_ambient::native_bm_glue as glue;
    use crate::game::client_ambient::native_bm_runtime as m;
    use crate::mobs::anim::native_bm_anim as anim;

    // The settings screen's frame: data files, sounds, closing.
    crate::game::native_bm_screen::frame(game);

    let s = glue::settings(game);
    anim::set_bm(anim::BmSettings {
        master: s.enabled,
        eat_drink: s.eat_drink,
        ladder: s.ladder,
        rotate_to_ladder: s.rotate_to_ladder,
        ladder_amplifier: s.ladder_amplifier,
        ladder_arm_height: s.ladder_arm_height,
        ladder_arm_speed: s.ladder_arm_speed,
        boat: s.boat,
        item_swap: s.item_swap,
        burning: s.burning,
        freezing: s.freezing,
        crawling: s.crawling,
        sleep: s.sleep,
        maps: s.maps,
        offhand_hiding: s.offhand_hiding,
        hug: s.hug,
        pet: s.pet,
        naruto: s.naruto,
        falling: s.falling,
        horse: s.horse,
        horse_leg: s.horse_leg,
        rotation_lock: s.rotation_lock,
        rotation_mode: match s.rotation_mode {
            m::RotationLock::None => anim::RotationLock::None,
            m::RotationLock::Fixed => anim::RotationLock::Fixed,
            m::RotationLock::Smooth => anim::RotationLock::Smooth,
        },
        limit_rotation_fp: s.limit_rotation_fp,
        rotation_everyone: s.rotation_everyone,
        max_normal_angle: s.max_normal_angle,
        max_blocking_angle: s.max_blocking_angle,
        bow_mode: match s.bow_mode {
            m::BowAnimation::Vanilla => anim::BowAnimation::Vanilla,
            m::BowAnimation::CustomV1 => anim::BowAnimation::CustomV1,
        },
        custom_bow_rotation: s.custom_bow_rotation,
        clamp_crossbow: s.clamp_crossbow,
        tweak_elytra: s.tweak_elytra,
        smoothing: s.smoothing,
        disable_leg_smoothing: s.disable_leg_smoothing,
        smoothing_speed: s.smoothing_speed,
        hold_up_mode: match s.hold_up_mode {
            m::HoldUpMode::Config => anim::HoldUpMode::Config,
            m::HoldUpMode::All => anim::HoldUpMode::All,
            m::HoldUpMode::None => anim::HoldUpMode::None,
            m::HoldUpMode::ConfigInverted => anim::HoldUpMode::ConfigInverted,
        },
        hold_up_target: match s.hold_up_target {
            m::HoldUpTarget::None => anim::HoldUpTarget::None,
            m::HoldUpTarget::Camera => anim::HoldUpTarget::Camera,
        },
        hold_up_item_offset: s.hold_up_item_offset,
        hold_up_camera_offset: s.hold_up_camera_offset,
        hold_up_only_self: s.hold_up_only_self,
    });
    // The Hold Up Items list (the tab's toggles).
    anim::set_holding_list(s.holding_items.clone());

    let Some(p) = game.player.as_mut() else {
        anim::set_powder_snow(false);
        anim::set_fire_ticks(0);
        anim::set_climbing(false);
        anim::set_ladder_lock(false);
        anim::set_burning(false);
        anim::set_freezing(false);
        anim::set_local_sprinting(false);
        anim::set_local_spin(false);
        anim::set_local_dying(false);
        anim::set_local_falling(false, 0.0);
        anim::set_local_k(1.0);
        anim::set_offhand_crossbow(false, 25.0);
        anim::set_hug(false);
        anim::set_pet(false);
        anim::set_horse_riders(Default::default());
        anim::set_horse_pump(0.0);
        anim::set_horse_stand(0.0);
        crate::game::client_ambient::native_bm_lantern::reset();
        anim::reset_animation_state();
        return;
    };

    // the original mod's LadderAnimation: onClimbable && !onGround && lastClimbable is a
    // ladder or trapdoor. `last_climbable` is set only when the player is on
    // a climbable block, so its presence stands in for `climbable()`.
    // `PlayerMotion.on_climbable` is the client's own `onClimbable()` result
    // for this tick; not on the ground, it means the player is climbing
    // (matching the original's `onClimbable() && !onGround`).
    let climbing = p.on_climbable && !p.ground.on_ground;
    // The body locks to the ladder's own block: its `facing` points away from
    // the wall, so the player faces the opposite way. `last_climbable` is the
    // block the player occupies, so its position is the ladder itself. Vines
    // and scaffolding have no `facing` and keep the vanilla body and head.
    let mut ladder_face = 0.0;
    let mut ladder_lock = false;
    if climbing
        && let Some(pos) = p.last_climbable
    {
        let id = game.world.block(pos.x, pos.y, pos.z);
        if let Some(state) = game.pack.assets.registry.state(id)
            && let Some(facing) = state.properties.get("facing")
        {
            ladder_face = match facing.as_str() {
                "north" => 0.0,
                "south" => std::f32::consts::PI,
                "west" => 3.0 * std::f32::consts::FRAC_PI_2,
                "east" => std::f32::consts::FRAC_PI_2,
                _ => 0.0,
            };
            ladder_lock = true;
        }
    }
    anim::set_climbing(climbing);
    anim::set_ladder_face(ladder_face);
    // `enableRotateToLadder`: only then does the body lock and the head clamp.
    let ladder_lock = ladder_lock && s.rotate_to_ladder;
    anim::set_ladder_lock(ladder_lock);
    // The model's world yaw is what actually turns the body: lock it to the
    // ladder so the player's own turning does not rotate it.
    if climbing && ladder_lock && s.enabled && s.ladder {
        if let Some(m) = game.player_model.as_mut() {
            let deg = ladder_face.to_degrees();
            m.body_yaw = deg;
            m.previous_body_yaw = deg;
        }
    }

    // the original mod's FreezingAnimation: in powder snow and able to freeze.
    let freezing = p.in_powder_snow && p.can_freeze();
    anim::set_freezing(freezing);

    // the original mod's BurningAnimation: on fire without fire resistance.
    let burning = p.fire_ticks > 0 && !p.status.fire_resistance;
    anim::set_burning(burning);

    // the original mod's FreezingAnimation: the local player stands in powder snow (the
    // game already tracks it on `PlayerMotion`). BurningAnimation reads the
    // local player's remaining fire ticks, since the model's own field is
    // only synced for remote players.
    anim::set_powder_snow(p.in_powder_snow);
    anim::set_fire_ticks(p.fire_ticks);

    // the original mod's ItemSwapAnimation: the hands traded since the last frame.
    let main = game.inventory[game.selected]
        .as_ref()
        .map(|s| s.item.clone());
    let off = game.offhand.as_ref().map(|s| s.item.clone());
    let age = game.player_model.as_ref().map_or(0, |m| m.age);
    anim::update_hands(age as f32, main, off);

    // the original mod's BoatAnimation: the boat the local player rides, and its paddles.
    let boat = game
        .riding
        .and_then(|id| game.boats.iter().find(|b| b.id == id))
        .map(|b| (b.paddling, b.paddles));
    anim::update_boat(boat);

    // The local player's mob entry: the use/block/sprint extras the synced
    // fields carry for it too.
    anim::set_local_sprinting(p.sprinting);
    anim::set_local_spin(p.auto_spin_attack_ticks > 0);
    anim::set_local_dying(p.health.dead_or_dying());
    anim::set_local_k((p.velocity.length_squared() as f32 / 0.2).powi(3).max(1.0));

    // The game tracks the loaded crossbow and its charge time in the main hand
    // only (`Perspective::update_model`); the mod mirrors the off-hand pose and
    // hides the main-hand map from these.
    let (off_charged, off_charge_max) = game.offhand.as_ref().map_or((false, 25.0), |s| {
        (
            &*s.item == "minecraft:crossbow"
                && s.data().is_some_and(|d| !d.charged.is_empty()),
            crate::game::Game::<'_>::charge_ticks(s).max(1) as f32,
        )
    });
    anim::set_offhand_crossbow(off_charged, off_charge_max);

    // the original mod's FallingAnimation: free fall, not gliding, climbing, flying (creative
    // or spectator) or swimming; the speed is |velocity|²/11 and the trigger
    // needs more than 4 blocks of fall distance (our threshold: the original
    // used 3).
    let falling_ok = !p.fall_flying
        && !p.ground.on_ground
        && !climbing
        && !p.flying
        && !p.spectator
        && !p.swimming;
    let fall_speed = if falling_ok { p.velocity.length_squared() as f32 / 11.0 } else { 0.0 };
    anim::set_local_falling(falling_ok && p.fall_distance > 4.0, fall_speed);

    // the original mod's HorseAnimation: the mob ids riding a horse-family mount.
    //
    // The original checks the mount is saddled (`AbstractHorse.isSaddled`),
    // but riding is only possible on a saddled mount anyway, and the flag is
    // not always synced to the client's mob list yet.
    let is_horse = |m: &crate::mobs::Mob| {
        matches!(
            m.kind.id.as_str(),
            "minecraft:horse"
                | "minecraft:donkey"
                | "minecraft:mule"
                | "minecraft:skeleton_horse"
                | "minecraft:zombie_horse"
                | "minecraft:llama"
                | "minecraft:trader_llama"
                | "minecraft:camel"
        )
    };
    let mut riders = crate::fast_hash::FxHashSet::default();
    let mut horse_walk = 0.0;
    let mut horse_standing = false;
    let mut ridden = false;
    if let Some(v) = game.riding
        && let Some(vm) = game.mobs.iter().find(|m| m.id == v)
        && is_horse(vm)
    {
        riders.insert(0);
        ridden = true;
        horse_walk = vm.limb_position;
        // `AbstractHorse.isStanding`: the synced rear flag.
        horse_standing = vm.extra.counter > 0;
        // The rear animation (`AbstractHorse.standAnim`): the horse's body
        // pitches up by 45°, and the rider's legs follow it.
        anim::set_horse_stand(vm.anim.lerp(1, game.clock.fraction() as f32));
        // The pump follows the mount's horizontal speed (0 when idle or
        // mid-jump), so the rider's legs and arms stop with it.
        let vel = vm.velocity;
        let speed = ((vel.x * vel.x + vel.z * vel.z).sqrt() / 0.25) as f32;
        anim::set_horse_pump(speed.clamp(0.0, 1.0));
    }
    for m in game.mobs.iter() {
        if let Some(v) = m.extra.mount
            && let Some(vm) = game.mobs.iter().find(|vm| vm.id == v)
            && is_horse(vm)
        {
            riders.insert(m.id);
        }
    }
    anim::set_horse_riders(riders);
    anim::set_horse_walk(horse_walk);
    anim::set_horse_standing(horse_standing);
    if !ridden {
        anim::set_horse_stand(0.0);
    }
    if !ridden || horse_standing {
        anim::set_horse_pump(0.0);
    }

    // the original mod's HugAnimation / PetAnimation: the local player crouches with another
    // crouching player (hug) or a wolf or cat (pet) in front of the eyes, on
    // about the same height. A ray along the view vector stands in for the
    // original's entity hit test.
    let mut hug = false;
    let mut pet = false;
    if p.local_crouching && !climbing && !boat.map_or(false, |(paddling, _)| paddling[0] || paddling[1]) {
        let yaw = p.yaw.to_radians() as f64;
        let pitch = p.pitch.to_radians() as f64;
        let view = [
            -yaw.sin() * pitch.cos(),
            -pitch.sin(),
            yaw.cos() * pitch.cos(),
        ];
        let eye_height = if p.local_crouching { 1.27 } else { 1.62 };
        let eye = [
            p.position.x,
            p.position.y + eye_height,
            p.position.z,
        ];
        for m in game.mobs.iter() {
            if m.removed || m.id == 0 {
                continue;
            }
            let center = [
                m.position.x,
                m.position.y + m.kind.height / 2.0,
                m.position.z,
            ];
            let rel = [center[0] - eye[0], center[1] - eye[1], center[2] - eye[2]];
            let t = rel[0] * view[0] + rel[1] * view[1] + rel[2] * view[2];
            if !(0.0..=1.0).contains(&t) {
                continue;
            }
            let perp = (rel[0] * rel[0] + rel[1] * rel[1] + rel[2] * rel[2] - t * t).max(0.0).sqrt();
            if perp > m.kind.width / 2.0 + 0.2 {
                continue;
            }
            let dy = (m.position.y - p.position.y).abs();
            if m.kind.id == "minecraft:player" && m.extra.crouching && dy < 0.3 {
                hug = true;
            }
            if matches!(m.kind.id.as_str(), "minecraft:wolf" | "minecraft:cat") && dy < 0.6 {
                pet = true;
            }
        }
    }
    anim::set_hug(hug);
    anim::set_pet(pet);

    // PlayerTransformer's rotation lock: the body follows the head. The
    // camera entity obeys `limitRotationLockToFP`; other players need
    // `applyRotationLockToEveryone`. The custom bow's body turn is separate
    // (`customBowRotationLock`).
    if s.enabled {
        if let Some(m) = game.player_model.as_mut() {
            anim::rotate_local_body(m, game.camera_type.first_person());
        }
        anim::rotate_remote_bodies(&mut game.mobs);
    }
}
