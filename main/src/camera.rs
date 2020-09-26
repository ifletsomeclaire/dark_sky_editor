use bevy::{
    input::mouse::MouseMotion, input::mouse::MouseWheel, math::vec3, prelude::*,
    render::camera::OrthographicProjection,
};

use crate::{material::GlobalMaterial, material::MeshMaterial, Handles};

#[derive(Default)]
pub struct MouseState {
    pub mouse_motion_event_reader: EventReader<MouseMotion>,
    pub mouse_wheel_event_reader: EventReader<MouseWheel>,
}
#[derive(Debug, Default)]
pub struct CameraMarker;
pub fn camera_fucking_blows(
    key: Res<Input<KeyCode>>,

    mut query: Query<(&CameraMarker, &mut OrthographicProjection)>,
) {
    for (_, mut p) in &mut query.iter() {
        if key.pressed(KeyCode::F1) {
            println!(" BEFORE {:#?}", p);

            p.near += 100.0;
            println!(" AFTER {:#?}", p);
        }
        if key.pressed(KeyCode::F2) {
            println!(" BEFORE {:#?}", p);
            p.bottom -= 100.0;
            p.top += 100.0;
            p.right += 100.0;
            p.left -= 100.0;
            println!(" AFTER {:#?}", p);
        }
        if key.pressed(KeyCode::F3) {
            p.right += 100.0;
        }
        if key.pressed(KeyCode::F4) {
            p.near += 100.0;
        }
        if key.pressed(KeyCode::F5) {}
    }
}
pub fn camera_movement(
    click: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
    mouse_move: Res<Events<MouseMotion>>,
    mouse_wheel_events: Res<Events<MouseWheel>>,
    mut state: ResMut<MouseState>,
    mut query: Query<(&CameraMarker, &mut Transform)>,
) {
    if key.pressed(KeyCode::LShift) {
        for event in state.mouse_motion_event_reader.iter(&mouse_move) {
            for (_, mut trans) in &mut query.iter() {
                trans.rotate(Quat::from_rotation_x(event.delta[1] * 0.01).normalize());
                trans.rotate(Quat::from_rotation_y(event.delta[0] * 0.01).normalize());
            }
        }
    } else {
        if click.pressed(MouseButton::Right) {
            for event in state.mouse_motion_event_reader.iter(&mouse_move) {
                for (_, mut trans) in &mut query.iter() {
                    let rot = trans.rotation();
                    trans.translate(rot.mul_vec3(vec3(-event.delta[0], event.delta[1], 0.0)))
                }
            }
        }
        if click.pressed(MouseButton::Middle) {
            for event in state.mouse_motion_event_reader.iter(&mouse_move) {
                for (_, mut trans) in &mut query.iter() {
                    let rot = trans.rotation();
                    trans.translate(rot.mul_vec3(vec3(0.0, 0.0, -event.delta[1] * 10.)));
                }
            }
        }
        for event in state.mouse_wheel_event_reader.iter(&mouse_wheel_events) {
            for (_, mut trans) in &mut query.iter() {
                let rot = trans.rotation();
                trans.translate(rot.mul_vec3(vec3(0.0, 0.0, -event.y * 300.)))
            }
        }
    }
}

pub fn update_camera_distance(
    handle_res: Res<Handles>,
    // mut globalmat: ResMut<Assets<GlobalMaterial>>,
    mut meshmat: ResMut<Assets<MeshMaterial>>,
    mut query: Query<(&CameraMarker, &Transform)>,
) {
    if let Some(m_mat) = meshmat.get_mut(&handle_res.ship_texture_mat) {
        for (_, transform) in &mut query.iter() {
            // g_mat.distance = transform.translation().z();
            m_mat.distance = transform.translation().z();
            // println!("distance: {}", m_mat.distance);
        }
    }
}
