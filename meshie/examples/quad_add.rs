use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin},
    math::vec2,
    math::vec3,
    prelude::*,
};
use meshie::{add_mesh, reverse_triangles, translate_mesh};

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(PrintDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        // light
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(40.0, -40.0, 50.0)),
            ..Default::default()
        })
        // camera
        .spawn(Camera3dComponents {
            transform: Transform::new(Mat4::face_toward(
                Vec3::new(0.0, -50.0, 150.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            )),
            ..Default::default()
        });

    let mut mesh = Mesh::from(shape::Cube { size: 10.0 });
    let mut mesh_two = Mesh::from(shape::Quad {
        size: vec2(100., 100.),
        flip: false,
    });
    translate_mesh(&mut mesh_two, vec3(10., 0.0, -5.0));
    add_mesh(&mut mesh, &mesh_two);
    // reverse_triangles(&mut mesh);
    let cube_handle = meshes.add(mesh);
    commands.spawn(PbrComponents {
        mesh: cube_handle,
        material: materials.add(StandardMaterial {
            ..Default::default()
        }),
        // transform: Transform::from_translation(Vec3::new(
        //     rng.gen_range(-50.0, 50.0),
        //     rng.gen_range(-50.0, 50.0),
        //     0.0,
        // )),
        ..Default::default()
    });
}
