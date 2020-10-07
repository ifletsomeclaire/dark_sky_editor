use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, diagnostic::PrintDiagnosticsPlugin, prelude::*};
// use collision_rays::CollisionRay;
use components::DarkSkyComponentRegistry;
// use main_2d_camera::Main2dCamera;
use main_3d_camera::Main3dCamera;
// use meshie_ship_test::MeshieShipTest;
// use motion_test::MotionTest;
// use player_ship::PlayerShip;
use sectors::Sectors;
use starmap::StarMap;

mod collision_rays;
mod components;
mod equations_of_motion;
mod main_2d_camera;
mod main_3d_camera;
mod main_menu;
mod material;
mod meshie_ship_test;
mod motion_test;
mod player_ship;
mod sectors;
mod starmap;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_resource(ClearColor(Color::BLACK))
        .add_default_plugins()
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(PrintDiagnosticsPlugin::default())
        // .add_plugin(Main2dCamera)
        .add_plugin(Main3dCamera)
        .add_plugin(DarkSkyComponentRegistry)
        .add_plugin(StarMap)
        .add_plugin(Sectors)
        // .add_plugin(PlayerShip)
        // .add_plugin(MeshieShipTest)
        // .add_plugin(MotionTest)
        // .add_plugin(CollisionRay)
        // .add_plugin(main_menu::MainMenu)
        .run();
}
