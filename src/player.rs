use crate::common::*;
use bevy::prelude::*;

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
