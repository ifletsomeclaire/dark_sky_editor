use bevy::{math::*, prelude::*, render::pipeline::PrimitiveTopology};
use meshie::Meshie;

use crate::equations_of_motion::*;

pub struct MovementDebug;

#[derive(Default, Debug)]
struct EffectsResource {
    mesh_handle: Handle<Mesh>,
    availability: Vec<Availability>,
    vertices: Vec<ds_range::Range>,
    chunk_size: u32,
    max_chunks: u32,
}
#[derive(Debug, PartialEq)]
enum Availability {
    Open,
    Used,
}
impl Default for Availability {
    fn default() -> Self {
        Self::Open
    }
}

impl Plugin for MovementDebug {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<EffectsResource>()
            .add_startup_system(start.system())
            .add_system(movement.system())
            .add_system(debug.system())
            .add_system(move_meshie.system())
            .add_system(update_meshie_momentum.system());
    }
}

fn start(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut effects: ResMut<EffectsResource>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // create the destination entity
    let destination = Destination {
        d: vec3(1000.0, 2000.0, 0.0),
    };
    let dest = commands
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 100.0,
                subdivisions: 5,
            })),
            material: materials.add(StandardMaterial {
                albedo: Color::rgb(0.0, 1.0, 0.5),
                shaded: false,
                ..Default::default()
            }),
            transform: Transform::from_translation(destination.d),
            ..Default::default()
        })
        .current_entity();
    // create the ship entity
    let ship = commands
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 100.0 })),
            material: materials.add(StandardMaterial {
                albedo: Color::rgb(1.0, 0.0, 1.0),
                shaded: false,
                ..Default::default()
            }),
            ..Default::default()
        })
        .with(Momentum {
            max_rotation: 0.05,
            thrust: 0.05,
            inertia: vec2(0.01, 0.01),
        })
        .with(EntityDestination {
            target: dest.expect("getting destination entity"),
        })
        .current_entity();
    // create the debug entity
    let debug_meshie = generate_debug_meshie(ship.expect("getting ship entity"), &mut meshes);
    let _debug = commands
        .spawn(PbrComponents {
            mesh: debug_meshie.mesh_handle,
            material: materials.add(StandardMaterial {
                albedo: Color::rgb(1.0, 1.0, 0.0),
                shaded: false,
                ..Default::default()
            }),
            ..Default::default()
        })
        .with(debug_meshie)
        .current_entity();
    effects.mesh_handle = meshes.add(generate_effects_meshie(4, 1000, &mut effects));
    let _ = commands
        .spawn(PbrComponents {
            mesh: effects.mesh_handle,
            material: materials.add(StandardMaterial {
                albedo: Color::rgb(0.5, 0.5, 0.5),
                shaded: false,
                ..Default::default()
            }),
            ..Default::default()
        })
        .current_entity();
}
fn movement(
    mut query: Query<(
        &mut Momentum,
        &EntityDestination,
        &mut Transform,
        // &mut MoveLogic,
    )>,
    sector_query: Query<&Transform>,
) {
    for (mut momentum, dest_entity, mut transform) in &mut query.iter() {
        let dest_transform: Transform = *(sector_query
            .get(dest_entity.target)
            .expect("unwrap destination"));
        let destination = dest_transform.translation();
        let dist = momentum.distance(&transform.translation(), &destination);
        println!("{}", dist);

        let mut pos = transform.translation();
        let null = Vec3::new(0.0, 0.0, 0.0);
        let mask = pos.cmpeq(null);
        if mask.all() {
            pos.set_x(1.0);
        }

        let facing = (transform.rotation().mul_vec3(Vec3::unit_y())).normalize();
        let vector_to_dest = (destination - pos).normalize();
        let mut bad_vec = vector_to_dest - momentum.inertia.extend(0.00001).normalize();
        if bad_vec.length_squared() < momentum.thrust * 10.0 {
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
        // let applied_thrust = vec2(0.01, 0.0);
        momentum.inertia += applied_thrust;

        transform.translate(momentum.inertia().extend(0.0));
    }
}

fn debug(
    // mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut effects: ResMut<EffectsResource>,
    mut debug_query: Query<(&mut DebugMeshie, &mut Transform)>,
    // effects_query: Query<&EffectsMeshie>,
) {
    for (mut debug, transform) in &mut debug_query.iter() {
        let effects_meshie = meshes
            .get_mut(&effects.mesh_handle)
            .expect("can't find meshie???");
        if (debug.last_path - transform.translation()).length() > 200.0 {
            debug.last_path = transform.translation();
            let length = effects.availability.len();
            for i in 0..length {
                if effects.availability[i] == Availability::Open {
                    effects_meshie
                        .translate_mesh(effects.vertices[i as usize], transform.translation());
                    effects.availability[i] = Availability::Used;
                    break;
                }
            }
        }
    }
}

fn update_meshie_momentum(
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<&mut DebugMeshie>,
    mom_query: Query<&Momentum>,
    tran_query: Query<&Transform>,
) {
    println!("I am running....");
    for mut debug in &mut query.iter() {
        let debug_meshie = meshes
            .get_mut(&debug.mesh_handle)
            .expect("I expected to get a debug mesh");
        let momentum: Ref<Momentum> = mom_query.get(debug.entity).expect("getting transform");
        let transform: Ref<Transform> = tran_query.get(debug.entity).expect("getting transform");
        let rot = transform.rotation();
        println!("{:?}", momentum);
        debug_meshie.set_positions(debug.momentum, debug.momentum_pos.clone());
        debug_meshie.rotate_from_meshie_center(
            debug.momentum,
            Quat::from_to_vec3(
                transform.rotation().mul_vec3(Vec3::unit_y()),
                momentum.inertia().extend(0.0),
            ),
        );
        debug.prior_inertia = momentum.inertia().extend(0.0);
    }
}
fn move_meshie(mut query: Query<(&mut Transform, &DebugMeshie)>, ship_query: Query<&Transform>) {
    for (mut transform, debug) in &mut query.iter() {
        let ship_transform: Ref<Transform> =
            ship_query.get(debug.entity).expect("getting transform");
        *transform = *ship_transform;
    }
}

pub struct DebugMeshie {
    entity: Entity,
    mesh_handle: Handle<Mesh>,
    momentum: ds_range::Range,
    momentum_pos: Vec<[f32; 3]>,
    // facing: ds_range::Range,
    last_path: Vec3,
    prior_inertia: Vec3,
}

pub fn generate_debug_meshie(entity: Entity, meshes: &mut ResMut<Assets<Mesh>>) -> DebugMeshie {
    let mut meshie = Mesh::from(shape::Quad {
        size: vec2(10.0, 200.0),
        flip: false,
    });
    meshie.translate_mesh(ds_range::Range { start: 0, end: 3 }, vec3(0.0, 190.0, 0.0));
    let momentum = meshie.add_mesh(&Mesh::from(shape::Quad {
        size: vec2(10.0, 200.0),
        flip: false,
    }));
    meshie.translate_mesh(momentum, vec3(0.0, 290.0, 0.0));
    let positions = meshie.get_positions(momentum);

    DebugMeshie {
        entity,
        mesh_handle: meshes.add(meshie),
        momentum,
        momentum_pos: positions,
        // facing: ds_range::Range { start: 0, end: 3},
        last_path: Vec3::default(),
        prior_inertia: Vec3::default(),
    }
}

struct EntityDestination {
    target: Entity,
}
struct EffectsMeshie;
#[derive(Default, Debug)]
struct Effect {
    indices: ds_range::Range,
}

fn generate_effects_meshie(
    chunk_size: u32,
    max_chunks: u32,
    effects: &mut ResMut<EffectsResource>,
) -> Mesh {
    effects.chunk_size = chunk_size;
    effects.max_chunks = max_chunks;

    let mut effects_meshie = Mesh::from(shape::Quad {
        size: vec2(80.0, 80.0),
        flip: false,
    });
    effects.vertices.push(ds_range::Range { start: 0, end: 3 });
    effects.availability.push(Availability::Used);

    for _ in 0..(max_chunks - 1) {
        let mesh = Mesh::from(shape::Quad {
            size: vec2(80.0, 80.0),
            flip: false,
        });
        effects.vertices.push(effects_meshie.add_mesh(&mesh));
        effects.availability.push(Availability::Open);
    }
    effects_meshie
}
