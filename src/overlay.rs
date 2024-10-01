//! It displays the current FPS in the top left corner, as well as text that changes color
//! in the bottom right.

use bevy::{
    color::palettes::css::GOLD,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::store::Store;

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct ScoreText;

pub fn setup(mut commands: Commands) {
    // Text with multiple sections
    commands.spawn((
        FpsText,
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font_size: 60.0,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 60.0,
                color: GOLD.into(),
                ..default()
            }),
        ]),
    ));

    // Text with multiple sections
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        ScoreText,
        TextBundle::from_sections([
            TextSection::new(
                "SCORE: ",
                TextStyle {
                    font_size: 60.0,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 60.0,
                color: GOLD.into(),
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(0.0),
            ..default()
        }),
    ));
}

pub fn text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}

pub fn score_update_system(store: Res<Store>, mut query: Query<&mut Text, With<ScoreText>>) {
    for mut text in &mut query {
        text.sections[1].value = format!("{:06}", store.score);
    }
}
