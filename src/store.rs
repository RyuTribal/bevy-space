use bevy::prelude::*;
use std::time::Instant;

#[derive(Resource)]
pub struct Store {
    pub texture_handler: Handle<Image>,
    pub instant: Instant,
    pub score: u32,
    pub aliens_killed: u8,
}

// Loads the drop sprite and stores handle in the Store resource
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Loads bullet sprite
    let texture_handler: Handle<Image> = asset_server.load("sprites/drop.png");
    commands.insert_resource(Store {
        texture_handler,
        instant: Instant::now(),
        score: 0,
        aliens_killed: 0,
    });
}
