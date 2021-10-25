use bevy::{ecs::schedule::ShouldRun, math::{Vec3Swizzles, Vec4Swizzles}, prelude::*, render::camera::Camera};

use super::PlayerShip;

pub fn pressing_fire(input: Res<Input<MouseButton>>) -> ShouldRun {
    input
        .pressed(MouseButton::Left)
        .then(|| ShouldRun::Yes)
        .unwrap_or(ShouldRun::No)
}

pub fn player_movement(
    mut player: Query<(&mut Transform), With<PlayerShip>>,
    input: Res<Input<KeyCode>>,
) {
    let (mut trans) = player.single_mut().expect("single player!");
    let axis = |pos_keys: &[KeyCode], neg_keys: &[KeyCode]| {
        let natural_axis = |keys: &[KeyCode]| -> f32 {
            keys.into_iter()
                .any(|k| input.pressed(*k))
                .then(|| 1.0)
                .unwrap_or_default()
        };
        natural_axis(pos_keys) - natural_axis(neg_keys)
    };
    trans.translation += Vec3::new(
        axis(&[KeyCode::D, KeyCode::Right], &[KeyCode::A, KeyCode::Left]),
        axis(&[KeyCode::W, KeyCode::Up], &[KeyCode::S, KeyCode::Down]),
        0.0,
    );
}


pub fn mouse_control(
    mut query: QuerySet<(
        Query<&Transform, With<Camera>>,
        Query<&mut Transform, With<PlayerShip>>,
    )>,
    wnd: Res<Windows>,
) {
    let pwnd = wnd.get_primary().unwrap();
    if let Some(c) = pwnd.cursor_position() {
        let size = Vec2::new(pwnd.width(), pwnd.height());
        let p = c - size / 2.0;
        let mc = query.q0().single().unwrap();
        let pos_wld = mc.compute_matrix() * p.extend(0.0).extend(1.0);
        let mut player = query.q1_mut().single_mut().expect("single player");
        player.rotation =
            Quat::from_rotation_z(Vec2::Y.angle_between(pos_wld.xy() - player.translation.xy()));
    }
}