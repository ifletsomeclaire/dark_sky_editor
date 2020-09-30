use bevy::{
    input::mouse::MouseMotion, input::mouse::MouseWheel, math::vec3, prelude::*,
    render::camera::Camera, render::camera::CameraProjection,
    render::camera::OrthographicProjection,
};

pub struct Main2dCamera;

impl Plugin for Main2dCamera {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<MouseState>()
            .add_startup_system(add_camera.system())
            .add_system(zoom.system())
            .add_system(camera_movement.system());
    }
}

fn add_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dComponents {
            transform: Transform::new(Mat4::face_toward(
                Vec3::new(0.0, -1000.01, 10000.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            )),
            orthographic_projection: OrthographicProjection {
                far: f32::MAX,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(CameraMarker);
}

#[derive(Default)]
pub struct MouseState {
    pub mouse_motion_event_reader: EventReader<MouseMotion>,
    pub mouse_wheel_event_reader: EventReader<MouseWheel>,
}
#[derive(Debug, Default)]
pub struct CameraMarker;
fn zoom(
    key: Res<Input<KeyCode>>,
    mut query: Query<(&CameraMarker, &mut OrthographicProjection, &mut Camera)>,
) {
    for (_, mut p, mut cam) in &mut query.iter() {
        if key.pressed(KeyCode::F1) {
            p.bottom *= 0.99;
            p.top *= 0.99;
            p.right *= 0.99;
            p.left *= 0.99;
            cam.projection_matrix = p.get_projection_matrix();
        }
        if key.pressed(KeyCode::F2) {
            p.bottom *= 1.01;
            p.top *= 1.01;
            p.right *= 1.01;
            p.left *= 1.01;
            cam.projection_matrix = p.get_projection_matrix();
        }
    }
}
fn camera_movement(
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
