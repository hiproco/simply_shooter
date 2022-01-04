use bevy::{math::Vec3, prelude::{Query, With, Transform}};

use crate::player::PlayerShip;


pub struct Velocity(pub Vec3);

pub fn velocity_system(mut query:Query<(&mut Velocity, &mut Transform), With<PlayerShip>>) {
    let (mut velocity, mut transform) = query.single_mut().expect("only single player");
    transform.translation += velocity.0;
    velocity.0 *= 0.9;
}