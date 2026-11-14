# Body Motion

Full-body player animations for [Horizon](https://playhorizon.dev), shipped as a native mod: maps held in both hands, eating and drinking, ladder climbing and crawling, boat rowing, horse riding, petting, hugging, falling, burning, freezing and more — all animated in third person.

Body Motion is a Horizon port of [Not Enough Animations](https://modrinth.com/mod/not-enough-animations) by tr7zw. The mod, its settings screen, its translations and every line of code are written from scratch in Rust by **romaeboj**.

## Features

- **World map rendering** — held maps render in the world, in one or both hands.
- **Animations** — ladders (with snap-to-ladder), crawling, eating/drinking, boat rowing, horse riding (plus a separate leg-pose option), item swapping, petting wolves and cats, freezing, burning, elytra tweaks, bed items and frozen sleep arms, offhand hiding while drawing a bow, hugging, Naruto running, and a falling animation that kicks in past 4 blocks.
- **Rotation locking** — locks the body to the head's rotation: None / Fixed / Smooth, first-person only, apply to every player, and separate normal/blocking angles.
- **Animation smoothing** — smooths arms and legs, with an amount slider and a leg-only toggle.
- **Hold Up Items** — players hold up what they inspect: configurable / all / no / inverted item modes, a camera target, and offset sliders.
- **Lanterns** — lanterns swing while held.

The settings screen has three tabs, a search field, delayed tooltips and is fully translated into 14 languages: English, Russian, Ukrainian, German (de/AT/CH), French, Polish, Portuguese (BR/PT), Swedish, Turkish, Chinese (simplified/traditional).

## Install

1. Download `BodyMotion-<version>.hnmod` from [Releases](../../releases).
2. Put it into your instance's `horizon-native-mods` folder (for example `AppData\Roaming\horizon\instances\<instance>\horizon-native-mods\`).
3. Restart the game. Native mods are compiled for your game build, so the launcher rebuilds the mod when Horizon updates.

Configure it in game: **Options → Mods → Body Motion → Configure**.

## Build from source

Clone the repo and compile it into your game build with the Horizon launcher:

```
horizon-launcher native-build <path to this repo> --version <Horizon build id> --trust-native-code
```

`--version` is the Horizon version/commit the mod compiles against (shown in the launcher), and `--trust-native-code` confirms the mod is allowed to hook into the game. The build is incremental; the first one takes a few minutes.

## Credits and license

- Based on **Not Enough Animations** © [tr7zw](https://github.com/tr7zw) — the original concept: <https://github.com/tr7zw/NotEnoughAnimations>
- Code, Horizon port, settings screen and translations: © 2026 **romaeboj**, released under the MIT-style license in [LICENSE](LICENSE).
- Not affiliated with Mojang or Microsoft.

## Русский

**Body Motion** — порт мода *Not Enough Animations* (автор tr7zw) для Horizon: анимации тела от третьего лица — карты в руках, еда, лестницы, гребля, езда на лошади, объятия, падение и многое другое, плюс экран настроек с поиском и подсказками на 14 языках.

Установка: скачайте `BodyMotion-<версия>.hnmod` из Releases и положите в папку `horizon-native-mods` вашего инстанса, затем перезапустите игру. Настройки: **Опции → Моды → Body Motion → Настроить**.
