use bevy::prelude::*;

// vintage television format
pub const RES_Y: f32 = 1080.0; // well a bit too modern
pub const RES_X: f32 = RES_Y * 4.0 / 3.0;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: Vec2 = Vec2::new(64.0, 40.0);
pub const PLAYER_HEIGHT: f32 = 50.0; // There should be a way to get this from sprite
pub const LAZER_SPEED: f32 = 1000.0;

pub const SCENE_WIDTH: f32 = RES_X / 2.0 - 100.0;
pub const SCENE_HEIGHT: f32 = RES_Y / 2.0 - 50.0;
pub const ALIENS_COL: usize = 11;
pub const ALIENS_ROW: usize = 5;
pub const ALIENS_TOTAL: u8 = ALIENS_COL as u8 * ALIENS_ROW as u8;
pub const ALIENS_SPACE: f32 = 80.0; // used for layout
pub const ALIEN_SIZE: Vec2 = Vec2::new(64.0, 40.0); // used for hit box
pub const ALIEN_BULLET_SPEED: f32 = 300.0;
pub const BUNKERS: usize = 5;
pub const BUNKER_SPACE: f32 = SCENE_WIDTH / BUNKERS as f32;
pub const BUNKERS_Y: f32 = 150.0;
pub const BUNKER_SIZE: Vec2 = Vec2::new(16.0, 16.0);

pub const ALIENS_SPEED: f32 = 30.0;

pub const SCORE_ALIEN: u32 = 10;
