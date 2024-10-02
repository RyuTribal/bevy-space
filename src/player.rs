use crate::common::*;
use bevy::{prelude::*, render::view::visibility};

#[derive(Component)]
pub enum Player {
    Left,
    Right,
    None,
}

/// player movement
pub fn player_movement(time: Res<Time>, mut player_query: Query<(&Player, &mut Transform)>) {
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

#[derive(Component, Deref, DerefMut)]
pub struct BlinkTimer(Timer);

pub fn blink_player_system(
    time: Res<Time>,
    mut blink_query: Query<(&mut Visibility, &mut BlinkTimer)>,
) {
    let (mut visibility, mut timer) = blink_query.single_mut();
    timer.tick(time.delta());
    if timer.just_finished() {
        println!("blink");
        if *visibility == Visibility::Visible {
            *visibility = Visibility::Hidden;
        } else {
            *visibility = Visibility::Visible
        }
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player::None,
        BlinkTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
        SpriteBundle {
            texture: asset_server.load("sprites/space.png"),
            transform: Transform::from_xyz(0., -SCENE_HEIGHT, 0.),
            ..default()
        },
    ));
}
