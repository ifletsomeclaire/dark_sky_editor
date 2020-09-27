use bevy::{
    math::vec2, prelude::*, render::mesh::Indices, render::mesh::VertexAttribute,
    render::pipeline::PrimitiveTopology,
};
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};
use noise::Seedable;
use rand::Rng;

pub fn generate_mega_mesh(
    width: i32,
    height: i32,
) -> Vec<MeshMaker> {
    let mut rng = rand::thread_rng();
    let mut stars = Vec::new();

    let noise = noise::Perlin::new().set_seed(1);
    let map = PlaneMapBuilder::new(&noise)
        .set_size(10000, 10000)
        .set_x_bounds(-500.0, 500.0)
        .set_y_bounds(-500.0, 500.0)
        .build();

    for h in 0..(height * 2) {
        for w in 0..(width * 2) {
            if map.get_value(w as usize, h as usize) > 0.0 {
                stars.push(vec2(((w - width) * 50) as f32, ((h - height) * 50) as f32));
            }
        }
    }
    println!("stars count: {:?}", stars.len());

    let mut meshmakers = Vec::new();
    let mut m_maker = MeshMaker::new();
    let mut count = 0;

    for star in stars {
        // let z_value = rng.gen_range(1000.0, 1500.0);
        let z_value = 1500.;

        let quad = Mesh::from(shape::Quad {
            size: vec2(5000.0, 5000.0),
            flip: false,
        });

        for position in quad.get_vertex_normals().unwrap() {
            let pos = [star.x() + position[0], star.y() + position[1], z_value];
            m_maker.vert_pos.push(pos);
        }
        // normals
        for norm in quad.get_vertex_normals().unwrap() {
            m_maker.vert_norm.push(norm);
        }
        // uvs
        for uv in quad.get_vertex_uvs().unwrap() {
            // m_maker.vert_uvs.push([
            //     match uv[0] {
            //         x if x < 0.0001 => rect.min[0] / atlas_size[0],
            //         _ => rect.max[0] / atlas_size[0],
            //     },
            //     match uv[1] {
            //         y if y < 0.0001 => rect.min[1] / atlas_size[1],
            //         _ => rect.max[1] / atlas_size[1],
            //     },
            // ]);
            m_maker.vert_uvs.push(uv);
        }

        match quad.indices.unwrap() {
            bevy::render::mesh::Indices::U16(_) => {}
            bevy::render::mesh::Indices::U32(i) => {
                for ind in i {
                    m_maker.indices.push(ind + count as u32);
                }
            }
        }
        count += 4;
        if count >= 64000 {
            meshmakers.push(m_maker);
            m_maker = MeshMaker::new();
            count = 0;
        }
    }
    meshmakers.push(m_maker);

    println!("vertex count: {:?}", count);
    // println!("meshes: {:?}", meshmakers);

    meshmakers
}

#[derive(Debug)]
pub struct MeshMaker {
    vert_pos: Vec<[f32; 3]>,
    vert_norm: Vec<[f32; 3]>,
    vert_uvs: Vec<[f32; 2]>,
    indices: Vec<u32>,
}
impl MeshMaker {
    pub fn new() -> Self {
        MeshMaker {
            vert_pos: Vec::new(),
            vert_norm: Vec::new(),
            vert_uvs: Vec::new(),
            indices: Vec::new(),
        }
    }
    pub fn generate_mesh(&self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.indices = Some(Indices::U32(self.indices.clone()));
        mesh.attributes
            .push(VertexAttribute::position(self.vert_pos.clone()));
        mesh.attributes
            .push(VertexAttribute::normal(self.vert_norm.clone()));
        mesh.attributes
            .push(VertexAttribute::uv(self.vert_uvs.clone()));
        mesh
    }
}

trait EditableMesh {
    fn get_vertex_positions(&self) -> Option<Vec<[f32; 3]>>;
    fn get_vertex_normals(&self) -> Option<Vec<[f32; 3]>>;
    fn get_vertex_uvs(&self) -> Option<Vec<[f32; 2]>>;
    fn get_mut_vertex_positions(&mut self) -> Option<&mut Vec<[f32; 3]>>;
    fn get_mut_vertex_normals(&mut self) -> Option<&mut Vec<[f32; 3]>>;
    fn get_mut_vertex_uvs(&mut self) -> Option<&mut Vec<[f32; 2]>>;
    fn add_mesh(&mut self, _other: &Mesh) {}
}

impl EditableMesh for Mesh {
    fn get_vertex_positions(&self) -> Option<Vec<[f32; 3]>> {
        match self.attributes[0].values {
            bevy::render::mesh::VertexAttributeValues::Float3(ref vertices) => {
                Some(vertices.clone())
            }
            _ => None,
        }
    }
    fn get_vertex_normals(&self) -> Option<Vec<[f32; 3]>> {
        match self.attributes[1].values {
            bevy::render::mesh::VertexAttributeValues::Float3(ref vertices) => {
                Some(vertices.clone())
            }
            _ => None,
        }
    }
    fn get_vertex_uvs(&self) -> Option<Vec<[f32; 2]>> {
        match self.attributes[2].values {
            bevy::render::mesh::VertexAttributeValues::Float2(ref vertices) => {
                Some(vertices.clone())
            }
            _ => None,
        }
    }
    fn get_mut_vertex_positions(&mut self) -> Option<&mut Vec<[f32; 3]>> {
        match self.attributes[0].values {
            bevy::render::mesh::VertexAttributeValues::Float3(ref mut vertices) => Some(vertices),
            _ => None,
        }
    }
    fn get_mut_vertex_normals(&mut self) -> Option<&mut Vec<[f32; 3]>> {
        match self.attributes[1].values {
            bevy::render::mesh::VertexAttributeValues::Float3(ref mut vertices) => Some(vertices),
            _ => None,
        }
    }
    fn get_mut_vertex_uvs(&mut self) -> Option<&mut Vec<[f32; 2]>> {
        match self.attributes[2].values {
            bevy::render::mesh::VertexAttributeValues::Float2(ref mut vertices) => Some(vertices),
            _ => None,
        }
    }
}
