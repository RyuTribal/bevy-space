use crate::common::*;
use bevy::prelude::*;
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

#[derive(Component)]
pub struct AlienBullet;

pub fn alien_bullet_movement(
    time: Res<Time>,
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &mut Transform), With<AlienBullet>>,
) {
    for (entity, mut transform) in &mut bullet_query {
        if transform.translation.y < -SCENE_HEIGHT {
            info!("bullet despawn");
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
    for (alien, mut transform) in &mut aliens {
        // drop bullet?
        if store.instant.elapsed() > Duration::from_millis(200)
            && rand::random::<f32>() < 0.25f32 / ((1 + ALIENS_TOTAL - store.aliens_killed) as f32)
        {
            store.instant = Instant::now();
            info!("bullet spawned {:?}", store.instant);
            commands.spawn((
                AlienBullet,
                SpriteBundle {
                    transform: *transform,
                    texture: store.texture_handler.clone(),
                    ..default()
                },
            ));
        }

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
