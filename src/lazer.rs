use crate::{common::*, player::Player};
use bevy::prelude::*;

#[derive(Component, PartialEq, Clone, Copy)]
pub enum Lazer {
    Fire,
    Fired,
    Idle,
}

/// lazer movement
pub fn lazer_movement(
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
