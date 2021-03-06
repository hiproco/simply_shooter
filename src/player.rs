use bevy::{
    ecs::schedule::ShouldRun,
    math::{Vec3Swizzles, Vec4Swizzles},
    prelude::*,
    render::camera::Camera,
};

use crate::velocity::Velocity;


#[derive(Debug, Clone, Copy, Component)]
pub struct PlayerShip;

#[derive(Debug, Clone, Copy, Component)]
pub struct SideArm;

#[derive(Component)]
pub struct Wing;

#[derive(Component)]
pub struct MissileModule;

pub fn pressing_fire(input: Res<Input<MouseButton>>) -> ShouldRun {
    input
        .pressed(MouseButton::Left)
        .then(|| ShouldRun::Yes)
        .unwrap_or(ShouldRun::No)
}

pub fn player_movement(
    mut player: Query<&mut Velocity, With<PlayerShip>>,
    input: Res<Input<KeyCode>>,
) {
    let mut velocity = player.single_mut();
    let axis = |pos_keys: &[KeyCode], neg_keys: &[KeyCode]| {
        let natural_axis = |keys: &[KeyCode]| -> f32 {
            keys.into_iter()
                .any(|k| input.pressed(*k))
                .then(|| 1.0)
                .unwrap_or_default()
        };
        natural_axis(pos_keys) - natural_axis(neg_keys)
    };
    velocity.0 = (velocity.0
        + Vec3::new(
            axis(&[KeyCode::D, KeyCode::Right], &[KeyCode::A, KeyCode::Left]),
            axis(&[KeyCode::W, KeyCode::Up], &[KeyCode::S, KeyCode::Down]),
            0.0,
        ))
    .clamp_length_max(Velocity::MAX);
}

pub fn mouse_control(
    mut query: QuerySet<(
        QueryState<&Transform, With<Camera>>,
        QueryState<&mut Transform, With<PlayerShip>>,
    )>,
    wnd: Res<Windows>,
) {
    let pwnd = wnd.get_primary().unwrap();
    if let Some(c) = pwnd.cursor_position() {
        let size = Vec2::new(pwnd.width(), pwnd.height());
        let p = c - size / 2.0;
        let mc = query.q0().single();
        let pos_wld = mc.compute_matrix() * p.extend(0.0).extend(1.0);
        let mut q1 = query.q1();
        let mut player = q1.single_mut();
        player.rotation = Quat::from_rotation_z(Vec2::Y.angle_between(pos_wld.xy() - player.translation.xy()));
    }
}

pub fn animate_wing(
    mut query: QuerySet<(
        QueryState<&mut Transform, With<Wing>>,
        QueryState<(&Velocity, &Transform), With<PlayerShip>>,
    )>,
) {
    let (pv, pt) = query.q1().single();
    let unnormilized_length = pv.0.length();
    let normilized_length = unnormilized_length / Velocity::MAX;
    let forward = pt.local_y().dot(pv.0).is_sign_positive();
    for mut wing in query.q0().iter_mut() {
        let right = wing.translation.x.is_sign_positive();
        let rotate = match (right, forward) {
            (true, true) | (false, false) => -normilized_length,
            (false, true) | (true, false) => normilized_length,
        };
        let length = wing.translation.length();
        let translation = (wing.local_x() * length + wing.local_y() * unnormilized_length)
            .clamp_length_max(length);
        wing.translation = if right { translation } else { -translation };
        wing.rotation = Quat::from_rotation_z(rotate / 2.0);
    }
}
