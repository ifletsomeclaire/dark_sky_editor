use bevy::{math::*, prelude::*};
use meshie::Meshie;

use crate::equations_of_motion::*;

pub struct MovementDebug;

#[derive(Default, Debug)]
struct MovementValues {
    applied_thrust: Vec3,
    destination: Vec3,
    ship_pos: Vec3,
    applied_rotation: Quat,
}

impl Plugin for MovementDebug {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<MovementValues>()
            .add_startup_system(start.system())
            .add_system(movement.system())
            .add_system(debug.system());
    }
}

fn start(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_meshie = generate_debug_meshie(&mut meshes);
    let meshie = meshes.get_mut(&debug_meshie.mesh_handle).expect("can't find meshie???");
    let destination = Destination {
        d: Vec3::new(2000., 1000., 0.0),
    };
    meshie.translate_mesh(debug_meshie.destination, destination.d);
    commands
        .spawn(PbrComponents {
            mesh: debug_meshie.mesh_handle,
            material: materials.add(StandardMaterial {
                albedo: Color::rgb(1.0, 0.0, 1.0),
                shaded: false,
                ..Default::default()
            }),
            transform: Transform::from_rotation(Quat::from_rotation_z(0.0)),
            ..Default::default()
        })
        .with(Momentum {
            max_rotation: 0.1,
            thrust: 0.01,
            ..Default::default()
        })
        .with(destination)
        .with(debug_meshie);
}
fn movement(
    mut values: ResMut<MovementValues>,
    mut query: Query<(
        &mut Momentum,
        &mut Destination,
        &mut Transform,
        // &mut MoveLogic,
    )>,
    mut sector_query: Query<&Transform>,
) {
    for (mut momentum, mut destination, mut transform) in &mut query.iter() {
        let dist = momentum.distance(&transform.translation(), &destination.d);
        print!("{}", dist);

        if dist < 10.0 {
            destination.d = sector_query
                .iter()
                .iter()
                .find(|t| {
                    if momentum.distance(&transform.translation(), &t.translation()) > 5000. {
                        true
                    } else {
                        false
                    }
                })
                .and_then(|v| Some(v.translation()))
                .unwrap();
        }
        let mut pos = transform.translation();
        let null = Vec3::new(0.0, 0.0, 0.0);
        let mask = pos.cmpeq(null);
        if mask.all() {
            pos.set_x(1.0);
        }

        let facing = (transform.rotation().mul_vec3(Vec3::unit_y())).normalize();
        let vector_to_dest = (destination.d - pos).normalize();
        let mut bad_vec = vector_to_dest - momentum.inertia.extend(0.00001).normalize();
        if bad_vec.length() < momentum.thrust * 2.0 {
            bad_vec = vector_to_dest
        }

        let (_axis, angle) = momentum.turn_to(facing, bad_vec);
        let s = (momentum.max_rotation() / angle).abs();

        let look_at = Quat::default_to_vec3(bad_vec);

        let rot = transform.rotation().normalize();

        if rot.dot(look_at) < 0.0 {
            transform.set_rotation(rot.slerp(-look_at, s.min(1.0)));
        } else {
            transform.set_rotation(rot.slerp(look_at, s.min(1.0)));
        }

        let thrust = momentum.thrust();
        let applied_thrust = vec2(
            facing.x().sin() * thrust,
            (facing.y().cos() * thrust).copysign(facing.y()),
        );
        momentum.inertia += applied_thrust;

        transform.translate(momentum.inertia().extend(0.0));
        values.destination = destination.d;
        // values.ship_pos = transform.translation();
        values.applied_thrust = applied_thrust.extend(0.0);

    }
}

fn debug(values: Res<MovementValues>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&mut DebugMeshie, &mut Transform)>
) {
    for (mut debug, mut transform) in &mut query.iter() {
        // transform.set_translation(values.ship_pos);
        let meshie = meshes.get_mut(&debug.mesh_handle).expect("can't find meshie???");
        if (debug.last_path - transform.translation()).length() > 30.0 {
            let _ = meshie.add_mesh(&Mesh::from(shape::Quad{ size: vec2(10.0, 10.0), flip: false}));
            debug.last_path = transform.translation();
        }
        meshie.translate_mesh(debug.destination, -values.applied_thrust);

    }
}

pub struct DebugMeshie {
    mesh_handle: Handle<Mesh>,
    momentum: ds_range::Range,
    facing: ds_range::Range,
    destination: ds_range::Range,
    last_path: Vec3,
}

pub fn generate_debug_meshie(meshes: &mut ResMut<Assets<Mesh>>) -> DebugMeshie {
    let mut meshie = Mesh::from(shape::Cube { size: 100.0 });
    let momentum = meshie.add_mesh(&Mesh::from(shape::Quad {
        size: vec2(10.0, 10.0),
        flip: false,
    }));
    let facing = meshie.add_mesh(&Mesh::from(shape::Quad {
        size: vec2(10.0, 10.0),
        flip: false,
    }));
    let destination = meshie.add_mesh(&Mesh::from(shape::Icosphere {
        radius: 50.0,
        subdivisions: 3,
    }));

    DebugMeshie {
        mesh_handle: meshes.add(meshie),
        momentum,
        facing,
        destination,
        last_path: Vec3::default(),
    }
}
