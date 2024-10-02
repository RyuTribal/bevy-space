//! It displays the current FPS in the top left corner and score top right

use bevy::{
    color::palettes::css::{DARK_CYAN, GOLD, MAGENTA, RED, YELLOW},
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::{
    common::*,
    game_state::{GameState, Store},
};

//
#[derive(Component)]
pub struct Fps;

#[derive(Component)]
pub struct StatusBar;

#[derive(Component, Debug)]
pub struct Overlay {
    game_state: GameState,
}

pub fn setup(mut commands: Commands) {
    // FPS
    commands.spawn((
        Fps,
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font_size: STATUS_BAR_FONT_SIZE,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: STATUS_BAR_FONT_SIZE,
                color: GOLD.into(),
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        }),
    ));

    // Status Bas
    commands.spawn((
        StatusBar,
        TextBundle::from_sections([
            TextSection::new(
                "LIVES: ",
                TextStyle {
                    font_size: STATUS_BAR_FONT_SIZE,
                    ..default()
                },
            ),
            // Lives = 1
            TextSection::from_style(TextStyle {
                font_size: STATUS_BAR_FONT_SIZE,
                color: GOLD.into(),
                ..default()
            }),
            TextSection::new(
                "WAVE: ",
                TextStyle {
                    font_size: STATUS_BAR_FONT_SIZE,
                    ..default()
                },
            ),
            // Wave = 3
            TextSection::from_style(TextStyle {
                font_size: STATUS_BAR_FONT_SIZE,
                color: GOLD.into(),
                ..default()
            }),
            TextSection::new(
                "SCORE: ",
                TextStyle {
                    font_size: STATUS_BAR_FONT_SIZE,
                    ..default()
                },
            ),
            // Score
            TextSection::from_style(TextStyle {
                font_size: STATUS_BAR_FONT_SIZE,
                color: GOLD.into(),
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(15.0),
            ..default()
        }),
    ));

    // GameOver
    commands.spawn((
        Overlay {
            game_state: GameState::GameOver,
        },
        TextBundle::from_section(
            "   GAME OVER", // Ugly, but works
            TextStyle {
                font_size: GAME_OVER_FONT_SIZE,
                color: RED.into(),

                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            align_self: AlignSelf::Center,
            ..default()
        }),
    ));
    // Insert Coin
    commands.spawn((
        Overlay {
            game_state: GameState::InsertCoin,
        },
        TextBundle::from_section(
            "   Press Enter\n       to\n   Insert Coin", // Ugly, but works
            TextStyle {
                font_size: INSERT_COIN_FONT_SIZE,
                color: MAGENTA.into(),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            align_self: AlignSelf::Center,
            ..default()
        }),
    ));

    // Start
    commands.spawn((
        Overlay {
            game_state: GameState::Start,
        },
        TextBundle::from_section(
            "   Let's Go", // Ugly, but works
            TextStyle {
                font_size: START_FONT_SIZE,
                color: YELLOW.into(),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            align_self: AlignSelf::Center,
            ..default()
        }),
    ));

    // New Wave
    commands.spawn((
        Overlay {
            game_state: GameState::NewWave,
        },
        TextBundle::from_section(
            "   New Wave", // Ugly, but works
            TextStyle {
                font_size: NEW_WAVE_FONT_SIZE,
                color: YELLOW.into(),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            align_self: AlignSelf::Center,
            ..default()
        }),
    ));

    // Leader Board
    commands.spawn((
        Overlay {
            game_state: GameState::LeaderBoard,
        },
        TextBundle::from_section(
            " Leader Board", // Ugly, but works
            TextStyle {
                font_size: LEADER_BOARD_FONT_SIZE,
                color: DARK_CYAN.into(),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            align_self: AlignSelf::Center,
            ..default()
        }),
    ));
}

pub fn text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<Fps>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}

pub fn score_update_system(store: Res<Store>, mut query: Query<&mut Text, With<StatusBar>>) {
    for mut text in &mut query {
        text.sections[1].value = format!("{:1}  ", store.lives);
        text.sections[3].value = format!("{:1}  ", store.wave);
        text.sections[5].value = format!("{:06}", store.score);
    }
}

pub fn state_update_system(
    store: ResMut<Store>,
    mut game_state_query: Query<(&mut Visibility, &Overlay)>,
) {
    for (mut visibility, overlay) in &mut game_state_query {
        if overlay.game_state == store.game_state {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}
