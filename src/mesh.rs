use bevy::{prelude::Mesh, render::mesh::VertexAttribute, render::pipeline::PrimitiveTopology};

pub struct MeshMaker {
    pub vert_pos: Vec<[f32; 3]>,
    pub vert_norm: Vec<[f32; 3]>,
    pub vert_uvs: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
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
        mesh.indices = Some(self.indices.clone());
        mesh.attributes
            .push(VertexAttribute::position(self.vert_pos.clone()));
        mesh.attributes
            .push(VertexAttribute::normal(self.vert_norm.clone()));
        mesh.attributes
            .push(VertexAttribute::uv(self.vert_uvs.clone()));
        mesh
    }
}
