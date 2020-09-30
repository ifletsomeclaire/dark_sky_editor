use bevy::prelude::*;
use camera::DarkSkyCamera;
use components::DarkSkyComponentRegistry;
use player_ship::PlayerShip;
use starmap::StarMap;

mod camera;
mod components;
mod material;
mod starmap;
mod player_ship;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_resource(ClearColor(Color::BLACK))
        .add_default_plugins()
        .add_plugin(DarkSkyCamera)
        .add_plugin(DarkSkyComponentRegistry)
        .add_plugin(StarMap)
        .add_plugin(PlayerShip)
        .run();
}
