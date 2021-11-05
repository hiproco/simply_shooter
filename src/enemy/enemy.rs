use super::Enemy;
use bevy::prelude::*;
use heron::{prelude::*, rapier_plugin::nalgebra};

#[derive(Bundle)]
struct EnemyBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    enemy: Enemy,
    collider: RigidBody,
    shape: CollisionShape,
}

pub fn spawn_enemy(
    mut command: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut c = 0.0;
    let material = materials.add(asset_server.load("enemy.png").into());
    asset_server.watch_for_changes().unwrap();
    command.spawn_batch(
        std::iter::from_fn(move || {
            c += 32.0;
            Some(EnemyBundle {
                sprite_bundle: SpriteBundle {
                    material: material.clone(),
                    transform: Transform::from_translation(Vec3::new(c, 0.0, 0.0)),
                    ..Default::default()
                },
                enemy: Enemy,
                collider: RigidBody::Sensor,
                shape: CollisionShape::Cuboid { half_extends: Vec3::new(8.0, 8.0, 0.0), border_radius: None },
            })
        })
        .take(5),
    );
    
    // command
    //     .spawn()
    //     .insert_bundle(SpriteBundle {
    //         material: materials.add(asset_server.load("enemy.png").into()),
    //         transform: Transform::from_translation(Vec3::new(10.0, 10.0, 0.0)),
    //         ..Default::default()
    //     })
    //     .insert(Enemy);
}
