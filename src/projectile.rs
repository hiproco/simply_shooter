use std::{marker::PhantomData, error::Error};

use bevy::{
    asset::{AssetLoader, LoadedAsset},
    ecs::component,
    prelude::*,
    reflect::TypeUuid,
};
use ron;
use serde::Deserialize;

#[derive(Deserialize, Default, TypeUuid, Component)]
#[uuid = "3081fd4d-e6da-438b-bb18-97237c0e0407"]
pub struct Projectile(f32);

#[derive(Default)]
pub struct RonLoader<A>(PhantomData<A>, &'static [&'static str]);

impl<A> AssetLoader for RonLoader<A>
where
    for<'de> A: std::marker::Send + std::marker::Sync + Deserialize<'de> + TypeUuid + 'static,
{
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::asset::BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
        Box::pin(async move {
            let asset = ron::de::from_bytes::<A>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        self.1
    }
}

pub struct RonAssetPlugin<T>(PhantomData<T>, &'static [&'static str]);

impl<T> RonAssetPlugin<T> {
    pub fn new(extension: &'static [&'static str]) -> Self {
        Self(PhantomData::<T>, extension)
    }
}

impl<T> Plugin for RonAssetPlugin<T> 
where 
    for<'de> T: Send + Sync + Deserialize<'de> + TypeUuid + 'static {
    fn build(&self, app: &mut App) {
        app
        .add_asset::<T>()
        .add_asset_loader(RonLoader::<T>(PhantomData::<T>, self.1));
    }

    fn name(&self) -> &str {
        "ron loader"
    }
}

#[allow(dead_code)]
struct Damage(i32);

pub fn bullet_life(
    mut bullet: Query<(Entity, &Timer), With<Handle<Projectile>>>,
    mut command: Commands,
) {
    bullet.for_each_mut(|(e, t)| {
        if t.finished() {
            command.entity(e).despawn_recursive();
        }
    });
}

pub fn spawnbullet<Launcher: component::Component>(
    mut command: Commands,
    asset_server: Res<AssetServer>,
    mut player: Query<(&mut Timer, &GlobalTransform), With<Launcher>>,
) {
    let texture_handle = asset_server.load("bullet.png");

    let projectile = asset_server.load::<Projectile, _>("bullet.proj.ron");

    for (mut timer, trans) in player.iter_mut() {
        let (_, rotation, translation) = trans.compute_matrix().to_scale_rotation_translation();
        timer.finished().then(|| {
            command
                .spawn_bundle(SpriteBundle {
                    texture: texture_handle.clone(),
                    transform: Transform::from_rotation(rotation).with_translation(translation),
                    ..Default::default()
                })
                .insert_bundle((projectile.clone(), Timer::from_seconds(5.0, false)));
            timer.reset();
        });
    }
}

pub fn projectile(
    mut bullet: Query<(&mut Transform, &Handle<Projectile>)>,
    projectile: Res<Assets<Projectile>>,
) {
    for (mut trans, proj) in bullet.iter_mut() {
        let local = trans.local_y();
        trans.translation += local * projectile.get(proj).unwrap_or(&Projectile(10.0)).0;
    }
}
