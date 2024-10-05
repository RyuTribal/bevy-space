# Bevy-Space

Space invaders in Bevy, why not?

Small experiment to assess complexity of getting a simple thing done from scratch.

So far:

- Basically a working game.

Todo:

- Mystery ship, possibly shooting homing missiles
- Joy stick control
- Particle system for bullet traces and explosions on impact
- Stretch goals
  - Screen projection shader to replicate CRT
  - Screen blur and suitable noise effects to get low quality video, VCR like shaders

## Tools used

- Pixelorama, well never used it before so why not trying it out.

---

## Setup

See `Cargo.toml`

- `dynamic_linking` (for reduced compilation time)
- `log` (settings for removing logging in release builds), seems not to work though
- `profile dev/release (for reasonable performance)

See `.cargo/config.toml`

- `mold` linker (for compilation speed)

All in all, after initial build, compile times are within seconds.

---

## How to play

- `[Enter]` to insert coin (start game)
- `[A]`/`[Left arrow]`, `[D]`/`[Right arrow]` to move
- `[Space]`/`[Up arrow]` to shoot


---

## Design Documentation

The game uses the Bevy ECS to partition state and functionality. The initial design used `Resources` shared among the `Components` to determine the game logic. While adding more `Components` (and functionality) the number of dependencies between the `systems` grew. This is not necessarily a problem, however the amount of code duplication was increasing (and it started to become messy to accomplish desired behavior). This is not a unique problem the this particular game, instead an expected effect of shared state. The problem can be addressed in various ways, e.g., by implementing methods on the state holding `Resources` and/or by using `Events`. I opted to migrate towards `Events` primarily.

### Events

Technically, events (if used correctly) increase available parallelism among systems (as under the Bevy hood, the need for "locking" of shared resources are reduced). For this particular application, this is not any major concern but in a realistic game parallel execution is in general desirable.

Events are currently declared along with their main `Resource`, so e.g., the `game_state` module defines the `GameStateEvent`. (Alternatively, all `Event` could be declared or re-exported by a separate module for convenience.) The `Events` are listed by `Component` below.

| Module            | Declared         | Used |
| ------------------| ---------------- | ---- |
| `alien`           | -                | -    |
| `audio`           | `PlaySoundEvent` | -    |
| `bunker`          | -                |      |
| `common`          | -                |      |
| `game_state`      | -                |      |
| `hit_detection`   | -                |      |
| `keyboard_input`  | -                |      |
| `lazer`           | -                |      |
| `lib`             | -                |      |
| `main`            | -                |      |
| `overlay`         | -                |      |
| `player`          | -                |      |


---

## License

Bevy-space is free, open source and permissively licensed! Except where noted (below and/or in individual files), all code in this repository is dual-licensed under either:

- MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer! This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are very good reasons to include both.

Some of the assets in this repo are cloned from Bevy game engine (under different licenses).

## Your contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
