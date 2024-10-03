use crate::{common::*, particle::*, player::Player};
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component, PartialEq, Clone)]
pub enum Lazer {
    Fire,
    Fired(Timer),
    Idle,
}

/// lazer movement
pub fn update_system(
    commands: Commands,
    time: Res<Time>,
    image: Res<BulletImage>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut lazer_position: Query<(&mut Lazer, &mut Visibility, &mut Transform), Without<Player>>,
) {
    let player_transform = player_query.single_mut();
    let (mut lazer, mut visibility, mut transform) = lazer_position.single_mut();

    match &mut *lazer {
        Lazer::Fire => {
            transform.translation =
                player_transform.translation + Vec3::new(0.0, PLAYER_HEIGHT, 0.0);
            *lazer = Lazer::Fired(Timer::new(
                Duration::from_secs_f32(LAZER_PARTICLE_INTERVAL),
                TimerMode::Repeating,
            ));
            *visibility = Visibility::Visible;
        }
        Lazer::Fired(timer) => {
            timer.tick(time.delta());
            if timer.just_finished() {
                spawn_particle(
                    commands,
                    image,
                    (transform.translation.x, transform.translation.y).into(),
                    (0.0, -LAZER_SPEED * 0.1).into(),
                );
            }

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

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Lazer::Idle,
        SpriteBundle {
            texture: asset_server.load("sprites/lazer.png"),
            transform: Transform::from_xyz(0., SCENE_HEIGHT, 0.),
            visibility: Visibility::Hidden,
            ..default()
        },
    ));
}
