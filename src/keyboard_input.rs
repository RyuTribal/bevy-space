use crate::{common::*, game_state::*, lazer::Lazer, player::Player};
use bevy::prelude::*;

/// keyboard input
pub fn update_system(
    mut store: ResMut<Store>,
    mut timer: Query<&mut StateTransitionTimer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Player>,
    mut lazer_query: Query<&mut Lazer>,
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
            if keyboard_input.just_pressed(KeyCode::Enter) {
                store.game_state = GameState::Start;
                let mut timer = timer.single_mut();
                timer.reset();
            }
        }
    }
}
