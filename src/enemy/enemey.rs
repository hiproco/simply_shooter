use bevy::prelude::*;
use super::Enemy;

pub fn spawn_enemy(mut commad: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commad.spawn().insert_bundle(SpriteBundle {
        material: materials.add(asset_server.load("enemy.png").into()),
        transform: Transform::from_translation(Vec3::new(10.0, 10.0, 0.0)),
        ..Default::default()
    }).insert(Enemy);
    asset_server.watch_for_changes().unwrap();
}