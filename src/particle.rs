use crate::common::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Particle(Vec2);

pub fn particle_update_system(
    time: Res<Time>,
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &mut Transform, &Particle)>,
) {
    for (entity, mut transform, bullet) in &mut bullet_query {
        let translation = &mut transform.translation;
        // maybe a rect here
        if translation.y < -SCENE_HEIGHT
            || translation.x > SCENE_HEIGHT
            || translation.x < -SCENE_WIDTH
            || translation.x > SCENE_WIDTH
        {
            info!("particle despawn");
            commands.entity(entity).despawn();
        } else {
            translation.x -= bullet.0.x * time.delta_seconds();
            translation.y -= bullet.0.y * time.delta_seconds();
        }
    }
}

pub fn spawn_particle(mut commands: Commands, image: Res<BulletImage>, pos: Vec2, delta: Vec2) {
    commands.spawn((
        Particle(delta),
        SpriteBundle {
            texture: image.0.clone(),
            transform: Transform::from_xyz(pos.x, pos.y, 0.0),
            visibility: Visibility::Hidden,
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
