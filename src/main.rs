use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin},
    math::vec2,
    math::vec4,
    prelude::*,
    render::camera::PerspectiveProjection,
    render::pipeline::DynamicBinding,
    render::pipeline::PipelineDescriptor,
    render::pipeline::PipelineSpecialization,
    render::pipeline::RenderPipeline,
    render::render_graph::base,
    render::render_graph::AssetRenderResourcesNode,
    render::render_graph::RenderGraph,
    render::shader::asset_shader_defs_system,
    render::shader::ShaderStage,
    render::shader::ShaderStages,
};

use camera::{camera_movement, CameraMarker, MouseState};
use material::MeshMaterial;
use mesh::{EditableMesh, MeshMaker};
use node_graph::{Graph, Ship};
use texture_atlas::{load_atlas, ta_setup, AtlasInfo, AtlasSpriteHandles};

// mod bevy_lyon;
mod camera;
mod dds_import;
mod material;
mod mesh;
mod node_graph;
mod texture_atlas;

#[derive(Default, Debug)]
struct MeshHandle {
    handle: Handle<Mesh>,
}

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_default_plugins()
        .init_resource::<MouseState>()
        .init_resource::<MeshHandle>()
        // .init_resource::<AtlasSpriteHandles>()
        .add_asset::<MeshMaterial>()
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(PrintDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        // .add_startup_system(ta_setup.system())
        // .add_system(load_atlas.system())
        .add_system(camera_movement.system())
        // .add_system(move_ship.system())
        .add_system_to_stage(
            stage::POST_UPDATE,
            asset_shader_defs_system::<MeshMaterial>.system(),
        )
        .run();
}

// TODO
// Generate Quad with texture references on vertex points (see if you can import a texture on vertex??)
// Connect 2D texture to Quad so that we can connect multiple different textures

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut mesh_res: ResMut<MeshHandle>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,
    mut materials: ResMut<Assets<MeshMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            include_str!("../shaders/forward simple.vert"),
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("../shaders/forward simple.frag"),
        ))),
    }));
    render_graph.add_system_node(
        "mesh_material",
        AssetRenderResourcesNode::<MeshMaterial>::new(true),
    );
    render_graph
        .add_node_edge("mesh_material", base::node::MAIN_PASS)
        .unwrap();
    let specialized_pipeline = RenderPipelines::from_pipelines(vec![RenderPipeline::specialized(
        pipeline_handle,
        PipelineSpecialization {
            dynamic_bindings: vec![
                // Transform
                DynamicBinding {
                    bind_group: 2,
                    binding: 0,
                },
                // MeshMaterial_basecolor
                DynamicBinding {
                    bind_group: 3,
                    binding: 0,
                },
            ],
            ..Default::default()
        },
    )]);

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

    let mut texture_handles = Vec::new();
    for h in dds_import::dds_to_texture("assets/texture_atlas.dds") {
        texture_handles.push(textures.add(h))
    }
    println!("{:?}", texture_handles);

    for h in texture_handles {
        commands
            .spawn(MeshComponents {
                mesh: meshes.add(Mesh::from(shape::Quad {
                    size: vec2(100., 100.),
                    flip: false,
                })),
                render_pipelines: specialized_pipeline.clone(),
                ..Default::default()
            })
            .with(materials.add(MeshMaterial {
                basecolor: Color::from(vec4(1.0, 1.0, 1.0, 1.0)),
                texture1: Some(h),
                shaded: false,
            }));
    }

    let atlas_handle = asset_server.load("assets/texture_atlas.png").unwrap();
    // let fly_handle = asset_server.load("assets/ship/model 512.png").unwrap();
    // let quail_handle = asset_server.load("assets/quail-color.png").unwrap();

    // NOT ACTUALLY AVAILABLE BECAUSE IT'S NOT LOADED YET
    // using values that I checked from the png details....
    // let atlas_size = textures.get(&atlas_handle).unwrap().size;
    let atlas_size = vec2(4096., 4096.);

    let atlas_info = AtlasInfo::import_from_file("assets/texture_atlas.ron");

    let material = materials.add(MeshMaterial {
        basecolor: Color::from(vec4(1.0, 1.0, 1.0, 1.0)),
        texture1: Some(atlas_handle),
        // texture1: Some(texture_handles[0]),
        // texture2: None,
        shaded: false,
    });

    let mut z_value: f32 = 0.0;
    for seed in 1..2 {
        let graph = Graph::new(10, 10, 10, seed);
        println!("nodes: {}", graph.nodes.len());
        let mut meshmakers = Vec::new();
        let mut m_maker = MeshMaker::new();
        let mut count = 0;

        for node in &graph.nodes {
            let rect = atlas_info.textures[node.texture as usize].rect;
            // let quad = Mesh::from(shape::Quad {
            //     size: vec2(340., 380.),
            //     flip: false,
            // });
            let quad = Mesh::from(shape::Quad {
                size: vec2(rect.max[0] - rect.min[0], rect.max[1] - rect.min[1]),
                flip: false,
            });
            // positions
            for position in quad.get_vertex_positions().unwrap() {
                let pos = [
                    node.position.x() + position[0],
                    node.position.y() + position[1],
                    z_value,
                ];
                z_value += 0.5;
                m_maker.vert_pos.push(pos);
            }
            // normals
            for norm in quad.get_vertex_normals().unwrap() {
                m_maker.vert_norm.push(norm);
            }
            // uvs
            for uv in quad.get_vertex_uvs().unwrap() {
                let x = if uv[0] < 0.01 {
                    rect.min[0] / atlas_size[0]
                } else {
                    rect.max[0] / atlas_size[0]
                };
                let y = if uv[1] < 0.01 {
                    rect.min[1] / atlas_size[1]
                } else {
                    rect.max[1] / atlas_size[1]
                };
                m_maker.vert_uvs.push([x, y]);
                // m_maker.vert_uvs.push(uv);
            }
            // // colors
            // for color in quad.get_vertex_colors().unwrap() {
            //     m_maker.vert_colors.push(color);
            // }
            // // texture index
            // for _ in quad.get_vertex_textures().unwrap() {
            //     m_maker.vert_textures.push(1.0);
            // }
            for ind in quad.indices.unwrap() {
                m_maker.indices.push(ind + count as u32);
            }

            commands.spawn((Ship {
                vert_indices: count..(count + 4),
                texture_index: 1.0,
            },));

            count += 4;
            if count >= 64000 {
                meshmakers.push(m_maker);
                m_maker = MeshMaker::new();
                count = 0;
            }
        }
        meshmakers.push(m_maker);
        println!("meshmakers: {}", meshmakers.len());

        for meshmaker in &meshmakers {
            let mesh_handle = meshes.add(meshmaker.generate_mesh());
            mesh_res.handle = mesh_handle;
            commands
                .spawn(MeshComponents {
                    mesh: mesh_handle,
                    render_pipelines: specialized_pipeline.clone(),
                    draw: Draw {
                        is_transparent: true,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with(material);
        }
    }
}

fn move_ship(
    mut meshes: ResMut<Assets<Mesh>>,
    mesh_handle: Res<MeshHandle>,
    mut query: Query<&Ship>,
) {
    if let Some(mesh) = meshes.get_mut(&mesh_handle.handle) {
        if let Some(positions) = mesh.get_mut_vertex_positions() {
            for ship in &mut query.iter() {
                // if ship.texture_index < 1.5 {
                for index in ship.vert_indices.clone() {
                    positions[index as usize][0] -= 0.1;
                }
                // }
            }
        }
    }
}
