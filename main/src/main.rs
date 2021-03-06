use std::path::PathBuf;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin},
    math::{vec2, vec3, vec4},
    prelude::*,
    render::{
        camera::{Camera, OrthographicProjection, PerspectiveProjection, WindowOrigin},
        pipeline::{DynamicBinding, PipelineDescriptor, PipelineSpecialization, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        shader::{ShaderStage, ShaderStages},
    },
};

use camera::{camera_movement, update_camera_distance, CameraMarker, MouseState};
use material::{GlobalMaterial, MeshMaterial, StarMaterial};
use mega_mesh::plugin::MegaMeshPlugin;
use mesh::{EditableMesh, MeshMaker};
use meshie::generator::{DistributionFn, MeshBuilder, MeshConfig};
use node_graph::{Graph, Ship};
use shape::Quad;
// use shapes::Skybox;
use skybox::plugin::SkyboxPlugin;
use texture_atlas::{load_atlas, ta_setup, AtlasInfo, AtlasSpriteHandles};

// mod bevy_lyon;
mod camera;
use camera::*;

// mod starscape;

mod dds_import;
mod material;
mod mesh;
mod node_graph;
mod othercamera;
// mod shapes;
mod texture_atlas;
use othercamera::*;
#[derive(Default, Debug)]
pub struct Handles {
    pub mesh_handle: Handle<Mesh>,
    pub ship_texture_mat: Handle<MeshMaterial>,
    pub global_mat: Handle<GlobalMaterial>,
}

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_resource(ClearColor(Color::BLACK))
        .add_default_plugins()
        .init_resource::<MouseState>()
        .init_resource::<Handles>()
        .init_resource::<MeshHandles>()
        // .init_resource::<AtlasSpriteHandles>()
        .add_asset::<MeshMaterial>()
        // .add_asset::<SkyboxMaterial>()
        .add_asset::<GlobalMaterial>()
        .add_asset::<StarMaterial>()
        .add_asset::<ColorMaterial>()
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(PrintDiagnosticsPlugin::default())
        // .add_plugin(SkyboxPlugin {
        //     size: 30000.,
        //     texture: Some(PathBuf::from("E:/Rust/Projects/dark_sky_editor/assets/STSCI-H-p1917b-q-5198x4801.png")),
        //     ..Default::default()
        // })
        // .add_plugin(MegaMeshPlugin::default())
        .add_startup_system(setup.system())
        .add_startup_system(setup_player.system())
        // .add_startup_system(background.system())
        // .add_startup_system(ta_setup.system())
        // .add_system(load_atlas.system())
        .add_system(camera_movement.system())
        .add_system(camera_fucking_blows.system())
        .add_system(update_camera_distance.system())
        .add_system(move_ship.system())
        .add_system(move_player.system())
        // .add_system(rts_camera_system.system())
        // .add_system(aspect_ratio.system())
        .add_system_to_stage(
            stage::POST_UPDATE,
            bevy::render::shader::asset_shader_defs_system::<MeshMaterial>.system(),
        )
        .add_system_to_stage(
            stage::POST_UPDATE,
            bevy::render::shader::asset_shader_defs_system::<StarMaterial>.system(),
        )
        // .add_system_to_stage(
        //     stage::POST_UPDATE,
        //     bevy::render::shader::asset_shader_defs_system::<SkyboxMaterial>.system(),
        // )
        .run();
}

// TODO
// Generate Quad with texture references on vertex points (see if you can import a texture on vertex??)
// Connect 2D texture to Quad so that we can connect multiple different textures

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut handle_res: ResMut<Handles>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,
    mut materials: ResMut<Assets<MeshMaterial>>,
    mut star_materials: ResMut<Assets<StarMaterial>>,
    // mut skymaterials: ResMut<Assets<SkyboxMaterial>>,
    mut globalmat: ResMut<Assets<GlobalMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    handle_res.global_mat = globalmat.add(GlobalMaterial { distance: 0.0 });

    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            include_str!("../../shaders/forward.vert"),
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("../../shaders/forward.frag"),
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
                // MeshMaterial_distance
                DynamicBinding {
                    bind_group: 3,
                    binding: 1,
                },
            ],
            ..Default::default()
        },
    )]);

    commands
        .spawn(Camera2dComponents {
            // global_transform
            transform: Transform::new(Mat4::face_toward(
                Vec3::new(0.0, -1000.01, 15000.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            )),
            // perspective_projection: PerspectiveProjection {
            //     far: f32::MAX,
            //     ..Default::default()
            // },
            orthographic_projection: OrthographicProjection {
                far: f32::MAX,

                ..Default::default()
            },

            ..Default::default()
        })
        .with(CameraMarker);

    let mut texture_handles = Vec::new();
    for h in dds_import::dds_to_texture("assets/texture_atlas.dds") {
        texture_handles.push(textures.add(h))
    }

    // let atlas_handle = asset_server.load("assets/texture_atlas.png").unwrap();
    // let fly_handle = asset_server.load("assets/ship/model 512.png").unwrap();
    // let quail_handle = asset_server.load("assets/quail-color.png").unwrap();

    // NOT ACTUALLY AVAILABLE BECAUSE IT'S NOT LOADED YET
    // using values that I checked from the png details....
    // let atlas_size = textures.get(&atlas_handle).unwrap().size;
    let atlas_size = vec2(4096., 4096.);

    let atlas_info = AtlasInfo::import_from_file("assets/texture_atlas.ron");

    let material = materials.add(MeshMaterial {
        basecolor: Color::from(vec4(1.0, 1.0, 1.0, 1.0)),
        texture1: Some(texture_handles[0]),
        texture2: Some(texture_handles[1]),
        texture3: Some(texture_handles[2]),
        texture4: Some(texture_handles[3]),
        texture5: Some(texture_handles[4]),
        shaded: false,
        distance: 0.0,
    });
    handle_res.ship_texture_mat = material;

    let mut z_value: f32 = 0.0;
    for seed in 1..2 {
        let graph = Graph::new(10, 60, 60, seed);
        println!("nodes: {}", graph.nodes.len());
        let mut meshmakers = Vec::new();
        let mut m_maker = MeshMaker::new();
        let mut count = 0;

        for node in &graph.nodes {
            let rect = atlas_info.textures[node.texture as usize].rect;
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
                m_maker.vert_uvs.push([
                    match uv[0] {
                        x if x < 0.0001 => rect.min[0] / atlas_size[0],
                        _ => rect.max[0] / atlas_size[0],
                    },
                    match uv[1] {
                        y if y < 0.0001 => rect.min[1] / atlas_size[1],
                        _ => rect.max[1] / atlas_size[1],
                    },
                ]);
                // m_maker.vert_uvs.push(uv);
            }

            match quad.indices.unwrap() {
                bevy::render::mesh::Indices::U16(_) => {}
                bevy::render::mesh::Indices::U32(i) => {
                    for ind in i {
                        m_maker.indices.push(ind + count as u32);
                    }
                }
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
            handle_res.mesh_handle = mesh_handle;
            commands
                .spawn(MeshComponents {
                    mesh: mesh_handle,
                    render_pipelines: specialized_pipeline.clone(),
                    draw: Draw {
                        is_transparent: true,
                        ..Default::default()
                    },
                    transform: Transform::from_rotation(Quat::from_rotation_z(
                        std::f32::consts::PI,
                    )),
                    // global_transform: GlobalTransform::from_rotation(Quat::from_rotation_y(2.0)),
                    ..Default::default()
                })
                .with(handle_res.global_mat)
                .with(material);
        }
    }

    let star_pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            include_str!("../../shaders/star_vert_shader.vert"),
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("../../shaders/star_frag_shader.frag"),
        ))),
    }));
    render_graph.add_system_node(
        "star_material",
        AssetRenderResourcesNode::<StarMaterial>::new(true),
    );
    render_graph
        .add_node_edge("star_material", base::node::MAIN_PASS)
        .unwrap();
    let star_specialized_pipeline =
        RenderPipelines::from_pipelines(vec![RenderPipeline::specialized(
            star_pipeline_handle,
            PipelineSpecialization {
                dynamic_bindings: vec![
                    // Transform
                    DynamicBinding {
                        bind_group: 2,
                        binding: 0,
                    },
                    // StarMaterial_basecolor
                    DynamicBinding {
                        bind_group: 3,
                        binding: 0,
                    },
                ],
                ..Default::default()
            },
        )]);

    let mat_handle = asset_server
        .load("../assets/STSCI-H-p1917b-q-5198x4801.png")
        .unwrap();
    let mut mesh_builder = MeshBuilder {
        texture_size: vec2(5198., 4807.),
        config: vec![],
    };
    mesh_builder.config.push(MeshConfig {
        count: 1000000,
        texture_position: bevy::sprite::Rect {
            min: vec2(592., 863.),
            max: vec2(601., 871.),
        },
        area: vec3(100000., 100000., 1000.),
        distribution: DistributionFn::Random,
    });
    mesh_builder.config.push(MeshConfig {
        count: 500000,
        texture_position: bevy::sprite::Rect {
            min: vec2(674., 857.),
            max: vec2(685., 869.),
        },
        area: vec3(100000., 100000., 1000.),
        distribution: DistributionFn::Random,
    });
    mesh_builder.config.push(MeshConfig {
        count: 500000,
        texture_position: bevy::sprite::Rect {
            min: vec2(526., 854.),
            max: vec2(543., 871.),
        },
        area: vec3(100000., 100000., 1000.),
        distribution: DistributionFn::Random,
    });
    mesh_builder.config.push(MeshConfig {
        count: 100000,
        texture_position: bevy::sprite::Rect {
            min: vec2(613., 880.),
            max: vec2(656., 917.),
        },
        area: vec3(100000., 100000., 1000.),
        distribution: DistributionFn::Random,
    });
    let mesh = mesh_builder.gen_mesh();
    // let mut rng = StdRng::from_entropy();
    let mesh_handle = meshes.add(mesh);
    commands
        .spawn(MeshComponents {
            mesh: mesh_handle,
            render_pipelines: star_specialized_pipeline,
            transform: Transform::from_scale(0.1),

            ..Default::default()
        })
        .with(star_materials.add(StarMaterial {
            texture: Some(mat_handle),
            ..Default::default()
        }));
}
#[derive(Default)]
struct MeshHandles {
    background: Option<Handle<Mesh>>,
}

fn aspect_ratio(mut meshes: ResMut<Assets<Mesh>>, handle: Res<MeshHandles>, windows: Res<Windows>) {
    if let Some(h) = &handle.background {
        if let Some(mesh) = meshes.get_mut(&h) {
            let Window { width, height, .. } = windows.get_primary().expect("No primary window");
            let width = *width as f32 * 3_f32;
            let height = *height as f32 * 3_f32;
            let left = -width;
            let right = width;
            let bottom = -height;
            let top = height;
            match mesh.attributes[0].values {
                bevy::render::mesh::VertexAttributeValues::Float3(ref mut vertices) => {
                    vertices[2] = [left, top, 0.0];
                    vertices[3] = [right, top, 0.0];
                    vertices[1] = [left, bottom, 0.0];
                    vertices[0] = [right, bottom, 0.0];
                }
                _ => (),
            }
        }
    }
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/flycatcher.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let aspect = texture.aspect();

    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        texture.size.x(),
        texture.size.y(),
    ))));
    let material_handle = materials.add(StandardMaterial {
        albedo_texture: Some(texture_handle),
        shaded: false,
        ..Default::default()
    });
    commands
        // textured quad - normal
        .spawn(PbrComponents {
            mesh: quad_handle,
            material: material_handle,
            transform: Transform::from_translation_rotation(
                Vec3::new(0.0, -1000.01, 8000.),
                Quat::from_rotation_x(-std::f32::consts::PI / 5.0),
            ),
            draw: Draw {
                is_transparent: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Player);
}

struct Player;

fn move_player(key: Res<Input<KeyCode>>, mut query: Query<(&Player, &mut Transform)>) {
    for (_, mut trans) in &mut query.iter() {
        if key.pressed(KeyCode::W) {
            trans.translate(vec3(0.0, 2.0, 0.0))
        }
        if key.pressed(KeyCode::A) {
            trans.translate(vec3(-2.0, 0.0, 0.0))
        }
        if key.pressed(KeyCode::S) {
            trans.translate(vec3(0.0, -2.0, 0.0))
        }
        if key.pressed(KeyCode::D) {
            trans.translate(vec3(2.0, 0.0, 0.0))
        }
    }
}

// fn background(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut backgroundhandle: ResMut<MeshHandles>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut textures: ResMut<Assets<Texture>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     let mut mesh = Mesh::from(shapes::Skybox { size: 10000000. });
//     let quad_handle = meshes.add(mesh);
//     let red_material_handle = materials.add(StandardMaterial {
//         albedo: Color::rgba(0.0, 0.0, 0.0, 1.0),
//         // albedo_texture: Some(texture_handle),
//         shaded: false,
//         ..Default::default()
//     });
//     backgroundhandle.background = Some(quad_handle);
//     commands
//         // textured quad - normal
//         .spawn(PbrComponents {
//             mesh: quad_handle,
//             material: red_material_handle,
//             draw: Draw {
//                 is_transparent: true,
//                 ..Default::default()
//             },
//             ..Default::default()
//         });
// }

fn move_ship(mut meshes: ResMut<Assets<Mesh>>, handles: Res<Handles>, mut query: Query<&Ship>) {
    if let Some(mesh) = meshes.get_mut(&handles.mesh_handle) {
        if let Some(positions) = mesh.get_mut_vertex_positions() {
            for ship in &mut query.iter() {
                // if ship.texture_index < 1.5 {
                for index in ship.vert_indices.clone() {
                    positions[index as usize][1] += 0.7;
                }
                // }
            }
        }
    }
}
