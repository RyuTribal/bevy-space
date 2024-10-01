//! Space Invaders revisited, why not?
//! RUST_LOG="bevy-space=info" cargo run

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::WindowResolution};
use bevy_space::{
    alien, bunker, common::*, hit_detection, keyboard_input, lazer, overlay, player, store,
};

fn setup(mut commands: Commands) {
    // we might want to setup a custom camera, for now just default
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(RES_X, RES_Y),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(
            Startup,
            (
                setup,
                store::setup,
                player::setup,
                lazer::setup,
                alien::setup,
                bunker::setup,
                overlay::setup,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                keyboard_input::keyboard_input_system,
                hit_detection::hit_detection,
                player::player_movement,
                lazer::lazer_movement,
                alien::alien_movement,
                alien::alien_bullet_movement,
                overlay::text_update_system,
                overlay::score_update_system,
            ), // now all systems parallel
               // .chain(), // all systems in sequential order to keep it simple
        )
        .run();
}
