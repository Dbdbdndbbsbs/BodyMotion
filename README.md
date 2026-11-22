# Body Motion

Smoother and more expressive third-person animations for [Horizon](https://playhorizon.dev): eating, drinking, climbing, riding, maps, lanterns and more.

A standalone reimplementation inspired by the Not Enough Animations mod, written as a **native mod** on Horizon's current native mod API. Not affiliated with the original mod.

## Requirements

- **Horizon nightly-2026-11-22-2b77287** (commit `2b77287de099c9316278ba7ac4d83590fdb7ef30`), Minecraft game `26.3`.
- Native mod support enabled in the launcher (Settings → Allow native mods). Native mods are compiled on your computer; they run with your user rights, not in a sandbox.
- The mod is built against a specific game commit. Newer Nightlies may rebuild it automatically (Update), older versions are not guaranteed to work.

## What's inside

- `src/` — the mod source (Rust).
- `assets/` — language files.
- `icon.png` — mod icon.
- `Cargo.toml`, `horizon-mod.toml` — the native mod manifest (pins the game commit and mixin revision).

## Install

1. In the launcher, open **Mods → Add from GitHub** and enter `https://github.com/Dbdbdndbbsbs/BodyMotion` (or run `horizon-launcher mods add-github https://github.com/Dbdbdndbbsbs/BodyMotion --trust-native-code`). The launcher puts the package in the instance's `horizon-native-mods/` folder.
2. Press **Build** for Body Motion. The launcher compiles it for your installed Nightly.
3. Launch the game. Settings live in **Options → Mods → Body Motion** (custom screen).

Manual install: put `bodymotion.hmod` into the instance's `horizon-native-mods/` folder, not `horizon-mods/`. A native package in `horizon-mods/` is treated as content-only and never compiled.

## Changelog highlights

- 0.3.1: rebuilt for **nightly-2026-11-22-2b77287**; new mod icon.
- Ported to the new native mod API (`api 0.2`) — no legacy `.hnmod`/mixin-patch files.
- Custom settings screen instead of registry settings rows.
- Map, lantern and item pose fixes (including filled maps in both hands).
- Petting animation follows the animal; smooth aim.
- Eating/drinking pose applies only to real food and drink — a spear in use keeps its vanilla pose.
- Fixed icon (`icon.png`) shown in the mods list.

## License

MIT — see `LICENSE`. The NOTICE file keeps the original authors' credits.
