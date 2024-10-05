use crate::{common::*, game_state::*};
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Player {
    pub direction: Direction3,
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

// it uses the shared game_state to determine if visible
// alternatively one could declare an event to determine state changes
// but the cost is low so we don't do that
pub fn blink_update_system(
    store: Res<Store>,
    mut player_query: Query<&mut Visibility, With<Player>>,
) {
    let mut visibility = player_query.single_mut();

    match store.game_state {
        GameState::PlayerSpawn(spawn_count) => {
            if spawn_count % 2u8 == 0 {
                *visibility = Visibility::Visible;
            } else {
                *visibility = Visibility::Hidden;
            }
        }
        _ => *visibility = Visibility::Visible,
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player::default(),
        SpriteBundle {
            texture: asset_server.load("sprites/space.png"),
            transform: Transform::from_xyz(0., -SCENE_HEIGHT, 0.),
            ..default()
        },
    ));
}
