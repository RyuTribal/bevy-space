use crate::{audio::PlayMusicEvent, common::*, game_state::*, lazer::Lazer, player::Player};
use bevy::prelude::*;
use std::time::Duration;

/// keyboard input
pub fn update_system(
    mut play_music_event_writer: EventWriter<PlayMusicEvent>,
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
            let mut lazer = lazer_query.single_mut();
            if *lazer == Lazer::Idle
                && (keyboard_input.just_pressed(KeyCode::Space)
                    || keyboard_input.pressed(KeyCode::ArrowUp))
            {
                *lazer = Lazer::Fire;
            }
        }

        _ => {
            if keyboard_input.just_pressed(KeyCode::Enter) {
                store.game_state = GameState::Start;
                play_music_event_writer.send(PlayMusicEvent(false));
                let mut timer = timer.single_mut();
                timer.set_duration(Duration::from_secs_f32(STATE_TRANSITION_DURATION_SHORT));
                timer.reset();
            }
        }
    }
}
