use super::*;

pub struct DarkSkyComponentRegistry;
impl Plugin for DarkSkyComponentRegistry {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.register_component::<Player>();
    }
}
#[derive(Properties, Default)]
struct Player {
    pub health: f32,
}
