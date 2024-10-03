use crate::{
    alien::*,
    bunker::*,
    common::*,
    game_state::{GameState, StateTransitionTimer, Store},
    lazer::Lazer,
    player::Player,
};
use bevy::prelude::*;
use std::time::Duration;

#[allow(clippy::too_many_arguments)]
pub fn update_system(
    mut store: ResMut<Store>,
    mut commands: Commands,
    mut timer_query: Query<&mut StateTransitionTimer>,
    alien_query: Query<(Entity, &Transform), With<Alien>>,
    mut lazer_query: Query<(&mut Lazer, &Transform)>,
    mut bunker_query: Query<(&mut TextureAtlas, Entity, &Transform), With<Bunker>>,
    alien_bullet_query: Query<(Entity, &Transform), With<AlienBullet>>,
    mut player_query: Query<(&mut Player, &Transform), With<Player>>,
) {
    // check if point:&Transform is in &target:Transform with size:Vec2
    #[inline(always)]
    fn in_rect(point: &Transform, target: &Transform, size: Vec2) -> bool {
        let t_vec: Vec2 = (target.translation.x, target.translation.y).into();
        let p_vec: Vec2 = (point.translation.x, point.translation.y).into();
        let rect = Rect::from_center_size(t_vec, size);
        rect.contains(p_vec)
    }

    let mut timer = timer_query.single_mut();

    // get a player singleton
    let (mut player, player_transform) = player_query.single_mut();
    // alien bullets
    for (bullet_entity, bullet_transform) in &alien_bullet_query {
        // hit player
        if in_rect(bullet_transform, player_transform, PLAYER_SIZE) {
            commands.entity(bullet_entity).despawn();
            if player.spawn_counter == 0 && store.lives > 0 {
                store.lives -= 1;
                if store.lives == 0 {
                    store.game_state = GameState::GameOver;
                    timer.set_duration(Duration::from_secs_f32(STATE_TRANSITION_DURATION));
                    timer.reset();
                } else {
                    player.spawn_counter = PLAYER_SPAWN_COUNTER;
                }
            }
        }
        // hit bunker?
        for (bunker_atlas, bunker_entity, bunker_transform) in &mut bunker_query {
            if in_rect(bullet_transform, bunker_transform, BUNKER_SIZE) {
                commands.entity(bullet_entity).despawn();
                if store.game_state == GameState::Play {
                    hit_bunker(&mut commands, bunker_entity, bunker_atlas);
                }
            }
        }
    }

    // get lazer singleton
    let (mut lazer, lazer_transform) = lazer_query.get_single_mut().unwrap();

    if let Lazer::Fired(_) = *lazer {
        // check bunkers
        for (atlas, entity, bunker_transform) in &mut bunker_query {
            if in_rect(lazer_transform, bunker_transform, BUNKER_SIZE) {
                hit_bunker(&mut commands, entity, atlas);
                *lazer = Lazer::Idle;
            }
        }

        // check aliens
        for (alien_entity, enemy_transform) in &alien_query {
            // Collision check
            if in_rect(lazer_transform, enemy_transform, ALIEN_SIZE) {
                commands.entity(alien_entity).despawn();
                *lazer = Lazer::Idle;
                store.aliens_killed += 1;
                store.alien_speed += ALIENS_SPEED_KILL;
                store.score += SCORE_ALIEN;
                if store.aliens_killed == ALIENS_TOTAL {
                    debug!("-- new wave --");
                    store.aliens_killed = 0;
                    store.game_state = GameState::NewWave;
                    timer.reset();
                }
            }
        }
    }
}
