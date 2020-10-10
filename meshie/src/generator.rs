use bevy::{
    math::{vec2, vec3, Vec2, Vec3},
    prelude::*,
    sprite::Rect,
};
use rand::Rng;

use crate::Meshie;

#[derive(Debug, Copy, Clone)]
pub enum DistributionFn {
    Random,
}

pub struct MeshBuilder {
    pub texture_size: Vec2,
    pub config: Vec<MeshConfig>,
}

#[derive(Debug, Copy, Clone)]
pub struct MeshConfig {
    pub count: u32,
    pub texture_position: Rect,
    pub area: Vec3,
    pub distribution: DistributionFn,
}

impl MeshBuilder {
    pub fn gen_mesh(&self) -> Mesh {
        let mut rng = rand::thread_rng();
        let mut z_value = 0.0;
        let mut mesh = Mesh::from(shape::Quad {
            size: vec2(1.0, 1.0),
            flip: false,
        });

        for config in &self.config {
            match config.distribution {
                DistributionFn::Random => {
                    for _ in 0..config.count {
                        let mut new_mesh = Mesh::from(shape::Quad {
                            size: vec2(
                                config.texture_position.max.x() - config.texture_position.min.x(),
                                config.texture_position.max.y() - config.texture_position.min.y(),
                            ),
                            flip: false,
                        });

                        // update uvs
                        match new_mesh.attributes[2].values {
                            bevy::render::mesh::VertexAttributeValues::Float2(ref mut values) => {
                                for uv in values {
                                    uv[0] = match uv[0] {
                                        x if x < 0.0001 => {
                                            config.texture_position.min[0] / self.texture_size[0]
                                        }
                                        _ => config.texture_position.max[0] / self.texture_size[0],
                                    };
                                    uv[1] = match uv[1] {
                                        y if y < 0.0001 => {
                                            config.texture_position.min[1] / self.texture_size[1]
                                        }
                                        _ => config.texture_position.max[1] / self.texture_size[1],
                                    };
                                }
                            }
                            _ => {}
                        }
                        let verts = mesh.add_mesh(&new_mesh);
                        mesh.translate_mesh(
                            verts,
                            vec3(
                                rng.gen_range(-config.area.x(), config.area.x()),
                                rng.gen_range(-config.area.y(), config.area.y()),
                                rng.gen_range(0., config.area.z()) + z_value,
                            ),
                        );
                    }
                }
            }
            z_value += config.area.z();
        }
        mesh
    }
}
