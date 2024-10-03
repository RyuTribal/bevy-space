use crate::common::*;
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct Particle {
    timer: Timer,
    delta: Vec2,
}

pub fn update_system(
    time: Res<Time>,
    mut commands: Commands,

    mut bullet_query: Query<(Entity, &mut Sprite, &mut Transform, &mut Particle)>,
) {
    for (entity, mut sprite, mut transform, mut particle) in &mut bullet_query {
        //for (entity, mut transform, bullet) in &mut bullet_query {
        particle.timer.tick(time.delta());

        let ratio = 1.0 - particle.timer.elapsed().as_secs_f32() / PARTICLE_DURATION;

        sprite.color.set_alpha(ratio);

        // maybe a rect here
        if particle.timer.just_finished() {
            info!("particle despawn");
            commands.entity(entity).despawn();
        } else {
            let translation = &mut transform.translation;
            translation.x += particle.delta.x * time.delta_seconds();
            translation.y += particle.delta.y * time.delta_seconds();
        }
    }
}

pub fn spawn_particle(mut commands: Commands, image: Res<BulletImage>, pos: Vec2, delta: Vec2) {
    commands.spawn((
        Particle {
            delta,
            timer: Timer::new(Duration::from_secs_f32(PARTICLE_DURATION), TimerMode::Once),
        },
        SpriteBundle {
            texture: image.0.clone(),
            transform: Transform::from_xyz(pos.x, pos.y, 0.0),
            ..default()
        },
    ));
}

#[derive(Resource)]
pub struct BulletImage(Handle<Image>);

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Loads bullet sprite and store resource
    commands.insert_resource(BulletImage(asset_server.load("sprites/cross.png")))
}
