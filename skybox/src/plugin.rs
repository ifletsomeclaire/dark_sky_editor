use std::path::PathBuf;

use bevy::{
    math::vec2,
    prelude::*,
    render::{
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
use shape::Quad;

const SKYBOX_PIPELINE_HANDLE: Handle<PipelineDescriptor> =
    Handle::from_u128(189483623150127713895864825450987265104);

#[derive(Debug, Clone)]
pub struct SkyboxPlugin {
    pub size: f32,
    pub basecolor: Color,
    pub texture: Option<PathBuf>,
}
impl Default for SkyboxPlugin {
    fn default() -> Self {
        SkyboxPlugin {
            size: 50000.,
            basecolor: Color::BLACK,
            texture: None,
        }
    }
}

#[derive(Default)]
struct SkyboxResource {
    pub plugin: SkyboxPlugin,
    pub mesh_handle: Option<Handle<Mesh>>,
    pub material_handle: Option<Handle<SkyboxMaterial>>,
}

#[derive(RenderResources, ShaderDefs)]
pub struct SkyboxMaterial {
    pub basecolor: Color,
    #[shader_def]
    pub texture: Option<Handle<Texture>>,
}

impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_resource(SkyboxResource {
            plugin: self.clone(),
            ..Default::default()
        })
        .add_asset::<SkyboxMaterial>()
        .add_system_to_stage(
            stage::POST_UPDATE,
            bevy::render::shader::asset_shader_defs_system::<SkyboxMaterial>.system(),
        )
        .add_startup_system(skybox_startup.system());
    }
}

fn skybox_startup(
    mut commands: Commands,
    mut skybox: ResMut<SkyboxResource>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut sky_materials: ResMut<Assets<SkyboxMaterial>>,
    asset_server: Res<AssetServer>,
) {
    pipelines.set(
        SKYBOX_PIPELINE_HANDLE,
        PipelineDescriptor::default_config(ShaderStages {
            vertex: shaders.add(Shader::from_glsl(
                ShaderStage::Vertex,
                include_str!("../shaders/vert_shader.vert"),
            )),
            fragment: Some(shaders.add(Shader::from_glsl(
                ShaderStage::Fragment,
                include_str!("../shaders/frag_shader.frag"),
            ))),
        }),
    );

    render_graph.add_system_node(
        "skybox_material",
        AssetRenderResourcesNode::<SkyboxMaterial>::new(true),
    );
    render_graph
        .add_node_edge("skybox_material", base::node::MAIN_PASS)
        .unwrap();

    let sky_specialized_pipeline =
        RenderPipelines::from_pipelines(vec![RenderPipeline::specialized(
            SKYBOX_PIPELINE_HANDLE,
            PipelineSpecialization {
                dynamic_bindings: vec![
                    // Transform
                    DynamicBinding {
                        bind_group: 2,
                        binding: 0,
                    },
                    // SkyboxMaterial_basecolor
                    DynamicBinding {
                        bind_group: 3,
                        binding: 0,
                    },
                ],
                ..Default::default()
            },
        )]);

    let s = skybox.plugin.size;
    let vertices = vec![
        // behind center
        ([-s, -s,  s], [ 0.,  0.,  -1.], [1., 0.]),
        ([ s, -s,  s], [ 0.,  0.,  -1.], [0., 0.]),
        ([-s,  s,  s], [ 0.,  0.,  -1.], [1., 1.]),
        ([ s,  s,  s], [ 0.,  0.,  -1.], [0., 1.]),
        // straight ahead
        ([ s, -s, -s], [ 0.,  0., 1.], [1., 0.]),
        ([-s, -s, -s], [ 0.,  0., 1.], [0., 0.]),
        ([ s,  s, -s], [ 0.,  0., 1.], [1., 1.]),
        ([-s,  s, -s], [ 0.,  0., 1.], [0., 1.]),
        // right
        ([ s, -s,  s], [ -1.,  0.,  0.], [1., 0.]),
        ([ s, -s, -s], [ -1.,  0.,  0.], [0., 0.]),
        ([ s,  s,  s], [ -1.,  0.,  0.], [1., 1.]),
        ([ s,  s, -s], [ -1.,  0.,  0.], [0., 1.]),
        // left
        ([-s, -s, -s], [1.,  0.,  0.], [1., 0.]),
        ([-s, -s,  s], [1.,  0.,  0.], [0., 0.]),
        ([-s,  s, -s], [1.,  0.,  0.], [1., 1.]),
        ([-s,  s,  s], [1.,  0.,  0.], [0., 1.]),
        // up
        ([-s,  s,  s], [ 0.,  -1.,  0.], [1., 0.]),
        ([ s,  s,  s], [ 0.,  -1.,  0.], [0., 0.]),
        ([-s,  s, -s], [ 0.,  -1.,  0.], [1., 1.]),
        ([ s,  s, -s], [ 0.,  -1.,  0.], [0., 1.]),
        // down
        ([-s, -s, -s], [ 0., 1.,  0.], [1., 0.]),
        ([ s, -s, -s], [ 0., 1.,  0.], [0., 0.]),
        ([-s, -s,  s], [ 0., 1.,  0.], [1., 1.]),
        ([ s, -s,  s], [ 0., 1.,  0.], [0., 1.]),
    ];

    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    for (position, normal, uv) in vertices {
        positions.push(position);
        normals.push(normal);
        uvs.push(uv);
    }

    let skymesh = Mesh {
        primitive_topology: PrimitiveTopology::TriangleList,
        attributes: vec![
            VertexAttribute::position(positions),
            VertexAttribute::normal(normals),
            VertexAttribute::uv(uvs),
        ],
        indices: Some(bevy::render::mesh::Indices::U32(vec![
            // FUCKING CLOCKWISE TRIANGLES
            2, 1, 0, 3, 1, 2, 
            6, 5, 4, 7, 5, 6, 
            10, 9, 8, 11, 9, 10, 
            14, 13, 12, 15, 13, 14, 
            18, 17, 16, 19, 17, 18, 
            22, 21, 20, 23, 21, 22,
            // 0, 1, 2, 2, 1, 3, 
            // 4, 5, 6, 6, 5, 7, 
            // 8, 9, 10, 10, 9, 11, 
            // 12, 13, 14, 14, 13, 15, 
            // 16, 17, 18, 18, 17, 19, 
            // 20, 21, 22, 22, 21, 23,
        ])),
    };
    skybox.mesh_handle = Some(meshes.add(skymesh));

    // skybox.mesh_handle = Some(meshes.add(Mesh::from(Quad {
    //     size: vec2(skybox.plugin.size, skybox.plugin.size),
    //     flip: false,
    // })));

    if let Some(path) = skybox.plugin.texture.clone() {
        let texture_handle = asset_server.load(path).unwrap();

        skybox.material_handle = Some(sky_materials.add(SkyboxMaterial {
            basecolor: skybox.plugin.basecolor,
            texture: Some(texture_handle),
        }));
    } else {
        skybox.material_handle = Some(sky_materials.add(SkyboxMaterial {
            basecolor: skybox.plugin.basecolor,
            texture: None,
        }));
    }

    commands
        .spawn(MeshComponents {
            mesh: skybox.mesh_handle.unwrap(),
            draw: Draw {
                is_transparent: true,
                ..Default::default()
            },
            render_pipelines: sky_specialized_pipeline,
            ..Default::default()
        })
        .with(skybox.material_handle.unwrap());
}
