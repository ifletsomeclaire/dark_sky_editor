use std::{fs::File, io::Write, path::Path};

use bevy::{
    asset::HandleId,
    asset::LoadState,
    diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin},
    math::vec2,
    math::vec3,
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
    render::texture::TextureFormat,
    sprite::TextureAtlasBuilder,
};
// use bevy_lyon::{
//     basic_shapes::{primitive, ShapeType},
//     TessellationMode,
// };

// use lyon::{lyon_tessellation::FillOptions, lyon_tessellation::StrokeOptions, math::Point};
use camera::{camera_movement, CameraMarker, MouseState};
// use image::*;
use material::MeshMaterial;
use mesh::{EditableMesh, MeshMaker};
use node_graph::{Graph, Ship};

// mod bevy_lyon;
mod camera;
mod material;
mod mesh;
mod node_graph;

#[derive(Default, Debug)]
struct MeshHandle {
    handle: Handle<Mesh>,
    texture_atlas: Vec<HandleId>,
}

#[derive(Default)]
pub struct RpgSpriteHandles {
    handles: Vec<HandleId>,
    atlas_loaded: bool,
}

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_default_plugins()
        .init_resource::<MouseState>()
        .init_resource::<MeshHandle>()
        .init_resource::<RpgSpriteHandles>()
        .add_asset::<MeshMaterial>()
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(PrintDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .add_startup_system(ta_setup.system())
        .add_system(camera_movement.system())
        .add_system(move_ship.system())
        .add_system(load_atlas.system())
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
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
    mut render_graph: ResMut<RenderGraph>,
    mut materials: ResMut<Assets<MeshMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut cmaterials: ResMut<Assets<ColorMaterial>>,
) {
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            include_str!("../shaders/forward.vert"),
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("../shaders/forward.frag"),
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

    let fly_handle = asset_server.load("assets/flycatcher.png").unwrap();
    let quail_handle = asset_server.load("assets/quail-color.png").unwrap();

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

    let material = materials.add(MeshMaterial {
        basecolor: Color::from(vec4(1.0, 1.0, 1.0, 1.0)),
        texture1: Some(fly_handle),
        texture2: Some(quail_handle),
        shaded: false,
        // ..Default::default()
    });

    for seed in 1..2 {
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
            for position in quad.get_vertex_positions().unwrap() {
                let pos = [
                    node.position.x() + position[0],
                    node.position.y() + position[1],
                    0.0,
                ];
                m_maker.vert_pos.push(pos);
            }
            // normals
            for norm in quad.get_vertex_normals().unwrap() {
                m_maker.vert_norm.push(norm);
            }
            // uvs
            for uv in quad.get_vertex_uvs().unwrap() {
                m_maker.vert_uvs.push(uv);
            }
            // colors
            for color in quad.get_vertex_colors().unwrap() {
                m_maker.vert_colors.push(color);
            }
            // texture index
            for _ in quad.get_vertex_textures().unwrap() {
                m_maker.vert_textures.push(node.texture);
            }
            for ind in quad.indices.unwrap() {
                m_maker.indices.push(ind + count as u32);
            }

            commands.spawn((
                Ship {
                    vert_indices: count..(count + 4),
                    texture_index: node.texture,
                },
                // Transform::from_translation(vec3(node.position.x(), node.position.y(), 0.0)),
            ));

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
                    ..Default::default()
                })
                .with(material);
        }
    }
    // println!("mesh {:#?}", mesh);
}
fn ta_setup(mut rpg_sprite_handles: ResMut<RpgSpriteHandles>, asset_server: Res<AssetServer>) {
    rpg_sprite_handles.handles = asset_server.load_asset_folder("assets").unwrap();
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

fn load_atlas(
    mut commands: Commands,
    mut rpg_sprite_handles: ResMut<RpgSpriteHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if rpg_sprite_handles.atlas_loaded {
        return;
    }

    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    if let Some(LoadState::Loaded(_)) =
        asset_server.get_group_load_state(&rpg_sprite_handles.handles)
    {
        for texture_id in rpg_sprite_handles.handles.iter() {
            let handle = Handle::from_id(*texture_id);
            let texture = textures.get(&handle).unwrap();
            texture_atlas_builder.add_texture(handle, &texture);
        }

        let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
        let texture_atlas_texture = texture_atlas.texture;
        let atlas_handle = texture_atlases.add(texture_atlas);

        // set up a scene to display our texture atlas
        // let vendor_handle = asset_server
        //     .get_handle("assets/flycatcher.png")
        //     .unwrap();
        // let vendor_index = texture_atlas.get_texture_index(vendor_handle).unwrap();
        // commands
        //     .spawn(SpriteSheetComponents {
        //         transform: Transform::from_scale(4.0).with_translation(Vec3::new(150.0, 0.0, 0.0)),
        //         sprite: TextureAtlasSprite::new(vendor_index as u32),
        //         texture_atlas: atlas_handle,
        //         ..Default::default()
        //     })
        //     // draw the atlas itself
        //     .spawn(SpriteComponents {
        //         material: materials.add(texture_atlas_texture.into()),
        //         transform: Transform::from_translation(Vec3::new(-300.0, 0.0, 0.0)),
        //         ..Default::default()
        //     });

        rpg_sprite_handles.atlas_loaded = true;

        let texas = textures.get(&texture_atlas_texture).unwrap();
        let texatlas = texture_atlases.get(&atlas_handle).unwrap();
        let fout = &mut File::create(&Path::new(&format!("texture_atlas.png"))).unwrap();
        let encoder = image::png::PngEncoder::new(fout);
        let ok = encoder.encode(
            &texas.data,
            texas.size.x() as u32,
            texas.size.y() as u32,
            image::ColorType::Rgba8,
        );

        let mut atlas_infos = Vec::new();
        let rects = &texatlas.textures;
        // for (h, u) in texatlas.texture_handles.as_ref().unwrap() {
        //     atlas_infos.push(AtlasInfo { filepath: format!("{:?}", h), rect: rects[u.to_owned()].clone()})
        // }
        for entry in walkdir::WalkDir::new("assets")
            .into_iter()
            .filter_map(|e| e.ok())
        {
            println!("{}", entry.path().display());
            if let Some(v_handle) = asset_server.get_handle(entry.path()) {
                if let Some(v_index) = texatlas.get_texture_index(v_handle) {
                    atlas_infos.push(AtlasInfo {
                        filepath: format!("{}", entry.path().display()),
                        rect: rects[v_index],
                    })
                }
            }
        }
        let outputfile = &mut File::create(&Path::new(&format!("texture_atlas.txt"))).unwrap();
        outputfile
            .write_all(format!("{:#?}", atlas_infos).as_bytes())
            .expect("else");
    }
}

#[derive(Debug)]
struct AtlasInfo {
    filepath: String,
    rect: bevy::sprite::Rect,
}
