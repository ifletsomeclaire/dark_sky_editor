use bevy::{
    prelude::*,
    render::{renderer::RenderResources, shader::ShaderDefs},
};

#[derive(RenderResources, ShaderDefs)]
pub struct MeshMaterial {
    pub basecolor: Color,
    #[shader_def]
    pub texture1: Option<Handle<Texture>>,
    // #[shader_def]
    // pub texture2: Option<Handle<Texture>>,
    #[render_resources(ignore)]
    #[shader_def]
    pub shaded: bool,
}
