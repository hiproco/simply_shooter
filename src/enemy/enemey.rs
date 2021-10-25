use super::Enemy;
use bevy::prelude::*;

pub fn spawn_enemy(
    mut command: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut c = 0.0;
    let material = materials.add(asset_server.load("enemy.png").into());
    asset_server.watch_for_changes().unwrap();
    #[derive(Bundle)]
    struct EnemyBundle {
        #[bundle]
        sprite_bundle: SpriteBundle,
        enemy: Enemy,
    }
    command.spawn_batch(
        std::iter::from_fn(move || {
            c += 16.0;
            Some(EnemyBundle {
                sprite_bundle: SpriteBundle {
                    material: material.clone(),
                    transform: Transform::from_translation(Vec3::new(c, 0.0, 0.0)),
                    ..Default::default()
                },
                enemy: Enemy,
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
