use crate::common::*;
use bevy::prelude::*;
use std::{default::Default, time::Instant};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum GameState {
    GameOver,
    InsertCoin,
    LeaderBoard,
    Start,
    Play,
}

#[derive(Resource)]
pub struct Store {
    pub texture_handler: Option<Handle<Image>>,
    pub instant: Instant,
    pub score: u32,
    pub aliens_killed: u8,
    pub alien_speed: f32,
    pub wave: u8,
    pub lives: u8,
    pub player_count_down: f32,
    pub game_state: GameState,
}

impl Default for Store {
    fn default() -> Self {
        Store {
            texture_handler: None,
            instant: Instant::now(),
            score: 0,
            aliens_killed: 0,
            alien_speed: ALIENS_SPEED_START,
            wave: 1,
            lives: 3,
            player_count_down: 3.0,
            game_state: GameState::InsertCoin,
        }
    }
}

impl Store {
    pub fn reset(&mut self) {
        *self = Self {
            texture_handler: self.texture_handler.clone(), // will this leak?
            ..default()
        }
    }
}

// Loads the drop sprite and stores handle in the Store resource
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Loads bullet sprite
    commands.insert_resource(Store {
        texture_handler: Some(asset_server.load("sprites/drop.png")),
        ..default()
    });
}
