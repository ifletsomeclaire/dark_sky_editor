use bevy::{
    prelude::Mesh, render::mesh::Indices, render::mesh::VertexAttribute,
    render::pipeline::PrimitiveTopology,
};
use hexasphere::Hexasphere;

use crate::plugin::reverse_triangles;

/// A sphere made from a subdivided Icosahedron.
pub struct SkySphere {
    /// The radius of the sphere.
    pub radius: f32,
    /// The number of subdivisions applied.
    pub subdivisions: usize,
}

impl Default for SkySphere {
    fn default() -> Self {
        Self {
            radius: 1.0,
            subdivisions: 5,
        }
    }
}

impl From<SkySphere> for Mesh {
    fn from(sphere: SkySphere) -> Self {
        if sphere.subdivisions >= 80 {
            let temp_sphere = Hexasphere::new(sphere.subdivisions, |_| ());

            panic!(
                    "Cannot create an icosphere of {} subdivisions due to there being too many vertices being generated: {} (Limited to 65535 vertices or 79 subdivisions)",
                    sphere.subdivisions,
                    temp_sphere.raw_points().len()
                );
        }
        let hexasphere = Hexasphere::new(sphere.subdivisions, |point| {
            let inclination = point.z().acos();
            let azumith = point.y().atan2(point.x());

            let norm_inclination = 1.0 - (inclination / std::f32::consts::PI);
            let mut norm_azumith = (azumith / std::f32::consts::PI) * 0.5;
            if norm_azumith < 0.0 {
                norm_azumith += 1.0
            }

            [norm_inclination, norm_azumith]
        });

        let raw_points = hexasphere.raw_points();

        let points = raw_points
            .iter()
            .map(|&p| (p * sphere.radius).into())
            .collect::<Vec<[f32; 3]>>();

        let normals = raw_points
            .iter()
            .copied()
            .map(Into::into)
            .collect::<Vec<[f32; 3]>>();

        let uvs = hexasphere.raw_data().to_owned();

        let mut indices = Vec::with_capacity(hexasphere.indices_per_main_triangle() * 20);

        for i in 0..20 {
            hexasphere.get_indices(i, &mut indices);
        }

        let indices = Indices::U32(indices);

        Mesh {
            primitive_topology: PrimitiveTopology::TriangleList,
            attributes: vec![
                VertexAttribute::position(points),
                VertexAttribute::normal(normals),
                VertexAttribute::uv(uvs),
            ],
            indices: reverse_triangles(Some(indices)),
        }
    }
}


