//! Renders a 2D scene containing a single, moving sprite.
//! RUST_LOG="game=info" cargo run --example game

use bevy::prelude::*;

const PLAYER_SPEED: f32 = 300.0;
const SCENE_WIDTH: f32 = 400.0;
const SCENE_HEIGHT: f32 = 300.0;

#[derive(Component)]
enum Direction {
    Left,
    Right,
    None,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Lazer {
    is_fired: bool,
}

/// keyboard input
fn keyboard_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut direction_match: Query<&mut Direction>,
    mut lazer_match: Query<&mut Lazer>,
) {
    for mut direction in &mut direction_match {
        let mut new_direction = Direction::None;
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            trace!("'A' / <-");
            new_direction = Direction::Left;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            trace!("'D' / ->");
            new_direction = Direction::Right;
        }
        *direction = new_direction;
    }

    for mut lazer in &mut lazer_match {
        if !lazer.is_fired
            && (keyboard_input.just_pressed(KeyCode::Space)
                || keyboard_input.pressed(KeyCode::ArrowUp))
        {
            info!("' ' ^");
            lazer.is_fired = true;
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/space.png"),
            transform: Transform::from_xyz(0., -SCENE_HEIGHT, 0.),
            ..default()
        },
        Player,
        Direction::None,
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/lazer.png"),
            transform: Transform::from_xyz(0., SCENE_HEIGHT, 0.),
            ..default()
        },
        Lazer { is_fired: false },
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/alien.png"),
            transform: Transform::from_xyz(0., SCENE_HEIGHT, 0.),
            ..default()
        },
        Direction::None,
    ));
}

/// player movement
fn player_movement(
    time: Res<Time>,
    mut sprite_position: Query<(&mut Direction, &mut Transform), With<Player>>,
) {
    for (direction, mut transform) in &mut sprite_position {
        match *direction {
            Direction::Left => {
                if transform.translation.x > -SCENE_WIDTH {
                    transform.translation.x -= PLAYER_SPEED * time.delta_seconds()
                }
            }
            Direction::Right => {
                if transform.translation.x < SCENE_WIDTH {
                    transform.translation.x += PLAYER_SPEED * time.delta_seconds()
                }
            }

            Direction::None => {}
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (keyboard_input_system, player_movement))
        .run();
}
