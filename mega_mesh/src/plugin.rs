use std::path::PathBuf;

use bevy::{
    math::vec2,
    math::vec3,
    prelude::*,
    render::{
        mesh::Indices,
        mesh::VertexAttribute,
        pipeline::DynamicBinding,
        pipeline::PipelineDescriptor,
        pipeline::PipelineSpecialization,
        pipeline::PrimitiveTopology,
        pipeline::RenderPipeline,
        render_graph::base,
        render_graph::AssetRenderResourcesNode,
        render_graph::RenderGraph,
        renderer::RenderResources,
        shader::{self, ShaderDefs},
    },
};
use shader::{ShaderStage, ShaderStages};

use crate::mega_mesh::generate_mega_mesh;

const MEGA_MESH_PIPELINE_HANDLE: Handle<PipelineDescriptor> =
    Handle::from_u128(189483654150134513895864864820988150104);

#[derive(Debug, Clone)]
pub struct MegaMeshPlugin {
    pub size: f32,
    pub basecolor: Color,
    pub texture: Option<PathBuf>,
}
impl Default for MegaMeshPlugin {
    fn default() -> Self {
        MegaMeshPlugin {
            size: 50000.,
            basecolor: Color::WHITE,
            texture: None,
        }
    }
}

#[derive(Default)]
pub struct MegaMeshResource {
    pub config: MegaMeshPlugin,
    pub mesh_handle: Vec<Handle<Mesh>>,
    pub material_handle: Option<Handle<MegaMeshMaterial>>,
}

#[derive(RenderResources, ShaderDefs)]
pub struct MegaMeshMaterial {
    pub basecolor: Color,
    #[shader_def]
    pub texture: Option<Handle<Texture>>,
}

impl Plugin for MegaMeshPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(MegaMeshResource {
            config: self.clone(),
            ..Default::default()
        })
        .add_asset::<MegaMeshMaterial>()
        .add_system_to_stage(
            stage::POST_UPDATE,
            bevy::render::shader::asset_shader_defs_system::<MegaMeshMaterial>.system(),
        )
        .add_startup_system(mega_mesh_startup.system());
    }
}

fn mega_mesh_startup(
    mut commands: Commands,
    mut megamesh: ResMut<MegaMeshResource>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mega_materials: ResMut<Assets<MegaMeshMaterial>>,
    asset_server: Res<AssetServer>,
) {
    pipelines.set(MEGA_MESH_PIPELINE_HANDLE,PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            include_str!("../shaders/vert_shader.vert"),
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("../shaders/frag_shader.frag"),
        ))),
    }));
    render_graph.add_system_node(
        "mega_material",
        AssetRenderResourcesNode::<MegaMeshMaterial>::new(true),
    );
    render_graph
        .add_node_edge("mega_material", base::node::MAIN_PASS)
        .unwrap();

    let mm_specialized_pipeline =
        RenderPipelines::from_pipelines(vec![RenderPipeline::specialized(
            MEGA_MESH_PIPELINE_HANDLE,
            PipelineSpecialization {
                dynamic_bindings: vec![
                    // Transform
                    DynamicBinding {
                        bind_group: 2,
                        binding: 0,
                    },
                    // MegaMeshMaterial_basecolor
                    DynamicBinding {
                        bind_group: 3,
                        binding: 0,
                    },
                ],
                ..Default::default()
            },
        )]);

    if let Some(path) = megamesh.config.texture.clone() {
        let texture_handle = asset_server.load(path).unwrap();

        megamesh.material_handle = Some(mega_materials.add(MegaMeshMaterial {
            basecolor: megamesh.config.basecolor,
            texture: Some(texture_handle),
        }));
    } else {
        megamesh.material_handle = Some(mega_materials.add(MegaMeshMaterial {
            basecolor: megamesh.config.basecolor,
            texture: None,
        }));
    }
    // megamesh
    //     .mesh_handle
    //     .push(meshes.add(Mesh::from(shape::Quad {
    //         size: vec2(10000., 10000.),
    //         flip: false,
    //     })));

    // GENERATE MESH
    let meshmakers = generate_mega_mesh(10, 10);
    println!("mesh count: {:?}", meshmakers.len());

    for m in meshmakers {
        megamesh.mesh_handle.push(meshes.add(m.generate_mesh()));
    }

    // SPAWN IT
    for mesh in &megamesh.mesh_handle {
        println!("{:?}", mesh);
        commands
            .spawn(MeshComponents {
                mesh: *mesh,
                // draw: Draw {
                //     is_transparent: true,
                //     ..Default::default()
                // },
                transform: Transform::from_translation(vec3(0.0, 0.1, 1000.0)),
                render_pipelines: mm_specialized_pipeline.clone(),
                ..Default::default()
            })
            .with(megamesh.material_handle.unwrap());
    }
}
