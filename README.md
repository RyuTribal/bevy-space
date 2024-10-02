# Bevy-Space

Space invaders in Bevy, why not?

Small experiment to assess complexity of getting a simple thing done from scratch.

So far:

- Basically a working game in three days.

Todo:

- Mystery ship, possibly shooting homing missiles
- Joy stick control
- Particle system for bullet traces and explosions on impact
- Stretch goals
  - Screen projection shader to replicate CRT
  - Screen blur and suitable noise effects to get low quality video, VCR like shaders

## Tools used

- Pixelorama, well never used it before so why not trying it out.
  
## Setup

See `Cargo.toml`

- `dynamic_linking` (for reduced compilation time)
- `log` (settings for removing logging in release builds), seems not to work though
- `profile dev/release (for reasonable performance)
  
See `.cargo/config.toml`

- `mold` linker (for compilation speed)

All in all, after initial build, compile times are within seconds.

## How to play

- A/Left arrow, D/Right arrow to move
- Space/Up arrow to shoot
