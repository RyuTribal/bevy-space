//! This example illustrates how to load and play an audio file, and control how it's played.

use bevy::prelude::*;

// fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands.spawn((
//         Music,
//         AudioBundle {
//             source: asset_server.load("sounds/Windless Slopes.ogg"),
//             settings: PlaybackSettings {
//                 mode: bevy::audio::PlaybackMode::Loop,
//                 paused: true,
//                 ..default()
//             },
//         },
//     ));

//     commands.spawn((
//         Beep,
//         AudioBundle {
//             source: asset_server.load("sounds/breakout_collision.ogg"),
//             settings: PlaybackSettings {
//                 mode: bevy::audio::PlaybackMode::Despawn,

//                 ..default(),
//             }
//             ..default()
//         },
//     ));
// }

// pub fn play_beep(
//     commands: &mut commands,

// )
// #[derive(Component)]
// struct Music;

// #[derive(Component)]
// struct Beep;

// fn update_speed(music_controller: Query<&AudioSink, With<Music>>, time: Res<Time>) {
//     if let Ok(sink) = music_controller.get_single() {
//         sink.set_speed(((time.elapsed_seconds() / 5.0).sin() + 1.0).max(0.1));
//     }
// }

// fn pause(
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     music_controller: Query<&AudioSink, With<Music>>,
// ) {
//     if keyboard_input.just_pressed(KeyCode::Space) {
//         if let Ok(sink) = music_controller.get_single() {
//             sink.toggle();
//         }
//     }
// }

// // fn restart(
// //     keyboard_input: Res<ButtonInput<KeyCode>>,
// //     music_controller: Query<&AudioSource, With<MyMusic>>,
// // ) {
// //     if keyboard_input.just_pressed(KeyCode::Space) {
// //         if let Ok(sink) = music_controller.get_single() {
// //             sink.toggle();
// //         }
// //     }
// // }

// fn volume(
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     music_controller: Query<&AudioSink, With<Music>>,
// ) {
//     if let Ok(sink) = music_controller.get_single() {
//         if keyboard_input.just_pressed(KeyCode::KeyA) {
//             sink.set_volume(sink.volume() + 0.1);
//         } else if keyboard_input.just_pressed(KeyCode::KeyB) {
//             sink.set_volume(sink.volume() - 0.1);
//         }
//     }
// }
