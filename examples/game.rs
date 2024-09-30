//! Renders a 2D scene containing a single, moving sprite.
//! RUST_LOG="game=info" cargo run --example game

use bevy::prelude::*;

const PLAYER_SPEED: f32 = 300.0;
const PLAYER_HEIGHT: f32 = 50.0; // There should be a way to get this from sprite
const LAZER_SPEED: f32 = 500.0;
const SCENE_WIDTH: f32 = 600.0;
const SCENE_HEIGHT: f32 = 300.0;
const ALIENS_COL: usize = 11;
const ALIENS_ROW: usize = 5;
const ALIENS_WIDTH: f32 = 50.0;
const ALIENS_HEIGHT: f32 = 50.0;

const ALIENS_SPEED: f32 = 30.0;

#[derive(Component)]
enum Player {
    Left,
    Right,
    None,
}

#[derive(Component, PartialEq, Clone, Copy)]
enum Lazer {
    Fire,
    Fired,
    Idle,
}

/// keyboard input
fn keyboard_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut direction_match: Query<&mut Player>,
    mut lazer_match: Query<&mut Lazer>,
) {
    for mut direction in &mut direction_match {
        let mut new_direction = Player::None;
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            trace!("'A' / <-");
            new_direction = Player::Left;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            trace!("'D' / ->");
            new_direction = Player::Right;
        }
        *direction = new_direction;
    }

    for mut lazer in &mut lazer_match {
        if *lazer == Lazer::Idle
            && (keyboard_input.just_pressed(KeyCode::Space)
                || keyboard_input.pressed(KeyCode::ArrowUp))
        {
            trace!("' ' ^");
            *lazer = Lazer::Fire;
        }
    }
}

/// player movement
fn player_movement(time: Res<Time>, mut player: Query<(&mut Player, &mut Transform)>) {
    for (direction, mut transform) in &mut player {
        match *direction {
            Player::Left => {
                if transform.translation.x > -SCENE_WIDTH {
                    transform.translation.x -= PLAYER_SPEED * time.delta_seconds()
                }
            }
            Player::Right => {
                if transform.translation.x < SCENE_WIDTH {
                    transform.translation.x += PLAYER_SPEED * time.delta_seconds()
                }
            }
            _ => {}
        }
    }
}

/// lazer movement
fn lazer_movement(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut lazer_position: Query<(&mut Lazer, &mut Visibility, &mut Transform), Without<Player>>,
) {
    // get a player_transform singleton
    let mut player_iterator = player_query.iter();
    let player_transform = player_iterator.next().unwrap();
    assert!(player_iterator.next().is_none());

    let mut lazer_iterator = lazer_position.iter_mut();
    let (mut lazer, mut visibility, mut transform) = lazer_iterator.next().unwrap();
    assert!(lazer_iterator.next().is_none());

    match *lazer {
        Lazer::Fire => {
            transform.translation =
                player_transform.translation + Vec3::new(0.0, PLAYER_HEIGHT, 0.0);
            *lazer = Lazer::Fired;
            *visibility = Visibility::Visible;
        }
        Lazer::Fired => {
            if transform.translation.y > SCENE_HEIGHT {
                *lazer = Lazer::Idle;
            } else {
                transform.translation.y += LAZER_SPEED * time.delta_seconds()
            }
        }
        _ => {
            *visibility = Visibility::Hidden;
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
}
#[derive(Component)]
struct Alien {
    direction: Direction,
}

/// alien movement
fn alien_movement(time: Res<Time>, mut aliens: Query<(&mut Alien, &mut Transform)>) {
    let mut new_direction = None;
    for (alien, mut transform) in &mut aliens {
        match alien.direction {
            Direction::Left => {
                transform.translation.x -= ALIENS_SPEED * time.delta_seconds();
                if transform.translation.x < -SCENE_WIDTH {
                    new_direction = Some(Direction::Right);
                }
            }
            Direction::Right => {
                transform.translation.x += ALIENS_SPEED * time.delta_seconds();
                if transform.translation.x > SCENE_WIDTH {
                    new_direction = Some(Direction::Left);
                }
            }
        }
    }

    if let Some(direction) = new_direction {
        for (mut alien, mut transform) in &mut aliens {
            transform.translation.y -= ALIENS_HEIGHT;
            alien.direction = direction;
        }
    }
}

fn hit_detection(
    mut commands: Commands,
    alien_query: Query<(Entity, &Transform), With<Alien>>,
    mut lazer_query: Query<(&mut Lazer, &Transform)>,
) {
    // get lazer singleton
    let mut lazer_iterator = lazer_query.iter_mut();
    let (mut lazer, lazer_transform) = lazer_iterator.next().unwrap();
    assert!(lazer_iterator.next().is_none());

    if *lazer == Lazer::Fired {
        let mut nr_checked = 0;
        for (entity, enemy_transform) in alien_query.iter() {
            nr_checked += 1;
            let x = enemy_transform.translation.x;
            let y = enemy_transform.translation.y;
            let half_w = ALIENS_WIDTH / 2.0;
            let half_h = ALIENS_HEIGHT / 2.0;

            let x_range = (x - half_w)..(x + half_w);
            let y_range = (y - half_h)..(x + half_h);

            let lazer_x = lazer_transform.translation.x;
            let lazer_y = lazer_transform.translation.y;

            // Your collision check
            if x_range.contains(&lazer_x) && (y_range.contains(&lazer_y)) {
                println!(
                    "hit at x {}, y {}, lazer_x {}, lazer_y {}",
                    x, y, lazer_x, lazer_y
                );
                commands.entity(entity).despawn();
                *lazer = Lazer::Idle;
            }
        }
        println!("nr_checked {}", nr_checked);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

    // Builds and spawns the sprites
    let sprite_handle = asset_server.load("sprites/alien.png");

    let mut aliens = vec![];
    let step_x = 1.5 * ALIENS_WIDTH;
    let step_y = 1.5 * ALIENS_HEIGHT;
    for y in 0..ALIENS_ROW {
        for x in 0..ALIENS_COL {
            aliens.push((
                Alien {
                    direction: Direction::Right,
                },
                SpriteBundle {
                    texture: sprite_handle.clone(),
                    transform: Transform::from_xyz(
                        (x as f32 - ALIENS_COL as f32 / 2.) * step_x,
                        SCENE_HEIGHT - (y as f32 * step_y),
                        0.,
                    ),
                    ..Default::default()
                },
            ));
        }
    }
    commands.spawn_batch(aliens);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            ((
                keyboard_input_system, // first
                (
                    hit_detection,                                               // second
                    (player_movement, (lazer_movement, alien_movement)).chain(), // 3rd in parallel
                )
                    .chain(),
            )
                .chain(),),
        )
        .run();
}
