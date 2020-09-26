use bevy::{prelude::Mesh, render::mesh::VertexAttribute, render::pipeline::PrimitiveTopology};

pub struct Skybox {
    pub size: f32,
}

impl Default for Skybox {
    fn default() -> Self {
        Skybox { size: 1.0 }
    }
}

// impl Skybox {
impl From<Skybox> for Mesh {
    fn from(cube: Skybox) -> Mesh {
        let size = cube.size;
        let vertices = &[
            // top (0., 0., size)
            ([-size, -size, size], [0., 0., size], [0., 0.]),
            ([size, -size, size], [0., 0., size], [size, 0.]),
            ([size, size, size], [0., 0., size], [size, size]),
            ([-size, size, size], [0., 0., size], [0., size]),
            // bottom (0., 0., -size)
            ([-size, size, -size], [0., 0., -size], [size, 0.]),
            ([size, size, -size], [0., 0., -size], [0., 0.]),
            ([size, -size, -size], [0., 0., -size], [0., size]),
            ([-size, -size, -size], [0., 0., -size], [size, size]),
            // right (size, 0., 0.)
            ([size, -size, -size], [size, 0., 0.], [0., 0.]),
            ([size, size, -size], [size, 0., 0.], [size, 0.]),
            ([size, size, size], [size, 0., 0.], [size, size]),
            ([size, -size, size], [size, 0., 0.], [0., size]),
            // left (-size, 0., 0.)
            ([-size, -size, size], [-size, 0., 0.], [size, 0.]),
            ([-size, size, size], [-size, 0., 0.], [0., 0.]),
            ([-size, size, -size], [-size, 0., 0.], [0., size]),
            ([-size, -size, -size], [-size, 0., 0.], [size, size]),
            // front (0., size, 0.)
            ([size, size, -size], [0., size, 0.], [size, 0.]),
            ([-size, size, -size], [0., size, 0.], [0., 0.]),
            ([-size, size, size], [0., size, 0.], [0., size]),
            ([size, size, size], [0., size, 0.], [size, size]),
            // back (0., -size, 0.)
            ([size, -size, size], [0., -size, 0.], [0., 0.]),
            ([-size, -size, size], [0., -size, 0.], [size, 0.]),
            ([-size, -size, -size], [0., -size, 0.], [size, size]),
            ([size, -size, -size], [0., -size, 0.], [0., size]),
        ];

        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        for (position, normal, uv) in vertices.iter() {
            positions.push(*position);
            normals.push(*normal);
            uvs.push(*uv);
        }

        let indices = vec![
            3, 2, 1, 1, 0, 3, // top
            7, 6, 5, 5, 4, 7, // bottom
            11, 10, 9, 9, 8, 11, // right
            15, 14, 13, 13, 12, 15, // left
            19, 18, 17, 17, 16, 19, // front
            23, 22, 21, 21, 20, 23, // back
        ];

        Mesh {
            primitive_topology: PrimitiveTopology::TriangleList,
            attributes: vec![
                VertexAttribute::position(positions),
                VertexAttribute::normal(normals),
                VertexAttribute::uv(uvs),
            ],
            indices: Some(indices),
        }
    }
}
