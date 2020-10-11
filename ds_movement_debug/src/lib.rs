use bevy::{math::*, prelude::*};
use meshie::Meshie;

pub mod systems;

pub struct MovementDebug;

#[derive(Default, Debug)]
pub struct EffectsResource {
    pub mesh_handle: Handle<Mesh>,
    pub availability: Vec<Availability>,
    pub vertices: Vec<ds_range::Range>,
    pub chunk_size: u32,
    pub max_chunks: u32,
}
#[derive(Debug, PartialEq)]
pub enum Availability {
    Open,
    Used,
}
impl Default for Availability {
    fn default() -> Self {
        Self::Open
    }
}

pub struct DebugMeshie {
    pub entity: Entity,
    pub mesh_handle: Handle<Mesh>,
    pub momentum: ds_range::Range,
    pub momentum_pos: Vec<[f32; 3]>,
    // facing: ds_range::Range,
    pub last_path: Vec3,
    pub prior_inertia: Vec3,
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

pub struct EntityDestination {
    pub target: Entity,
}
pub struct EffectsMeshie;
#[derive(Default, Debug)]
pub struct Effect {
    pub indices: ds_range::Range,
}

pub fn generate_effects_meshie(
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
