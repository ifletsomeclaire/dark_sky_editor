use bevy::math::*;
use bevy::prelude::*;
use meshie::*;
use rand::Rng;

use crate::meshie_ship_test::{distance, rays::Ray3d};

pub struct CollisionRay;

impl Plugin for CollisionRay {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(start.system())
            .add_system(collision_detection_rays_for_me.system());
    }
}

pub struct CubeShip {
    position: Vec3,
    radius: f32,
}
pub struct MeshieMarker {
    cubeships: Vec<CubeShip>,
    radius: f32,
}

fn start(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();
    let mut meshie = Mesh::from(shape::Cube { size: 100.0 });
    let mut cubeships = vec![CubeShip {
        position: vec3(0.0, 0.0, 0.0),
        radius: 50.0,
    }];

    for _ in 0..10 {
        let mut mesh = Mesh::from(shape::Cube { size: 100.0 });
        let position = vec3(
            rng.gen_range(-205.0, 205.0),
            rng.gen_range(-205.0, 205.0),
            rng.gen_range(-205.0, 205.0),
        );
        let verts = add_mesh(&mut meshie, &mesh);
        translate_mesh(&mut meshie, verts, position);

        cubeships.push(CubeShip {
            position,
            radius: 50.,
        });
    }
    let mesh_handle = meshes.add(meshie);
    let mat_handle = materials.add(StandardMaterial::default());
    commands
        .spawn(PbrComponents {
            mesh: mesh_handle,
            material: mat_handle,
            ..Default::default()
        })
        .with(MeshieMarker {
            cubeships,
            radius: 200.0,
        });
}

fn collision_detection_rays_for_me(mut query: Query<(&MeshieMarker, &Transform)>) {
    let ray = Ray3d::new(vec3(0.0, -1000.0, 0.0), vec3(0.0, 1.0, 0.0));
    for (meshie, transform) in &mut query.iter() {
        let dist = distance(ray.origin(), &transform.translation());
        let ray_pos = (ray.direction().normalize() * dist) + *ray.origin();
        let ray_dist = distance(&ray_pos, &transform.translation());
        // print!("ray_pos {} ray_dist {} radius {}", ray_pos, ray_dist, meshie.radius);
        if ray_dist < meshie.radius {
            for ship in &meshie.cubeships {
                let ship_pos = ship.position + transform.translation();
                let ship_dist = distance(ray.origin(), &ship_pos);
                let ray_ship_pos = (ray.direction().normalize() * ship_dist) + *ray.origin();
                let ray_ship_dist = distance(&ray_ship_pos, &ship_pos);
                if ray_ship_dist < ship.radius {
                    print!(" HIT {}", ship.position);
                }
            }
        }
    }
    println!("Result");
}
