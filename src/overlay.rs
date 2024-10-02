//! It displays the current FPS in the top left corner and score top right

use bevy::{
    color::palettes::css::{GOLD, MAGENTA, RED},
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::{
    common::*,
    store::{GameState, Store},
};

//
#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct StatusBar;

#[derive(Component)]
pub struct GameOver;

#[derive(Component)]
pub struct InsertCoin;

pub fn setup(mut commands: Commands) {
    // FPS
    commands.spawn((
        FpsText,
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
    spawn_insert_coin(&mut commands);
}

pub fn text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
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

pub fn spawn_game_over(commands: &mut Commands) {
    // GameOver
    commands.spawn((
        GameOver,
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
}

pub fn spawn_insert_coin(commands: &mut Commands) {
    // Insert Coin
    commands.spawn((
        InsertCoin,
        TextBundle::from_section(
            "   Press Space\n       to\n   Insert Coin", // Ugly, but works
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
}

pub fn state_update_system(
    store: ResMut<Store>,
    mut commands: Commands,

    insert_coin_query: Query<Entity, With<InsertCoin>>,
    game_over_query: Query<Entity, With<GameOver>>,
) {
    match store.game_state {
        GameState::Play => {
            let _ = insert_coin_query
                .get_single()
                .map(|entity| commands.entity(entity).despawn());
            let _ = game_over_query
                .get_single()
                .map(|entity| commands.entity(entity).despawn());
        }
        _ => {}
    }
}
