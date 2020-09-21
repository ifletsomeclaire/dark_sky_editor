use bevy::{prelude::Mesh, render::mesh::Vertex, render::mesh::VertexAttribute, render::pipeline::PrimitiveTopology};

pub struct MeshMaker {
    pub vert_pos: Vec<[f32; 3]>,
    pub vert_norm: Vec<[f32; 3]>,
    pub vert_uvs: Vec<[f32; 2]>,
    pub vert_colors: Vec<[f32; 4]>,
    pub vert_textures: Vec<f32>,
    pub indices: Vec<u32>,
}
impl MeshMaker {
    pub fn new() -> Self {
        MeshMaker {
            vert_pos: Vec::new(),
            vert_norm: Vec::new(),
            vert_uvs: Vec::new(),
            vert_colors: Vec::new(),
            vert_textures: Vec::new(),
            indices: Vec::new(),
        }
    }
    pub fn generate_mesh(&self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.indices = Some(self.indices.clone());
        mesh.attributes
            .push(VertexAttribute::position(self.vert_pos.clone()));
        mesh.attributes
            .push(VertexAttribute::normal(self.vert_norm.clone()));
        mesh.attributes
            .push(VertexAttribute::uv(self.vert_uvs.clone()));
        mesh.attributes
            .push(VertexAttribute::color(self.vert_colors.clone()));
        mesh.attributes
            .push(VertexAttribute::texture(self.vert_textures.clone()));
        mesh
    }
}


pub trait EditableMesh {
    fn get_vertex_positions(&self) -> Option<Vec<[f32; 3]>>;
    fn get_vertex_normals(&self) -> Option<Vec<[f32; 3]>>;
    fn get_vertex_uvs(&self) -> Option<Vec<[f32; 2]>>;
    fn get_vertex_colors(&self) -> Option<Vec<[f32; 4]>>;
    fn get_vertex_textures(&self) -> Option<Vec<f32>>;
    // fn get_vertices(&self) -> Option<Vec<Vertex>>;
    fn get_mut_vertex_positions(&mut self) -> Option<&mut Vec<[f32; 3]>>;
    fn get_mut_vertex_normals(&mut self) -> Option<&mut Vec<[f32; 3]>>;
    fn get_mut_vertex_uvs(&mut self) -> Option<&mut Vec<[f32; 2]>>;
    fn add_mesh(&mut self, other: &Mesh) {}
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
    fn get_vertex_colors(&self) -> Option<Vec<[f32; 4]>> {
        match self.attributes[3].values {
            bevy::render::mesh::VertexAttributeValues::Float4(ref vertices) => {
                Some(vertices.clone())
            }
            _ => None,
        }
    }
    fn get_vertex_textures(&self) -> Option<Vec<f32>> {
        match self.attributes[4].values {
            bevy::render::mesh::VertexAttributeValues::Float(ref vertices) => {
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
