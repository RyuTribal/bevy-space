//! Space Invaders revisited, why not?
//! RUST_LOG="bevy-space=info" cargo run

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::WindowResolution};
// use rand::prelude::*;
use bevy_space::{
    alien, bunker::Bunker, common::*, hit_detection, keyboard_input, lazer, lazer::Lazer, overlay,
    player, player::Player, store::Store,
};

use std::time::Instant;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // we might want to setup a custom camera, for now just default
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Player::None,
        SpriteBundle {
            texture: asset_server.load("sprites/space.png"),
            transform: Transform::from_xyz(0., -SCENE_HEIGHT, 0.),
            ..default()
        },
    ));
    commands.spawn((
        Lazer::Idle,
        SpriteBundle {
            texture: asset_server.load("sprites/lazer.png"),
            transform: Transform::from_xyz(0., SCENE_HEIGHT, 0.),
            visibility: Visibility::Hidden,
            ..default()
        },
    ));

    // Builds and spawns the bunker sprites
    let texture = asset_server.load("sprites/defense.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 4, 2, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // The sprite index layout of the bunker
    let bunker_matrix = [
        [0, 1, 1, 1, 1, 2],
        [1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 1],
    ];

    for b in 0..BUNKERS {
        let mut bunker = vec![];
        for (r, row) in bunker_matrix.iter().enumerate() {
            for (c, data) in row.iter().enumerate() {
                bunker.push((
                    Bunker,
                    SpriteBundle {
                        transform: Transform::from_xyz(
                            (c as f32 - (row.len() as f32 - 1.0) / 2.0) * 16.0
                                + (2.0 * b as f32 - (BUNKERS as f32 - 1.0)) * BUNKER_SPACE,
                            BUNKERS_Y - SCENE_HEIGHT - (r as f32) * 16.0,
                            0.0,
                        ),
                        texture: texture.clone(),
                        ..default()
                    },
                    TextureAtlas {
                        layout: texture_atlas_layout.clone(),
                        index: *data,
                    },
                ));
            }
        }
        commands.spawn_batch(bunker);
    }

    // Loads bullet sprite
    let texture_handler: Handle<Image> = asset_server.load("sprites/drop.png");
    commands.insert_resource(Store {
        texture_handler,
        instant: Instant::now(),
        score: 0,
        aliens_killed: 0,
    });
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
        .add_systems(Startup, (setup, alien::setup, overlay::setup).chain())
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
