use bevy::{diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin}, math::vec2, math::vec3, prelude::*, render::camera::PerspectiveProjection};
use meshie::{generator::DistributionFn, generator::MeshBuilder, generator::MeshConfig};
// use rand::{FromEntropy, Rng, StdRng};

fn main() {
    App::build()
        .add_default_plugins()
        .add_resource(ClearColor(Color::BLACK))
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(PrintDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
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
                Vec3::new(0.0, -15.0, 1500.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            )),
            perspective_projection: PerspectiveProjection {
                far: 100000., ..Default::default()
            },
            ..Default::default()
        });

    let mat_handle = asset_server
        .load("../assets/STSCI-H-p1917b-q-5198x4801.png")
        .unwrap();
    let mut mesh_builder = MeshBuilder {
        texture_size: vec2(5198., 4807.),
        config: vec![],
    };
    mesh_builder.config.push(MeshConfig {
        count: 100,
        texture_position: bevy::sprite::Rect {
            min: vec2(592., 863.),
            max: vec2(601., 871.),
        },
        area: vec3(1000., 1000., 10.),
        distribution: DistributionFn::Random,
    });
    mesh_builder.config.push(MeshConfig {
        count: 50,
        texture_position: bevy::sprite::Rect {
            min: vec2(674., 857.),
            max: vec2(685., 869.),
        },
        area: vec3(1000., 1000., 10.),
        distribution: DistributionFn::Random,
    });
    mesh_builder.config.push(MeshConfig {
        count: 50,
        texture_position: bevy::sprite::Rect {
            min: vec2(526., 854.),
            max: vec2(543., 871.),
        },
        area: vec3(1000., 1000., 10.),
        distribution: DistributionFn::Random,
    });
    mesh_builder.config.push(MeshConfig {
        count: 10,
        texture_position: bevy::sprite::Rect {
            min: vec2(613., 880.),
            max: vec2(656., 917.),
        },
        area: vec3(1000., 1000., 10.),
        distribution: DistributionFn::Random,
    });
    let mesh = mesh_builder.gen_mesh();

    // let mut rng = StdRng::from_entropy();
    let mesh_handle = meshes.add(mesh);
    commands.spawn(PbrComponents {
        mesh: mesh_handle,
        material: materials.add(StandardMaterial {
            // albedo: Color::rgb(
            //     rng.gen_range(0.0, 1.0),
            //     rng.gen_range(0.0, 1.0),
            //     rng.gen_range(0.0, 1.0),
            // ),
            albedo_texture: Some(mat_handle),
            shaded: false,
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
