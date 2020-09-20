use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin},
    math::vec3,
    prelude::*,
};
use bevy_lyon::{
    basic_shapes::{primitive, ShapeType},
    TessellationMode,
};

use lyon::{lyon_tessellation::FillOptions, lyon_tessellation::StrokeOptions, math::Point};
use node_graph::Graph;

mod bevy_lyon;
mod node_graph;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_default_plugins()
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(PrintDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(Camera2dComponents::default());

    let material = materials.add(Color::rgb(0.3, 0.4, 0.3).into());

    let graph = Graph::new(10, 200, 200);
    println!("nodes: {}", graph.nodes.len());
    for node in &graph.nodes {
        commands.spawn(primitive(
            material,
            &mut meshes,
            ShapeType::Circle(node.size),
            TessellationMode::Fill(&FillOptions::default()),
            Transform::from_translation(vec3(node.position.x(), node.position.y(), 0.0)),
        ));
    }
    for connection in &graph.connections {
        commands.spawn(primitive(
            material,
            &mut meshes,
            ShapeType::Polyline {
                points: vec![
                    Point {
                        x: graph.nodes[connection.0 as usize].position.x(),
                        y: graph.nodes[connection.0 as usize].position.y(),
                        ..Default::default()
                    },
                    Point {
                        x: graph.nodes[connection.1 as usize].position.x(),
                        y: graph.nodes[connection.1 as usize].position.y(),
                        ..Default::default()
                    },
                ],
                closed: false,
            },
            TessellationMode::Stroke(&StrokeOptions::default().with_line_width(8.0)),
            Transform::default(),
        ));
    }

    // commands
    //     .spawn(primitive(
    //         material,
    //         &mut meshes,
    //         ShapeType::Circle(40.0),
    //         TessellationMode::Stroke(&StrokeOptions::default().with_line_width(8.0)),
    //         Transform::from_translation(vec3(-50.0, 0.0, 0.0)),
    //     ))
    //     .spawn(primitive(
    //         material,
    //         &mut meshes,
    //         ShapeType::Circle(40.0),
    //         TessellationMode::Stroke(&StrokeOptions::default().with_line_width(8.0)),
    //         Transform::from_translation(vec3(50.0, 0.0, 0.0)),
    //     ))
    //     .spawn(primitive(
    //         material,
    //         &mut meshes,
    //         ShapeType::Polyline {
    //             points: vec![Point {
    //                 x: -10.0,
    //                 y: 0.0,
    //                 ..Default::default()
    //             }, Point {
    //                 x: 10.0,
    //                 y: 0.0,
    //                 ..Default::default()
    //             }],
    //             closed: false,
    //         },
    //         TessellationMode::Stroke(&StrokeOptions::default().with_line_width(8.0)),
    //         Transform::default(),
    //     ));
}
