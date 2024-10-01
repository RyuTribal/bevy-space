use bevy::prelude::*;
use std::time::Instant;

#[derive(Resource)]
pub struct Store {
    pub texture_handler: Handle<Image>,
    pub instant: Instant,
    pub score: u32,
    pub aliens_killed: u8,
}
