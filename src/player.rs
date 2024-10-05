use crate::{common::*, game_state::*};
use bevy::prelude::*;

#[derive(Event)]
pub struct PlayerEvent(pub f32);

#[derive(Component)]
pub struct Player;

/// player movement
pub fn update_system(
    time: Res<Time>,
    mut player_er: EventReader<PlayerEvent>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let mut transform = player_query.single_mut();

    for event in player_er.read() {
        if event.0 < 0.0 && transform.translation.x > -SCENE_WIDTH
            || event.0 > 0.0 && transform.translation.x < SCENE_WIDTH
        {
            transform.translation.x += event.0 * PLAYER_SPEED * time.delta_seconds()
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

    *visibility = match store.game_state {
        GameState::PlayerSpawn(spawn_count) => {
            if spawn_count % 2u8 == 0 {
                Visibility::Visible
            } else {
                Visibility::Hidden
            }
        }
        _ => Visibility::Visible,
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player,
        SpriteBundle {
            texture: asset_server.load("sprites/space.png"),
            transform: Transform::from_xyz(0., -SCENE_HEIGHT, 0.),
            ..default()
        },
    ));
}
