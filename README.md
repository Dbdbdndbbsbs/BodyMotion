# Body Motion

Smoother and more expressive third-person animations for [Horizon](https://playhorizon.dev): eating, drinking, climbing, riding, maps, lanterns and more.

A standalone reimplementation inspired by the Not Enough Animations mod, written as a **native mod** on Horizon's current native mod API. Not affiliated with the original mod.

## Requirements

- **Horizon nightly-2026-11-21-f037744** (commit `f037744a79f9982c47876f49c5a3067279f88706`), Minecraft game `26.3`.
- Native mod support enabled in the launcher (Settings → Allow native mods). Native mods are compiled on your computer; they run with your user rights, not in a sandbox.
- The mod is built against a specific game commit. Newer Nightlies may rebuild it automatically (Update), older versions are not guaranteed to work.

## What's inside

- `src/` — the mod source (Rust).
- `assets/` — language files.
- `icon.png` — mod icon.
- `Cargo.toml`, `horizon-mod.toml` — the native mod manifest (pins the game commit and mixin revision).

## Install

1. Download `bodymotion.hmod` from the latest release.
2. Place it in the instance's mods folder (or install through the launcher's mod browser / `mods add-github`).
3. In the launcher, press **Build** for the mod. The launcher compiles it for your installed version.
4. Launch the game. Settings live in **Options → Mods → Body Motion** (custom screen).

## Changelog highlights

- Ported to the new native mod API (`api 0.2`) — no legacy `.hnmod`/mixin-patch files.
- Custom settings screen instead of registry settings rows.
- Map, lantern and item pose fixes (including filled maps in both hands).
- Petting animation follows the animal; smooth aim.
- Eating/drinking pose applies only to real food and drink — a spear in use keeps its vanilla pose.
- Fixed icon (`icon.png`) shown in the mods list.

## License

MIT — see `LICENSE`. The NOTICE file keeps the original authors' credits.
