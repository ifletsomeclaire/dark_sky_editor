use bevy::{
    math::{vec2, Vec2, Vec3},
    prelude::Commands,
};
use noise::*;
use utils::{NoiseMapBuilder, PlaneMapBuilder};

// use crate::bevy_lyon::basic_shapes::primitive;

#[derive(Debug)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub connections: Vec<Connection>,
}
impl Graph {
    pub fn new(node_count: i32, width: i32, height: i32, seed: u32) -> Self {
        let mut nodes = Vec::new();
        let mut connections = Vec::new();

        let noise = noise::Perlin::new().set_seed(seed);
        let map = PlaneMapBuilder::new(&noise)
            .set_size(10000, 10000)
            .set_x_bounds(-50.0, 50.0)
            .set_y_bounds(-50.0, 50.0)
            .build();

        for h in (-height)..(height) {
            for w in (-width)..(width) {
                // println!("noise: {}", noise.get([(w as f64) / 0.2, (h as f64) / 0.8]));
                if map.get_value((w+width) as usize, (h+height) as usize) > 0.0 {
                    nodes.push(Node::new(vec2((w) as f32, (h) as f32)));
                }
                // if noise.get([(w as f64) * 0.754275, (h as f64) * 0.428]) > 0.0 {
                //     nodes.push(Node::new(vec2((w * 2) as f32, (h * 2) as f32)));
                // }
            }
        }
        // for (i, _node) in nodes.iter().enumerate() {
        //     if i + 1 != nodes.len() {
        //         connections.push(Connection(i as i32, (i + 1) as i32))
        //     }
        // }

        Self { nodes, connections }
    }
}

#[derive(Debug)]
pub struct Node {
    pub position: Vec2,
    pub size: f32,
}
// all nodes are circles
impl Node {
    fn new(position: Vec2) -> Self {
        Node {
            position,
            size: 1.0,
        }
    }
}

#[derive(Debug)]
pub struct Connection(pub i32, pub i32);
