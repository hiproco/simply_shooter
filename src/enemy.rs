use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

pub fn spawn_enemy(
    mut commad: Commands,
    asset_server: Res<AssetServer>,
) {
    commad
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("enemy.png"),
            transform: Transform::from_translation(Vec3::new(10.0, 10.0, 0.0)),
            ..Default::default()
        })
        .insert(Enemy);
    asset_server.watch_for_changes().unwrap();
}
