use bevy::prelude::*;
pub struct MeshieShipTest;

impl Plugin for MeshieShipTest {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(start.system());
    }
}



fn start(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut cube_mesh = Mesh::from(shape::Cube { size: 100.0 });
    let other = Mesh::from(shape::Cube { size: 50.0 });
    meshie::translate_mesh(&mut cube_mesh, Vec3::new(200., 0.0, 0.0));
    meshie::add_mesh(&mut cube_mesh, &other);
    let cube_handle = meshes.add(cube_mesh);

    commands.spawn(PbrComponents {
        mesh: cube_handle,

        material: materials.add(StandardMaterial {
            // albedo: Color::rgb(
            //     rng.gen_range(0.0, 1.0),
            //     rng.gen_range(0.0, 1.0),
            //     rng.gen_range(0.0, 1.0),
            // ),
            ..Default::default()
        }),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1000.0)),
        ..Default::default()
    });
}
