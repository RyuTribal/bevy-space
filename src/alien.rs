use crate::common::*;
use bevy::prelude::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::store::*;

#[derive(Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Component)]
pub struct Alien {
    pub direction: Direction,
}

#[derive(Component, Clone, Copy)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub fn animate_alien_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

#[derive(Component)]
pub struct AlienBullet;

pub fn alien_bullet_movement(
    time: Res<Time>,
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &mut Transform), With<AlienBullet>>,
) {
    for (entity, mut transform) in &mut bullet_query {
        if transform.translation.y < -SCENE_HEIGHT {
            trace!("bullet despawn");
            commands.entity(entity).despawn();
        } else {
            transform.translation.y -= ALIEN_BULLET_SPEED * time.delta_seconds();
        }
    }
}

/// alien movement and shooting
pub fn alien_movement(
    time: Res<Time>,

    mut store: ResMut<Store>,
    mut commands: Commands,

    mut aliens: Query<(&mut Alien, &mut Transform)>,
) {
    let mut new_direction = None;

    let delta = time.delta_seconds();

    for (alien, mut transform) in &mut aliens {
        match alien.direction {
            Direction::Left => {
                transform.translation.x -= store.alien_speed * delta;
                if transform.translation.x < -SCENE_WIDTH {
                    new_direction = Some(Direction::Right);
                }
            }
            Direction::Right => {
                transform.translation.x += store.alien_speed * delta;
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

    // calculate the lowest y value among aliens (lowest row)
    let mut hm = HashMap::new();
    aliens.iter().for_each(|(_, t)| {
        let Vec3 { x, y, z: _ } = t.translation;
        let x = x as i32;
        if let Some(y_min) = hm.get(&x) {
            if y < *y_min {
                hm.insert(x, y);
            }
        } else {
            hm.insert(x, y);
        }
    });

    // filter out candidates at lowest row for each column
    let mut aliens = aliens.iter_mut().filter(|(_, t)| {
        let Vec3 { x, y, z: _ } = t.translation;
        let x = x as i32;
        &y == hm.get(&x).unwrap()
    });

    for (_, transform) in &mut aliens {
        // drop bullet?
        if store.instant.elapsed() > Duration::from_millis(500)
            && rand::random::<f32>() < 0.1f32 / (hm.len() as f32)
        {
            store.instant = Instant::now();
            trace!("bullet spawned {:?}", store.instant);
            let mut texture = None;
            store.texture_handler.clone_into(&mut texture);
            let texture = texture.unwrap();

            commands.spawn((
                AlienBullet,
                SpriteBundle {
                    transform: *transform,
                    texture,
                    ..default()
                },
            ));
        }
    }
}

// Builds and spawns the Alien sprites
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Builds and spawns the Alien sprites
    let texture = asset_server.load("sprites/alien.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(64, 48), 4, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 3 };

    let mut aliens = vec![];
    let step_x = ALIENS_SPACE;
    let step_y = ALIENS_SPACE * 0.75;
    for y in 0..ALIENS_ROW {
        for x in 0..ALIENS_COL {
            aliens.push((
                Alien {
                    direction: Direction::Right,
                },
                SpriteBundle {
                    transform: Transform::from_xyz(
                        (x as f32 - ALIENS_COL as f32 / 2.0) * step_x,
                        SCENE_HEIGHT - 100.0 - (y as f32 * step_y),
                        -1.0, // behind in scene
                    ),
                    texture: texture.clone(),
                    ..default()
                },
                TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: animation_indices.first,
                },
                animation_indices,
                AnimationTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
            ));
        }
    }
    commands.spawn_batch(aliens);
}
