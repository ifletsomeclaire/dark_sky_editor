use ds_range::Range;

use bevy::{math::Quat, math::Vec2, math::Vec3, prelude::Mesh, render::mesh::Indices};

pub mod generator;

pub trait Meshie {
    fn reverse_triangles(&mut self);
    fn add_mesh(&mut self, other: &Self) -> Range;
    fn translate_mesh(&mut self, vertices: ds_range::Range, translation: Vec3);
    fn rotate_mesh(&mut self, vertices: ds_range::Range, rotation: Quat);
    fn rotate_from_meshie_center(&mut self, vertices: ds_range::Range, rotation: Quat);
    fn get_center(&self, vertices: ds_range::Range) -> Vec3;
    fn extend_mesh(&mut self, vertices: ds_range::Range, direction: Vec3);
    fn set_uvs(&mut self, vertices: Range, uv: Vec<[f32;2]>);
    fn get_positions(&self, vertices: ds_range::Range) -> Vec<[f32; 3]>;
    fn set_positions(&mut self, vertices: ds_range::Range, positions: Vec<[f32; 3]>);
}

impl Meshie for Mesh {
    fn reverse_triangles(&mut self) {
        if let Some(i) = self.indices.as_mut() {
            match i {
                Indices::U16(_) => {}
                Indices::U32(ref mut ind) => {
                    for triangle in ind.rchunks_exact_mut(3) {
                        let t0 = triangle[0];
                        triangle[0] = triangle[2];
                        triangle[2] = t0;
                    }
                }
            }
        }
    }
    fn add_mesh(&mut self, other: &Mesh) -> Range {
        let mut result: Range = Range::default();
        if let Some(indices) = self.indices.as_mut() {
            match indices {
                Indices::U16(_) => {}
                Indices::U32(ref mut values) => {
                    match other.indices.as_ref().expect("other indices") {
                        Indices::U16(_) => {}
                        Indices::U32(ref addons) => {
                            add_indices(values, addons, self.attributes[0].values.len());
                        }
                    }
                }
            }
        } else {
            match other.indices.as_ref().expect("other indices") {
                Indices::U16(_) => {}
                Indices::U32(ref addons) => {
                    add_indices(&mut vec![], addons, 0);
                }
            }
        }
        match self.attributes[0].values {
            bevy::render::mesh::VertexAttributeValues::Float3(ref mut values) => {
                match other.attributes[0].values {
                    bevy::render::mesh::VertexAttributeValues::Float3(ref addons) => {
                        result.start = values.len();
                        result.end = values.len() + addons.len();
                        add_positions(values, addons);
                    }
                    _ => panic!(" no positions on mesh"),
                }
            }
            _ => {}
        }
        match self.attributes[1].values {
            bevy::render::mesh::VertexAttributeValues::Float3(ref mut values) => {
                match other.attributes[1].values {
                    bevy::render::mesh::VertexAttributeValues::Float3(ref addons) => {
                        add_normals(values, addons);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        match self.attributes[2].values {
            bevy::render::mesh::VertexAttributeValues::Float2(ref mut values) => {
                match other.attributes[2].values {
                    bevy::render::mesh::VertexAttributeValues::Float2(ref addons) => {
                        add_uvs(values, addons);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        result
    }

    fn translate_mesh(&mut self, vertices: ds_range::Range, translation: Vec3) {
        match self.attributes[0].values {
            bevy::render::mesh::VertexAttributeValues::Float3(ref mut values) => {
                for i in vertices.iter() {
                    values[i][0] = values[i][0] + translation.x();
                    values[i][1] = values[i][1] + translation.y();
                    values[i][2] = values[i][2] + translation.z();
                }
            }
            _ => {}
        }
    }
    fn rotate_mesh(&mut self, vertices: ds_range::Range, rotation: Quat) {
        let center = self.get_center(vertices);
        match self.attributes[0].values {
            bevy::render::mesh::VertexAttributeValues::Float3(ref mut values) => {
                for i in vertices.iter() {
                    let new_pos =
                        rotation.mul_vec3(Vec3::from_slice_unaligned(&values[i]) - center) + center;
                    values[i] = [new_pos.x(), new_pos.y(), new_pos.z()];
                }
            }
            _ => {}
        }
    }
    fn rotate_from_meshie_center(&mut self, vertices: ds_range::Range, rotation: Quat) {
        // let center = self.get_center(vertices);
        match self.attributes[0].values {
            bevy::render::mesh::VertexAttributeValues::Float3(ref mut values) => {
                for i in vertices.iter() {
                    let new_pos = rotation.mul_vec3(Vec3::from_slice_unaligned(&values[i]));
                    values[i] = [new_pos.x(), new_pos.y(), new_pos.z()];
                }
            }
            _ => {}
        }
    }
    fn get_center(&self, vertices: ds_range::Range) -> Vec3 {
        match self.attributes[0].values {
            bevy::render::mesh::VertexAttributeValues::Float3(ref values) => {
                let mut average = Vec3::default();
                for i in vertices.iter() {
                    average += Vec3::from_slice_unaligned(&values[i]);
                }
                average /= vertices.len() as f32;
                average
            }
            _ => panic!("Vertices are in the wrong order"),
        }
    }
    fn extend_mesh(&mut self, vertices: ds_range::Range, direction: Vec3) {
        let center = self.get_center(vertices);
        match self.attributes[0].values {
            bevy::render::mesh::VertexAttributeValues::Float3(ref mut values) => {
                for i in vertices.iter() {
                    let vert = Vec3::from_slice_unaligned(&values[i]);
                    if direction.dot(vert - center) > 0.0 {
                        let new_pos = vert + direction;
                        values[i] = [new_pos.x(), new_pos.y(), new_pos.z()];
                    }
                }
            }
            _ => {}
        }
    }
    fn set_uvs(&mut self, vertices: Range, uv: Vec<[f32;2]>) {
        match self.attributes[2].values {
            bevy::render::mesh::VertexAttributeValues::Float2(ref mut values) => {
                for i in vertices.iter() {
                    values[i] = uv[i - vertices.start];
                }
            }
            _ => panic!("no uvs on mesh??"),
        }
    }

    fn get_positions(&self, vertices: Range) -> Vec<[f32; 3]> {
        match self.attributes[0].values {
            bevy::render::mesh::VertexAttributeValues::Float3(ref values) => {
                let mut result = Vec::new();
                for i in vertices.iter() {
                    result.push(values[i]);
                }
                result
            }
            _ => panic!("no positions on mesh??"),
        }
    }

    fn set_positions(&mut self, vertices: Range, positions: Vec<[f32; 3]>) {
        match self.attributes[0].values {
            bevy::render::mesh::VertexAttributeValues::Float3(ref mut values) => {
                for i in vertices.iter() {
                    values[i] = positions[i - vertices.start];
                }
            }
            _ => panic!("no positions on mesh??"),
        }
    }
}

fn add_indices(mesh: &mut Vec<u32>, other: &Vec<u32>, count: usize) {
    mesh.extend(other.iter().map(|o| *o + count as u32))
}
fn add_positions(mesh: &mut Vec<[f32; 3]>, other: &Vec<[f32; 3]>) {
    mesh.extend(other)
}
fn add_normals(mesh: &mut Vec<[f32; 3]>, other: &Vec<[f32; 3]>) {
    mesh.extend(other)
}
fn add_uvs(mesh: &mut Vec<[f32; 2]>, other: &Vec<[f32; 2]>) {
    mesh.extend(other)
}

// pub fn remove(&mut Mesh, indices)

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::{
        math::vec2,
        prelude::{shape, Mesh},
    };

    #[test]
    fn it_works() {
        let mut mesh_one = Mesh::from(shape::Quad {
            size: vec2(200., 200.),
            flip: false,
        });
        let mesh_two = Mesh::from(shape::Quad {
            size: vec2(200., 200.),
            flip: false,
        });
        mesh_one.add_mesh(&mesh_two);
        println!("{:?}", mesh_one);
    }
}
