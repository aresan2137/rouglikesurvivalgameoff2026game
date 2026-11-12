use bevy_ecs::prelude::*;
use glam::*;
use smq_engine::*;

use crate::renderer::DrawedObject;

const BULLET_SPEED: f32 = 25.0;
const BULLET_LIFESPAN: f32 = 1.0;

#[derive(Component)]
pub struct Bullet {
    pub lifespan: f32
}

pub fn bullet_provider(mut commands: Commands, time: Res<Delta>, mut bullets: Query<(Entity, &mut Bullet, &mut DrawedObject)>) {
    for (entity, mut bullet, mut obj) in bullets.iter_mut() {
        bullet.lifespan += time.delta;

        if bullet.lifespan >= BULLET_LIFESPAN {
            commands.entity(entity).despawn();
            continue;
        }

        let forward = obj.rotation * Vec3::Y;

        obj.position += forward * BULLET_SPEED * time.delta;
    }
}