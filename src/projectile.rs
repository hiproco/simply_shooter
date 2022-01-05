use bevy::ecs::component;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use serde::Deserialize;

#[derive(Deserialize, Default, TypeUuid)]
#[uuid = "3081fd4d-e6da-438b-bb18-97237c0e0407"]
pub struct Projectile(f32);

struct Damage(i32);



pub fn bullet_life(bullet: Query<(Entity, &Timer), With<Handle<Projectile>>>, mut command: Commands) {
    bullet.for_each_mut(|(e, t)| {
        if t.finished() {
            command.entity(e).despawn_recursive();
        }
    });
}

pub fn spawnbullet<Launcher: component::Component>(
    mut command: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut player: Query<(&mut Timer, &GlobalTransform), With<Launcher>>,
) {
    let texture_handle = asset_server.load("bullet.png");
    let proj_handle: Handle<Projectile> = asset_server.load("bullet.proj.ron");

    for (mut timer, trans) in player.iter_mut() {
        timer.finished().then(|| {
            command
                .spawn_bundle(SpriteBundle {
                    material: materials.add(texture_handle.clone().into()),
                    transform: Transform::from_matrix(trans.compute_matrix()),
                    ..Default::default()
                })
                .insert_bundle((proj_handle.clone(), Timer::from_seconds(5.0, false)));
            timer.reset();
        });
    }
}

pub fn projectile(
    mut bullet: Query<(&mut Transform, &Handle<Projectile>)>,
    projectile: ResMut<Assets<Projectile>>,
) {
    for (mut trans, proj) in bullet.iter_mut() {
        let local = trans.local_y();
        trans.translation += local * projectile.get(proj).unwrap_or(&Projectile::default()).0;
    }
}
