use bevy::prelude::*;
use camera::DarkSkyCamera;
use components::DarkSkyComponentRegistry;
use starmap::StarMap;

mod camera;
mod components;
mod material;
mod starmap;
fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_resource(ClearColor(Color::BLACK))
        .add_default_plugins()
        .add_plugin(DarkSkyCamera)
        .add_plugin(DarkSkyComponentRegistry)
        .add_plugin(StarMap)
        .run();
}
