use crate::common::*;
use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct Bunker;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Builds and spawns the bunker sprites
    let texture = asset_server.load("sprites/defense.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 3, 3, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // The sprite index layout of the bunker
    let bunker_matrix = [
        [0, 1, 1, 1, 1, 2],
        [1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 1],
    ];

    for b in 0..BUNKERS {
        let mut bunker = vec![];
        for (r, row) in bunker_matrix.iter().enumerate() {
            for (c, data) in row.iter().enumerate() {
                bunker.push((
                    Bunker,
                    SpriteBundle {
                        transform: Transform::from_xyz(
                            (c as f32 - (row.len() as f32 - 1.0) / 2.0) * 16.0
                                + (2.0 * b as f32 - (BUNKERS as f32 - 1.0)) * BUNKER_SPACE,
                            BUNKERS_Y - SCENE_HEIGHT - (r as f32) * 16.0,
                            0.0,
                        ),
                        texture: texture.clone(),
                        ..default()
                    },
                    TextureAtlas {
                        layout: texture_atlas_layout.clone(),
                        index: *data,
                    },
                ));
            }
        }
        commands.spawn_batch(bunker);
    }
}
