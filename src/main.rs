use std::{io::Write, iter};

use bevy::{
    ecs::system::EntityCommands,
    math::vec2,
    prelude::*,
    reflect::TypeRegistry,
    render::camera,
    sprite::collide_aabb::{collide, Collision},
    utils::HashMap,
};
use simply_shooter::{
    enemy::*,
    player::*,
    projectile::*,
    velocity::{velocity_system, Velocity},
};

//Just make a simple side scroll shooter

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RonAssetPlugin::<Projectile>::new(&["proj.ron"]))
        .add_startup_system(spawn_camera)
        .add_startup_system(startup)
        .add_startup_system(spawn_enemy)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(pressing_fire)
                .with_system(spawnbullet::<MissileModule>)
                .with_system(spawnbullet::<SideArm>),
        )
        .add_system(projectile)
        .add_system(timer)
        .add_system(player_movement)
        .add_system(bullet_life)
        .add_system(mouse_control)
        .add_system(velocity_system)
        .add_system(animate_wing)
        .add_system(enemy_hit)
        // .add_system(save_scene.exclusive_system())
        // .add_system(load_scene.exclusive_system())
        .run();
}

fn enemy_hit(
    proj: Query<(Entity, &Transform), With<Handle<Projectile>>>,
    enemy: Query<(Entity, &Transform), With<Enemy>>,
    mut command: Commands,
) {
    for p in proj.iter() {
        for e in enemy.iter() {
            if let Some(_) = collide(
                p.1.translation,
                vec2(10.0, 10.0),
                e.1.translation,
                vec2(10.0, 10.0),
            ) {
                command.entity(p.0).despawn_recursive();
                command.entity(e.0).despawn_recursive();
            }
        }
    }
}

fn load_scene(mut scene_spawner: ResMut<SceneSpawner>, asset_server: Res<AssetServer>) {
    scene_spawner.spawn(asset_server.load("scene/test.scn.ron"));
    asset_server.watch_for_changes().unwrap();
}

fn save_scene(world: &mut World) {
    let registry = world.get_resource::<TypeRegistry>().unwrap();
    let scene = DynamicScene::from_world(world, registry);
    let mut file = std::fs::File::create("assets/scene/test.scn.ron").unwrap();
    file.write(scene.serialize_ron(registry).unwrap().as_bytes())
        .unwrap();
}

fn timer(mut timers: Query<&mut Timer>, time: Res<Time>) {
    timers.iter_mut().for_each(|mut t| {
        t.tick(time.delta());
    });
}

fn spawn_camera(mut command: Commands) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scale *= 0.5;
    command.spawn_bundle(camera_bundle);
}

fn startup(
    mut command: Commands,
    asset_server: Res<AssetServer>,
    _scence_spawner: Res<SceneSpawner>,
) {
    asset_server.watch_for_changes().unwrap();
    // let children = command.spawn_batch(vec![
    //     ()
    // ]);
    // command.spawn_scene(asset_server.load("scene/test.scn.ron"));
    let mut textures = [
        "shootingcraft/gunmodule.png",
        "shootingcraft/left_wing.png",
        "shootingcraft/right_wing.png",
        "shootingcraft/missilemodule.png",
        "shootingcraft/halothingy.png",
        "shootingcraft/cockpit.png",
    ]
    .iter()
    .zip([5,1,1,1,1,1])
    .flat_map(|(&path,n)| iter::repeat(asset_server.load(path)).take(n));

    let offset = Vec3::X * 100.0;
    let mut e = textures
        .zip(
            (-2..3)
                .map(|m| Vec3::new(10.0 * m as f32, 42.0, 0.0))
                .chain([-offset, offset, Vec3::Y * 45.0, Vec3::ZERO, Vec3::ZERO]),
        )
        .map(|(texture, translation)| SpriteBundle {
            texture,
            transform: Transform::from_translation(translation),
            ..Default::default()
        })
        .map(|s| command.spawn_bundle(s).id())
        .collect::<Vec<_>>();
    for id in 0..5 {
        command.entity(e[id]).insert_bundle((SideArm, Timer::from_seconds(1.0, false)));
    }
    for id in 5..7 {
        command.entity(e[id]).insert(Wing);
    }
    for id in 7..8 {
        command
            .entity(e[id])
            .insert_bundle((MissileModule, Timer::from_seconds(0.2, false)));
    }
    let main = e.pop().unwrap();
    // side_arms.extend(e);
    command
        .entity(main)
        .insert(PlayerShip)
        .insert(Velocity(Vec3::ZERO))
        .push_children(&e);
}
