use std::ops::Range;

use bevy::{math::Quat, math::Vec3, prelude::Mesh, render::mesh::Indices};

pub mod generator;

pub fn reverse_triangles(mesh: &mut Mesh) {
    if let Some(i) = mesh.indices.as_mut() {
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

pub fn add_mesh(mesh: &mut Mesh, other: &Mesh) -> Range<usize> {
    let mut result: Range<usize> = Range::default();
    if let Some(indices) = mesh.indices.as_mut() {
        match indices {
            Indices::U16(_) => {}
            Indices::U32(ref mut values) => match other.indices.as_ref().expect("other indices") {
                Indices::U16(_) => {}
                Indices::U32(ref addons) => {
                    add_indices(values, addons, mesh.attributes[0].values.len());
                }
            },
        }
    } else {
        match other.indices.as_ref().expect("other indices") {
            Indices::U16(_) => {}
            Indices::U32(ref addons) => {
                add_indices(&mut vec![], addons, 0);
            }
        }
    }
    match mesh.attributes[0].values {
        bevy::render::mesh::VertexAttributeValues::Float3(ref mut values) => {
            match other.attributes[0].values {
                bevy::render::mesh::VertexAttributeValues::Float3(ref addons) => {
                    result = values.len()..values.len() + addons.len();
                    add_positions(values, addons);
                }
                _ => {panic!(" no positions on mesh")}
            }
        }
        _ => {}
    }
    match mesh.attributes[1].values {
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
    match mesh.attributes[2].values {
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

pub fn translate_mesh(mesh: &mut Mesh, position: Vec3) {
    match mesh.attributes[0].values {
        bevy::render::mesh::VertexAttributeValues::Float3(ref mut values) => {
            for value in values {
                value[0] = value[0] + position.x();
                value[1] = value[1] + position.y();
                value[2] = value[2] + position.z();
            }
        }
        _ => {}
    }
}
pub fn rotate_mesh(mesh: &mut Mesh, vertices: Range<usize>, rotation: Quat) {
    let center = get_center(&mesh, vertices.clone()); // because Range isn't fucking Copy...... TODO: make our own RANGE
    match mesh.attributes[0].values {
        bevy::render::mesh::VertexAttributeValues::Float3(ref mut values) => {
            for i in vertices {
                let new_pos =
                    rotation.mul_vec3(Vec3::from_slice_unaligned(&values[i]) - center) + center;
                values[i] = [new_pos.x(), new_pos.y(), new_pos.z()];
            }
        }
        _ => {}
    }
}
pub fn get_center(mesh: &Mesh, vertices: Range<usize>) -> Vec3 {
    match mesh.attributes[0].values {
        bevy::render::mesh::VertexAttributeValues::Float3(ref values) => {
            let mut average = Vec3::default();
            for i in vertices.clone() {
                // AGAIN??????????
                average += Vec3::from_slice_unaligned(&values[i]);
            }
            average /= vertices.len() as f32;
            average
        }
        _ => panic!("Vertices are in the wrong order"),
    }
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
        add_mesh(&mut mesh_one, &mesh_two);
        println!("{:?}", mesh_one);
    }
}
