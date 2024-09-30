//! Renders a 2D scene containing a single, moving sprite.
//! RUST_LOG="move_sprite_keyboard=info" cargo run --example move_sprite_keyboard

use bevy::prelude::*;

const PLAYER_SPEED: f32 = 300.0;
const SCENE_WIDTH: f32 = 400.0;
const SCENE_HEIGHT: f32 = 300.0;

/// This system prints 'A' key state
fn keyboard_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut direction: Query<&mut Direction>,
) {
    for (mut direction) in &mut direction {
        let mut new_direction = Direction::None;
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            info!("'A' / <-");
            new_direction = Direction::Left;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            info!("'D' / ->");
            new_direction = Direction::Right;
        }

        *direction = new_direction;

        if keyboard_input.just_pressed(KeyCode::Space) || keyboard_input.pressed(KeyCode::ArrowUp) {
            info!("' ' ^");
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (keyboard_input_system, sprite_movement))
        .run();
}

#[derive(Component)]
enum Direction {
    Left,
    Right,
    None,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/space.png"),
            transform: Transform::from_xyz(0., -SCENE_HEIGHT, 0.),
            ..default()
        },
        Direction::None,
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

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
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
