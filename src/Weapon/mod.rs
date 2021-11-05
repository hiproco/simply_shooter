use bevy::prelude::{Commands, Entity, EventReader, Query, QuerySet, With};
use heron::CollisionEvent;

use crate::enemy::Enemy;

pub struct Projectile;

pub struct Launcher<P>(P);

pub fn hit(
    mut collision: EventReader<CollisionEvent>,
    e: QuerySet<(Query<Entity, With<Enemy>>, Query<Entity, With<Projectile>>)>,
    mut c: Commands,
) {
    for col in collision.iter() {
        let (r,l) = col.collision_shape_entities();
        c.entity(r).despawn();
        c.entity(l).despawn();
    }
}
