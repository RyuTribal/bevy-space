# Bevy-Space

Space invaders in Bevy, why not?

Small experiment to assess complexity of getting a simple thing done from scratch.

So far:

- Basically a working game already (in less than two days)

Todo:

- Game menu and such

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
