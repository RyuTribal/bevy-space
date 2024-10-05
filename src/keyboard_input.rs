use crate::{common::*, game_state::*, lazer::FireLazerEvent, player::PlayerEvent};
use bevy::prelude::*;

/// keyboard input
pub fn update_system(
    mut fire_lazer_ew: EventWriter<FireLazerEvent>,
    mut game_state_ew: EventWriter<GameStateEvent>,
    mut player_ew: EventWriter<PlayerEvent>,
    mut store: ResMut<Store>,

    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    // slow or fast
    let speed = if keyboard_input.pressed(KeyCode::ShiftLeft) {
        PLAYER_SLOW
    } else {
        1.0
    };
    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        player_ew.send(PlayerEvent(-speed));
    }
    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        player_ew.send(PlayerEvent(speed));
    }

    if keyboard_input.just_pressed(KeyCode::KeyI) {
        store.show_state ^= true; // toggle
    }

    match store.game_state {
        GameState::InsertCoin | GameState::LeaderBoard => {
            if keyboard_input.just_pressed(KeyCode::Enter) {
                game_state_ew.send(GameStateEvent::PressPlay);
            }
        }
        _ => {
            if keyboard_input.just_pressed(KeyCode::Space)
                || keyboard_input.pressed(KeyCode::ArrowUp)
            {
                println!("-- fire lazer event sent --");
                fire_lazer_ew.send(FireLazerEvent);
            }
        }
    }
}
