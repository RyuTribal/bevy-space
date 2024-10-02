use crate::{lazer::Lazer, player::Player, store::*};
use bevy::prelude::*;

/// keyboard input
pub fn keyboard_input_system(
    mut store: ResMut<Store>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Player>,
    mut lazer_query: Query<&mut Lazer>,
) {
    for mut direction in &mut player_query {
        let mut new_direction = Player::None;
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            new_direction = Player::Left;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            new_direction = Player::Right;
        }
        *direction = new_direction;
    }

    match store.game_state {
        GameState::Play => {
            for mut lazer in &mut lazer_query {
                if *lazer == Lazer::Idle
                    && (keyboard_input.just_pressed(KeyCode::Space)
                        || keyboard_input.pressed(KeyCode::ArrowUp))
                {
                    *lazer = Lazer::Fire;
                }
            }
        }

        _ => {
            if keyboard_input.just_pressed(KeyCode::Space) {
                store.game_state = GameState::Play
            }
        }
    }
}
