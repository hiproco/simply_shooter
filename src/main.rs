use std::ops::{Add, Mul};

use bevy::{
    ecs::{
        schedule::ShouldRun,
        system::{Command, EntityCommands},
    },
    input::mouse::MouseButtonInput,
    math::{Vec3Swizzles, Vec4Swizzles},
    prelude::*,
    reflect::TypeUuid,
    render::camera::{Camera, OrthographicProjection, ScalingMode},
};
use bevy_asset_ron::RonAssetPlugin;
use serde::Deserialize;
use simply_shooter::{
    enemy::*,
    player::*,
    projectile::Projectile,
    projectile::*,
    velocity::{velocity_system, Velocity},
};

//Just make a simple side scroll shooter

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera.system())
        .add_plugin(RonAssetPlugin::<Projectile>::new(&["proj.ron"]))
        .add_startup_system(startup.system())
        .add_startup_system(spawn_enemy.system())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(pressing_fire.system())
                .with_system(spawnbullet::<MissileModule>.system())
                .with_system(spawnbullet::<SideArm>.system()),
        )
        .add_system(projectile.system())
        .add_system(timer.system())
        .add_system(player_movement.system())
        .add_system(bullet_life.system())
        .add_system(mouse_control.system())
        .add_system(velocity_system.system())
        .add_system(animate_wing.system())
        .run();
}

fn timer(mut timers: Query<(&mut Timer)>, time: Res<Time>) {
    timers.iter_mut().for_each(|mut t| {
        t.tick(time.delta());
    });
}

fn spawn_camera(mut command: Commands) {
    command.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn startup(
    mut command: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("shootingcraft/cockpit.png");
    asset_server.watch_for_changes().unwrap();
    command
        .spawn_bundle(SpriteBundle {
            material: materials.add(texture_handle.into()),
            transform: Transform::from_rotation(Quat::from_rotation_z(-1.57)),
            ..Default::default()
        })
        .insert(PlayerShip)
        .insert(Velocity(Vec3::ZERO))
        .with_children(|c| {
            let from_translation = |translation| Transform::from_translation(translation);
            let mut spawn_sidearm = |side_handle: Handle<Texture>, translation: Vec3| {
                c.spawn_bundle(SpriteBundle {
                    material: materials.add(side_handle.into()),
                    transform: from_translation(translation),
                    ..Default::default()
                })
                .insert_bundle((SideArm, Timer::from_seconds(1.0, false)));
            };
            let side_handle = asset_server.load("shootingcraft/gunmodule.png");
            let translation = Vec3::new(10.0, 42.0, 0.0);
            spawn_sidearm(side_handle.clone(), (translation - Vec3::X * 20.0).clamp_length_max(450.0));
            spawn_sidearm(side_handle.clone(), translation.clamp_length_max(450.0));
            spawn_sidearm(side_handle.clone(), (translation - Vec3::X * 30.0).clamp_length_max(450.0));
            spawn_sidearm(side_handle, (translation + Vec3::X * 10.0).clamp_length_max(450.0));
            let mut spawn_wing = |wing_handle: Handle<Texture>, translation: Vec3| {
                c.spawn_bundle(SpriteBundle {
                    material: materials.add(wing_handle.into()),
                    transform: from_translation(translation),
                    ..Default::default()
                })
                .insert(Wing);
            };

            let left_wing = asset_server.load("shootingcraft/left_wing.png");
            let right_wing = asset_server.load("shootingcraft/right_wing.png");
            let offset = Vec3::X * 100.0;
            spawn_wing(left_wing, -offset);
            spawn_wing(right_wing, offset);
            c.spawn_bundle(SpriteBundle {
                material: materials
                    .add(asset_server.load("shootingcraft/missilemodule.png").into()),
                transform: from_translation(Vec3::Y * 45.0),
                ..Default::default()
            })
            .insert_bundle((MissileModule, Timer::from_seconds(0.2, false)));
            c.spawn_bundle(SpriteBundle {
                material: materials.add(asset_server.load("shootingcraft/halothingy.png").into()),
                transform: Transform::from_translation(Vec3::ZERO),
                ..Default::default()
            });
        });
}
