use crate::common::*;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Player {
    pub direction: Direction3,
    pub spawn_counter: u8,
}

/// player movement
pub fn update_system(time: Res<Time>, mut player_query: Query<(&Player, &mut Transform)>) {
    for (player, mut transform) in &mut player_query {
        match player.direction {
            Direction3::Left => {
                if transform.translation.x > -SCENE_WIDTH {
                    transform.translation.x -= PLAYER_SPEED * time.delta_seconds()
                }
            }
            Direction3::Right => {
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

pub fn blink_update_system(
    time: Res<Time>,
    mut blink_query: Query<(&mut Visibility, &mut Player, &mut BlinkTimer)>,
) {
    let (mut visibility, mut player, mut timer) = blink_query.single_mut();

    timer.tick(time.delta());
    if player.spawn_counter > 0 && timer.just_finished() {
        // toggle visibility
        if *visibility == Visibility::Visible {
            *visibility = Visibility::Hidden;
        } else {
            player.spawn_counter -= 1;
            *visibility = Visibility::Visible
        }
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player::default(),
        BlinkTimer(Timer::from_seconds(
            PLAYER_SPAWN_DURATION,
            TimerMode::Repeating,
        )),
        SpriteBundle {
            texture: asset_server.load("sprites/space.png"),
            transform: Transform::from_xyz(0., -SCENE_HEIGHT, 0.),
            ..default()
        },
    ));
}
