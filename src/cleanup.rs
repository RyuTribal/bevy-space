use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    alien,
    bunker::{self, Bunker},
    store::*,
};

pub fn cleanup_state<T>(commands: &mut Commands, query: Query<Entity, With<T>>)
where
    T: Component,
{
    for item in &query {
        commands.entity(item).despawn_recursive();
    }
}

// #[derive(SystemParam)]
// pub struct Cleanup<'w, 's> {
//     commands: Commands<'w, 's>,
//     bunkers: Query<'w, 's, Entity, With<Bunker>>,
//     aliens: Query<'w, 's, Entity, With<Alien>>,
//     alien_bullets: Query<'w, 's, Entity, With<AlienBullet>>,
// }

// impl<'w, 's> Cleanup<'w, 's> {
//     pub fn cleanup(&mut self) {
//         for item in &self.bunkers {
//             self.commands.entity(item).despawn_recursive();
//         }
//         for item in &self.aliens {
//             self.commands.entity(item).despawn_recursive();
//         }
//         for item in &self.alien_bullets {
//             self.commands.entity(item).despawn_recursive();
//         }
//     }
// }

// cleanup system
// pub fn cleanup_system(mut store: ResMut<Store>, mut cleanup: Cleanup) {
//     if store.game_state == GameState::GameOver {
//         cleanup.cleanup();
//         store.game_state = GameState::InsertCoin;
//     }
// }

pub fn cleanup_system(
    mut commands: Commands,
    mut store: ResMut<Store>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    alien_query: Query<Entity, With<alien::Alien>>,
    alien_bullet_query: Query<Entity, With<alien::AlienBullet>>,
    bunker_query: Query<Entity, With<Bunker>>,
) {
    if store.game_state == GameState::GameOver {
        println!("--- reset ---");
        alien::reset(
            &mut commands,
            &asset_server,
            &mut texture_atlas_layout,
            alien_query,
            alien_bullet_query,
        );
        bunker::reset(
            &mut commands,
            &asset_server,
            &mut texture_atlas_layout,
            bunker_query,
        );
        store.game_state = GameState::InsertCoin;
    }
}
