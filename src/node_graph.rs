use bevy::math::{vec2, Vec2};
use noise::*;
use utils::{NoiseMapBuilder, PlaneMapBuilder};

#[derive(Debug)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub connections: Vec<Connection>,
}
impl Graph {
    pub fn new(_node_count: i32, width: i32, height: i32, seed: u32) -> Self {
        let mut nodes = Vec::new();
        let mut connections = Vec::new();

        let noise = noise::Perlin::new().set_seed(seed);
        let map = PlaneMapBuilder::new(&noise)
            .set_size(10000, 10000)
            .set_x_bounds(-500.0, 500.0)
            .set_y_bounds(-500.0, 500.0)
            .build();

        for h in (-height)..(height) {
            for w in (-width)..(width) {
                if map.get_value((w + width) as usize, (h + height) as usize) > 0.0 {
                    nodes.push(Node::new(
                        vec2((w * 6) as f32, (h * 6) as f32),
                        if h > w { 1.0 } else { 2.0 },
                    ));
                }
            }
        }
        for (i, _node) in nodes.iter().enumerate() {
            if i + 1 != nodes.len() {
                connections.push(Connection(i as i32, (i + 1) as i32))
            }
        }
        Self { nodes, connections }
    }
}

#[derive(Debug)]
pub struct Node {
    pub position: Vec2,
    pub size: Vec2,
    pub texture: f32,
}
impl Node {
    fn new(position: Vec2, texture: f32) -> Self {
        Node {
            position,
            size: vec2(5.0, 5.0),
            texture,
        }
    }
}

#[derive(Debug)]
pub struct Connection(pub i32, pub i32);

pub struct Ship {
    pub vert_indices: std::ops::Range<u32>,
    pub texture_index: f32,
}
