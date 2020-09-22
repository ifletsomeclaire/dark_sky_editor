use bevy::{
    prelude::*,
    render::{renderer::RenderResources, shader::ShaderDefs},
};

#[derive(RenderResources, ShaderDefs)]
pub struct MeshMaterial {
    pub basecolor: Color,
    pub distance: f32,

    #[shader_def]
    pub texture1: Option<Handle<Texture>>,
    #[shader_def]
    pub texture2: Option<Handle<Texture>>,
    #[shader_def]
    pub texture3: Option<Handle<Texture>>,
    #[shader_def]
    pub texture4: Option<Handle<Texture>>,
    #[shader_def]
    pub texture5: Option<Handle<Texture>>,
    #[render_resources(ignore)]
    #[shader_def]
    pub shaded: bool,
}

pub const GLOBALMATERIAL: u128 = 468344185765435115;
#[derive(RenderResources, ShaderDefs)]
pub struct GlobalMaterial {
    pub distance: f32,
}