use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin},
    math::vec2,
    prelude::*,
    render::camera::PerspectiveProjection,
};
// use bevy_lyon::{
//     basic_shapes::{primitive, ShapeType},
//     TessellationMode,
// };

// use lyon::{lyon_tessellation::FillOptions, lyon_tessellation::StrokeOptions, math::Point};
use camera::{camera_movement, CameraMarker, MouseState};
use mesh::MeshMaker;
use node_graph::Graph;

// mod bevy_lyon;
mod camera;
mod material;
mod mesh;
mod node_graph;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_default_plugins()
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(PrintDiagnosticsPlugin::default())
        .init_resource::<MouseState>()
        .add_startup_system(setup.system())
        .add_system(camera_movement.system())
        .run();
}

// TODO
// Generate Quad with texture references on vertex points (see if you can import a texture on vertex??)
// Connect 2D texture to Quad so that we can connect multiple different textures

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let texture_handle = asset_server.load("assets/flycatcher.png").unwrap();
    // commands.spawn(PbrComponents {
    //     mesh: meshes.add(Mesh::from(shape::Quad{ size: vec2(200. , 200. ), flip: false})),
    //     material: materials.add(texture_handle.into()),
    //     ..Default::default()
    // });

    commands
        .spawn(Camera3dComponents {
            transform: Transform::new(Mat4::face_toward(
                Vec3::new(0.0, -0.01, 150.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            )),
            perspective_projection: PerspectiveProjection {
                far: 200000.,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(CameraMarker);

    let material = materials.add(StandardMaterial {
        // albedo: Color::from(vec4(0.3, 0.4, 0.3, 1.0)),
        albedo_texture: Some(texture_handle),
        // shaded: true,
        ..Default::default()
    });
    let green_mat = materials.add(Color::rgb(0.3, 0.4, 0.3).into());
    let red_mat = materials.add(Color::rgb(0.8, 0.0, 0.0).into());
    let blue_mat = materials.add(Color::rgb(0.1, 0.4, 0.5).into());

    for seed in 1..2 {
        // let mut material = mat;
        // if seed % 3 == 0 {
        //     material = blue_mat;
        // } else if seed % 2 == 0 {
        //     material = red_mat;
        // }

        let graph = Graph::new(10, 50, 50, seed);
        println!("nodes: {}", graph.nodes.len());
        let mut meshmakers = Vec::new();
        let mut m_maker = MeshMaker::new();
        let mut count = 0;

        for node in &graph.nodes {
            let quad = Mesh::from(shape::Quad {
                size: vec2(5.0, 5.0),
                flip: false,
            });
            // positions
            match quad.attributes[0].values {
                bevy::render::mesh::VertexAttributeValues::Float3(ref qval) => {
                    for q in qval {
                        let pos = [node.position.x() + q[0], node.position.y() + q[1], 0.0];
                        m_maker.vert_pos.push(pos);
                    }
                }
                _ => {}
            }
            // normals
            match quad.attributes[1].values {
                bevy::render::mesh::VertexAttributeValues::Float3(ref qval) => {
                    for q in qval {
                        m_maker.vert_norm.push(q.clone());
                    }
                }
                _ => {}
            }
            // uvs
            match quad.attributes[2].values {
                bevy::render::mesh::VertexAttributeValues::Float2(ref qval) => {
                    for q in qval {
                        m_maker.vert_uvs.push(q.clone());
                    }
                }
                _ => {}
            }
            for ind in quad.indices.unwrap() {
                m_maker.indices.push(ind + count as u32);
            }
            count += 4;
            if count > 64000 {
                meshmakers.push(m_maker);
                m_maker = MeshMaker::new();
                count = 0;
            }
        }
        meshmakers.push(m_maker);
        println!("meshmakers: {}", meshmakers.len());
        // println!("vert: {}", positions.len());
        // println!("normal: {}", normals.len());
        // println!("uv: {}", uvs.len());
        // println!("indices: {}", indices.len());

        for meshmaker in &meshmakers {
            commands.spawn(PbrComponents {
                mesh: meshes.add(meshmaker.generate_mesh()),
                material: material,
                ..Default::default()
            });
        }
    }
    // println!("mesh {:#?}", mesh);

    // for node in &graph.nodes {
    //     commands.spawn(primitive(
    //         material,
    //         &mut meshes,
    //         ShapeType::Circle(node.size),
    //         TessellationMode::Fill(&FillOptions::default()),
    //         Transform::from_translation(vec3(node.position.x(), node.position.y(), 0.0)),
    //     ));
    // }
    // for connection in &graph.connections {
    //     commands.spawn(primitive(
    //         material,
    //         &mut meshes,
    //         ShapeType::Polyline {
    //             points: vec![
    //                 Point {
    //                     x: graph.nodes[connection.0 as usize].position.x(),
    //                     y: graph.nodes[connection.0 as usize].position.y(),
    //                     ..Default::default()
    //                 },
    //                 Point {
    //                     x: graph.nodes[connection.1 as usize].position.x(),
    //                     y: graph.nodes[connection.1 as usize].position.y(),
    //                     ..Default::default()
    //                 },
    //             ],
    //             closed: false,
    //         },
    //         TessellationMode::Stroke(&StrokeOptions::default().with_line_width(8.0)),
    //         Transform::default(),
    //     ));
    // }

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
