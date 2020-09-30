use bevy::prelude::*;
use components::DarkSkyComponentRegistry;
use main_2d_camera::Main2dCamera;
use main_3d_camera::Main3dCamera;
use meshie_ship_test::MeshieShipTest;
use starmap::StarMap;

mod components;
mod main_2d_camera;
mod main_3d_camera;
mod main_menu;
mod material;
mod meshie_ship_test;
mod starmap;
fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_resource(ClearColor(Color::BLACK))
        .add_default_plugins()
        // .add_plugin(Main2dCamera)
        .add_plugin(Main3dCamera)
        .add_plugin(DarkSkyComponentRegistry)
        .add_plugin(StarMap)
        .add_plugin(MeshieShipTest)
        // .add_plugin(main_menu::MainMenu)
        .run();
}
