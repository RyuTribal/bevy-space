use crate::{alien::*, bunker::*, common::*, lazer::Lazer, player::Player, store::Store};
use bevy::prelude::*;

pub fn hit_detection(
    mut store: ResMut<Store>,
    mut commands: Commands,
    alien_query: Query<(Entity, &Transform), With<Alien>>,
    mut lazer_query: Query<(&mut Lazer, &Transform)>,
    mut bunker_query: Query<(&mut TextureAtlas, Entity, &Transform), With<Bunker>>,
    alien_bullet_query: Query<(Entity, &Transform), With<AlienBullet>>,
    player_query: Query<&Transform, With<Player>>,
) {
    // check if point:&Transform is in &target:Transform with size:Vec2
    #[inline(always)]
    fn in_rect(point: &Transform, target: &Transform, size: Vec2) -> bool {
        let t_vec: Vec2 = (target.translation.x, target.translation.y).into();
        let p_vec: Vec2 = (point.translation.x, point.translation.y).into();
        let rect = Rect::from_center_size(t_vec, size);
        rect.contains(p_vec)
    }

    #[inline(always)]
    fn hit_bunker(commands: &mut Commands, entity: Entity, mut atlas: Mut<TextureAtlas>) {
        if atlas.index < 10 {
            atlas.index += 5;
        } else {
            commands.entity(entity).despawn();
        }
    }

    // get a player_transform singleton
    let mut player_iterator = player_query.iter();
    let player_transform = player_iterator.next().unwrap();
    assert!(player_iterator.next().is_none());

    // alien bullets
    for (bullet_entity, bullet_transform) in &alien_bullet_query {
        // hit player
        if in_rect(bullet_transform, player_transform, PLAYER_SIZE) {
            error!("you died");
            commands.entity(bullet_entity).despawn();
        }
        // hit bunker?
        for (bunker_atlas, bunker_entity, bunker_transform) in &mut bunker_query {
            if in_rect(bullet_transform, bunker_transform, BUNKER_SIZE) {
                hit_bunker(&mut commands, bunker_entity, bunker_atlas);
                commands.entity(bullet_entity).despawn();
            }
        }
    }

    // get lazer singleton
    let mut lazer_iterator = lazer_query.iter_mut();
    let (mut lazer, lazer_transform) = lazer_iterator.next().unwrap();
    assert!(lazer_iterator.next().is_none());

    if *lazer == Lazer::Fired {
        // check bunkers
        for (atlas, entity, bunker_transform) in &mut bunker_query {
            if in_rect(lazer_transform, bunker_transform, BUNKER_SIZE) {
                hit_bunker(&mut commands, entity, atlas);
                *lazer = Lazer::Idle;
            }
        }

        // check aliens
        for (entity, enemy_transform) in &alien_query {
            // Collision check
            if in_rect(lazer_transform, enemy_transform, ALIEN_SIZE) {
                commands.entity(entity).despawn();
                *lazer = Lazer::Idle;
                store.aliens_killed += 1;
                store.alien_speed += ALIENS_SPEED_KILL;
                store.score += SCORE_ALIEN;
            }
        }
    }
}
