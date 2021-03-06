use bevy::{math::vec2, prelude::*};

use crate::equations_of_motion::{Destination, EquationsOfMotion, Momentum, QuatMath};
pub struct MotionTest;

impl Plugin for MotionTest {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(start.system())
            .add_system(movement.system());
    }
}
fn start(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = Mesh::from(shape::Cube { size: 60.0 });
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
            max_rotation: 0.1,
            thrust: 0.01,
            ..Default::default()
        })
        .with(Destination {
            d: Vec3::new(2000., 10000., 0.0),
        });
}
fn movement(mut query: Query<(&mut Momentum, &Destination, &mut Transform)>) {
    for (mut momentum, destination, mut transform) in &mut query.iter() {
        let mut pos = transform.translation();
        let null = Vec3::new(0.0, 0.0, 0.0);
        let mask = pos.cmpeq(null);
        if mask.all() {
            pos.set_x(1.0);
        }

        let facing = (transform.rotation().mul_vec3(Vec3::unit_y())).normalize();
        let vector_to_dest = (destination.d - pos).normalize();

        let (_axis, angle) = momentum.turn_to(facing, vector_to_dest);
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

