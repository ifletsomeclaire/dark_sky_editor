use bevy::{
    prelude::*,
    render::{renderer::RenderResources, shader::ShaderDefs},
};

#[derive(RenderResources, ShaderDefs)]
pub struct MeshMaterial {
    pub basecolor: Color,
    #[shader_def]
    pub texture: Option<Handle<Texture>>,
    #[render_resources(ignore)]
    #[shader_def]
    pub shaded: bool,
}
impl Default for MeshMaterial {
    fn default() -> Self {
        MeshMaterial {
            basecolor: Color::rgb(1.0, 1.0, 1.0),
            texture: None,
            shaded: true,
        }
    }
}