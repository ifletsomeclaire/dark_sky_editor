use bevy::render::{
    pipeline::DynamicBinding, pipeline::PipelineDescriptor, pipeline::PipelineSpecialization,
    pipeline::RenderPipeline, render_graph::base, render_graph::AssetRenderResourcesNode,
    render_graph::RenderGraph, shader::ShaderStage, shader::ShaderStages,
};

use bevy::math::*;
use material::StarMaterial;
use meshie::generator::{DistributionFn, MeshBuilder, MeshConfig};

use super::*;

pub struct StarMap;
pub const STAR_PIPELINE_HANDLE: Handle<PipelineDescriptor> =
    Handle::from_u128(189483654225434513895898134820988150104);

impl Plugin for StarMap {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<StarMaterial>()
            .add_startup_system(start.system())
            .add_system_to_stage(
                stage::POST_UPDATE,
                bevy::render::shader::asset_shader_defs_system::<StarMaterial>.system(),
            );
    }
}

fn start(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,
    mut star_materials: ResMut<Assets<StarMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // let star_pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
    pipelines.set(STAR_PIPELINE_HANDLE,PipelineDescriptor::default_config(ShaderStages {
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
            STAR_PIPELINE_HANDLE,
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
        count: 100000,
        texture_position: bevy::sprite::Rect {
            min: vec2(592., 863.),
            max: vec2(601., 871.),
        },
        area: vec3(10000., 10000., 1000.),
        distribution: DistributionFn::Random,
    });
    mesh_builder.config.push(MeshConfig {
        count: 50000,
        texture_position: bevy::sprite::Rect {
            min: vec2(674., 857.),
            max: vec2(685., 869.),
        },
        area: vec3(10000., 10000., 1000.),
        distribution: DistributionFn::Random,
    });
    mesh_builder.config.push(MeshConfig {
        count: 50000,
        texture_position: bevy::sprite::Rect {
            min: vec2(526., 854.),
            max: vec2(543., 871.),
        },
        area: vec3(10000., 10000., 1000.),
        distribution: DistributionFn::Random,
    });
    mesh_builder.config.push(MeshConfig {
        count: 10000,
        texture_position: bevy::sprite::Rect {
            min: vec2(613., 880.),
            max: vec2(656., 917.),
        },
        area: vec3(10000., 10000., 1000.),
        distribution: DistributionFn::Random,
    });
    let mesh = mesh_builder.gen_mesh();
    let mesh_handle = meshes.add(mesh);
    commands
        .spawn(MeshComponents {
            mesh: mesh_handle,
            render_pipelines: star_specialized_pipeline,
            transform: Transform::from_translation_rotation_scale(vec3(0.0, 0.0, -100000.), Quat::default(), 1.0),
            draw: Draw {
                is_transparent: true,
                ..Default::default()
            },

            ..Default::default()
        })
        .with(star_materials.add(StarMaterial {
            texture: Some(mat_handle),
            ..Default::default()
        }));
}
