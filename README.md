# Bevy-Space

Space invaders in Bevy, why not?

Small experiment to assess complexity of getting a simple thing done from scratch.

So far:

- Alien and space ship sprites.
- Keyboard input for movement.

## Tools used

- Pixelorama, well never used it before so why not trying it out.
  
## Setup

See `Cargo.toml`

- `dynamic_linking` (for reduced compilation time)
- `log` (settings for removing logging in release builds)
- `profile dev/release (for reasonable performance)
  
See `.cargo/config.toml`

- `mold` linker (for compilation speed)

All in all, after initial build, compile times are within seconds.
