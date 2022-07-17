use bevy::{
    math::Vec3,
    prelude::{Query, Transform, With, Component},
};

use crate::player::PlayerShip;

#[derive(Component)]
pub struct Velocity(pub Vec3);

impl Velocity {
    pub const MAX: f32 = 10.0;
}

pub fn velocity_system(mut query: Query<(&mut Velocity, &mut Transform), With<PlayerShip>>) {
    let (mut velocity, mut transform) = query.single_mut();
    transform.translation += velocity.0;
    velocity.0 *= 0.9;
}
