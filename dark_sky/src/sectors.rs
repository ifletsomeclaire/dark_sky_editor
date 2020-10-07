use bevy::math::*;
use bevy::prelude::*;
use rand::Rng;

use crate::equations_of_motion::{Destination, EquationsOfMotion, Momentum};
pub struct Sectors;
impl Plugin for Sectors {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(sector_init.system())
            .add_startup_system(sector_movement_test_init.system())
            .add_system(movement.system())
            .add_system(sector_tick.system());
    }
}
#[derive(Debug, Default)]
struct Sector {
    iter_count: u32,
}
struct SectorMesh;
fn sector_init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();
    let mut vec_to_spawn = Vec::new();

    let mut mesh = Mesh::from(shape::Icosphere {
        radius: 10.0,
        subdivisions: 2,
    });
    for _ in 0..10000 {
        let mut trans = Transform::default();
        let x = rng.gen_range(-15000., 15000.);
        let y = rng.gen_range(-15000., 15000.);
        let z = rng.gen_range(3000., 3300.);
        trans.translate(Vec3::new(x, y, z));
        vec_to_spawn.push((trans, Sector::default()));
        let mut other = Mesh::from(shape::Icosphere {
            radius: 100.0,
            subdivisions: 2,
        });

        meshie::translate_mesh(&mut other, Vec3::new(x, y, z));
        // return the indicies added
        meshie::add_mesh(&mut mesh, &other);
    }

    let meshie_handle = meshes.add(mesh);
    commands.spawn_batch(vec_to_spawn);
    commands
        .spawn(PbrComponents {
            mesh: meshie_handle,
            draw: Draw {
                is_transparent: true,
                ..Default::default()
            },
            material: materials.add(StandardMaterial {
                albedo: Color::rgba(
                    rng.gen_range(0.0, 1.0),
                    rng.gen_range(0.0, 1.0),
                    rng.gen_range(0.0, 1.0),
                    rng.gen_range(0.3, 0.7),
                ),
                ..Default::default()
            }),
            ..Default::default()
        })
        .with(SectorMesh);
}
fn sector_tick(mut query: Query<(&Transform, &mut Sector)>) {
    for (_trans, mut sector) in &mut query.iter() {
        sector.iter_count += 1;
    }
}

fn sector_movement_test_init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = Mesh::from(shape::Cube { size: 160.0 });
    let cube_handle = meshes.add(mesh);
    commands
        .spawn(PbrComponents {
            mesh: cube_handle,
            material: materials.add(StandardMaterial {
                albedo: Color::rgb(1.0, 0.0, 1.0),
                shaded: false,
                ..Default::default()
            }),
            transform: Transform::from_rotation(Quat::from_rotation_z(0.0)),
            ..Default::default()
        })
        .with(Momentum {
            max_rotation: 2.14,
            thrust: 0.1,
            ..Default::default()
        })
        .with(Destination {
            d: Vec3::new(2000., 3000., 0.0),
        });
}

fn movement(
    mut query: Query<(&mut Momentum, &mut Destination, &mut Transform)>,
    mut sector_query: Query<(&Transform, &mut Sector)>,
) {
    for (mut momentum, mut destination, mut transform) in &mut query.iter() {
        let dist = momentum.distance(&transform.translation(), &destination.d);
        // println!("{}", dist);

        if dist < 1000.0 {
            destination.d = sector_query
                .iter()
                .iter()
                .find(|(t, _)| {
                    if momentum.distance(&transform.translation(), &t.translation()) > 5000. {
                        true
                    } else {
                        false
                    }
                })
                .and_then(|(v, _)| Some(v.translation()))
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

        let (axis, angle) = momentum.turn_to(facing, vector_to_dest);
        let s = (momentum.max_rotation() / angle).abs();
        let look_at = Quat::default_to_vec3(vector_to_dest);

        let rot = transform.rotation().normalize();

        if rot.dot(look_at) < 0.0 {
            transform.set_rotation(rot.slerp(-look_at, s.min(1.0)));
        } else {
            transform.set_rotation(rot.slerp(look_at, s.min(1.0)));
        }

        // let s = momentum.max_rotation() / facing.angle_between(vector_to_dest);
        // let dot = facing.dot(vector_to_dest);
        // // let final_angle = Vec3::unit_y().angle_between(vector_to_dest);
        // // let facing_angle = Vec3::unit_y().angle_between(facing);
        // // let turn_angle = final_angle - facing_angle;
        // // let final_angle = momentum.turn_to(facing, vector_to_dest);
        // // if final_angle > 0.00001 {
        // //     transform.set_rotation(Quat::from_rotation_z(facing_angle+turn_angle));
        // // }
        //     let (axis, angle) = momentum.turn_to(facing, vector_to_dest);
        //     transform.rotate(Quat::from_axis_angle(axis, angle).normalize());

        let thrust = momentum.thrust();
        // momentum.inertia += vec2(0.0, 0.1);
        momentum.inertia += vec2(
            facing.x().sin() * thrust,
            (facing.y().cos() * thrust).copysign(facing.y()),
        );

        transform.translate(momentum.inertia().extend(0.0));

        // println!(
        //     "a {:<13?} d {:<13?} t {:<13?} f {:<13?} ax {:<13?}",
        //     angle,
        //     //  0,0
        //     // angle_to_turn,
        //     momentum.inertia,
        //     transform.translation(),
        //     // transform.rotation().to_axis_angle(),
        //     facing,
        //     axis
        // );
    }
}

trait QuatMath {
    fn from_to_vec3(from: Vec3, to: Vec3) -> Quat;
    fn default_to_vec3(to: Vec3) -> Quat;
}

impl QuatMath for Quat {
    fn from_to_vec3(from: Vec3, to: Vec3) -> Quat {
        let from_vec = from.normalize();
        let to_vec = to.normalize();
        let dot = from_vec.dot(to_vec);
        if dot >= 1.0 {
            return Quat::identity();
        }
        if dot < 1e-6_f32 - 1.0 {
            let mut axis = Vec3::unit_x().cross(from);
            if axis.length() == 0.0 {
                axis = Vec3::unit_y().cross(from);
            }
            return Quat::from_axis_angle(axis.normalize(), std::f32::consts::PI);
        }
        let angle = dot.acos();
        Quat::from_axis_angle(from_vec.cross(to_vec).normalize(), angle).normalize()
    }
    fn default_to_vec3(forward: Vec3) -> Quat {
        Quat::from_to_vec3(Vec3::unit_y(), forward)
    }
}
