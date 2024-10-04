use crate::{common::*, game_state::*, lazer::FireLazerEvent, player::Player};
use bevy::prelude::*;

/// keyboard input
pub fn update_system(
    mut fire_lazer_event_writer: EventWriter<FireLazerEvent>,
    mut game_state_event_writer: EventWriter<GameStateEvent>,
    mut store: ResMut<Store>,

    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Player>,
) {
    let mut player = player_query.single_mut();

    let mut new_direction = Direction3::None;
    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        new_direction = Direction3::Left;
    }
    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        new_direction = Direction3::Right;
    }
    player.direction = new_direction;

    if keyboard_input.just_pressed(KeyCode::KeyI) {
        store.show_state ^= true; // toggle
    }

    match store.game_state {
        GameState::Play => {
            if keyboard_input.just_pressed(KeyCode::Space)
                || keyboard_input.pressed(KeyCode::ArrowUp)
            {
                fire_lazer_event_writer.send(FireLazerEvent);
            }
        }

        _ => {
            if keyboard_input.just_pressed(KeyCode::Enter) {
                game_state_event_writer.send(GameStateEvent(GameState::Start));
            }
        }
    }
}
