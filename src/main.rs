//! Space Invaders revisited, why not?
//! RUST_LOG="bevy-space=info" cargo run

use bevy::{prelude::*, window::WindowResolution};

// vintage television format
const RES_Y: f32 = 1080.0; // well a bit too modern
const RES_X: f32 = RES_Y * 4.0 / 3.0;

const PLAYER_SPEED: f32 = 500.0;
const PLAYER_HEIGHT: f32 = 50.0; // There should be a way to get this from sprite
const LAZER_SPEED: f32 = 1000.0;

const SCENE_WIDTH: f32 = RES_X / 2.0 - 100.0;
const SCENE_HEIGHT: f32 = RES_Y / 2.0 - 50.0;
const ALIENS_COL: usize = 11;
const ALIENS_ROW: usize = 5;
const ALIENS_SPACE: f32 = 80.0; // used for layout
const ALIEN_SIZE: Vec2 = Vec2::new(64.0, 10.0); // used for hit box

const BUNKERS: usize = 5;
const BUNKER_SPACE: f32 = SCENE_WIDTH / BUNKERS as f32;
const BUNKERS_Y: f32 = 150.0;
const BUNKER_SIZE: Vec2 = Vec2::new(16.0, 16.0);

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
    mut commands: Commands,
    bullet: Res<Bullet>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Player>,
    mut lazer_query: Query<&mut Lazer>,
) {
    for mut direction in &mut player_query {
        let mut new_direction = Player::None;
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            new_direction = Player::Left;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            new_direction = Player::Right;
        }
        *direction = new_direction;
    }

    for mut lazer in &mut lazer_query {
        if *lazer == Lazer::Idle
            && (keyboard_input.just_pressed(KeyCode::Space)
                || keyboard_input.pressed(KeyCode::ArrowUp))
        {
            *lazer = Lazer::Fire;
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyS) {
        println!("S");
        commands.spawn(SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            texture: bullet.texture_handler.clone(),
            ..default()
        });
    }
}

/// player movement
fn player_movement(time: Res<Time>, mut player_query: Query<(&Player, &mut Transform)>) {
    for (direction, mut transform) in &mut player_query {
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

    // get a lazer singleton
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

    // set new direction for all aliens
    if let Some(direction) = new_direction {
        for (mut alien, mut transform) in &mut aliens {
            transform.translation.y -= ALIEN_SIZE.y;
            alien.direction = direction;
        }
    }
}

#[derive(Component, Clone, Copy)]
struct Bunker;

fn hit_detection(
    mut commands: Commands,
    alien_query: Query<(Entity, &Transform), With<Alien>>,
    mut lazer_query: Query<(&mut Lazer, &Transform)>,
    mut bunker_query: Query<(&mut TextureAtlas, Entity, &Transform), With<Bunker>>,
) {
    // check if point:&Transform is in &target:Transform with size:Vec2
    #[inline(always)]
    fn in_rect(point: &Transform, target: &Transform, size: Vec2) -> bool {
        let t_vec: Vec2 = (target.translation.x, target.translation.y).into();
        let p_vec: Vec2 = (point.translation.x, point.translation.y).into();
        let rect = Rect::from_center_size(t_vec, size);
        rect.contains(p_vec)
    }

    #[inline(always)]
    fn hit_bunker(commands: &mut Commands, entity: Entity, mut atlas: Mut<TextureAtlas>) {
        if atlas.index < 4 {
            atlas.index += 4;
        } else {
            commands.entity(entity).despawn();
        }
    }

    // get lazer singleton
    let mut lazer_iterator = lazer_query.iter_mut();
    let (mut lazer, lazer_transform) = lazer_iterator.next().unwrap();
    assert!(lazer_iterator.next().is_none());

    if *lazer == Lazer::Fired {
        // check bunkers
        for (atlas, entity, bunker_transform) in &mut bunker_query {
            if in_rect(lazer_transform, bunker_transform, BUNKER_SIZE) {
                hit_bunker(&mut commands, entity, atlas);
                *lazer = Lazer::Idle;
            }
        }

        // check aliens
        for (entity, enemy_transform) in &alien_query {
            // Collision check
            if in_rect(lazer_transform, enemy_transform, ALIEN_SIZE) {
                commands.entity(entity).despawn();
                *lazer = Lazer::Idle;
            }
        }
    }
}

#[derive(Resource)]
struct Bullet {
    texture_handler: Handle<Image>,
}

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

    // Builds and spawns the Alien sprites
    let sprite_handle = asset_server.load("sprites/alien.png");

    let mut aliens = vec![];
    let step_x = ALIENS_SPACE;
    let step_y = ALIENS_SPACE;
    for y in 0..ALIENS_ROW {
        for x in 0..ALIENS_COL {
            aliens.push((
                Alien {
                    direction: Direction::Right,
                },
                SpriteBundle {
                    texture: sprite_handle.clone(),
                    transform: Transform::from_xyz(
                        (x as f32 - ALIENS_COL as f32 / 2.0) * step_x,
                        SCENE_HEIGHT - (y as f32 * step_y),
                        -1.0, // behind in scene
                    ),
                    ..Default::default()
                },
            ));
        }
    }
    commands.spawn_batch(aliens);

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
                        ..default()
                    },
                ));
            }
        }
        commands.spawn_batch(bunker);
    }

    // Loads bullet sprite
    let texture_handler: Handle<Image> = asset_server.load("sprites/drop.png");
    commands.insert_resource(Bullet { texture_handler });
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
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                keyboard_input_system,
                hit_detection,
                player_movement,
                lazer_movement,
                alien_movement,
            ), // now all systems parallel
               // .chain(), // all systems in sequential order to keep it simple
        )
        .run();
}
