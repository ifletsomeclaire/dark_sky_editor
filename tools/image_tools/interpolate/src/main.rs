use raster::{editor, BlendMode, PositionMode, *};

fn main() {
    let mut image1 =
        raster::open("X:/Rust Projects/dark_sky_editor/assets/test_ships/starling.png").unwrap();
    let mut image2 =
        raster::open("X:/Rust Projects/dark_sky_editor/assets/test_ships/scout.png").unwrap();

    let h = image2.height;
    let w = image2.width;

    if image1.height > image2.height {
        editor::resize(&mut image2, w, h, ResizeMode::ExactHeight).unwrap();
        raster::interpolate::resample(&mut image2, 2000, 2000, InterpolationMode::Bicubic).unwrap();
        raster::save(&image2, "resize2.png").unwrap();
    } else {
    }
    let normal = editor::blend(
        &image1,
        &image2,
        BlendMode::Normal,
        1.0,
        PositionMode::Center,
        0,
        0,
    )
    .unwrap();

    // All the other blend modes
    let difference = editor::blend(
        &image1,
        &image2,
        BlendMode::Difference,
        1.0,
        PositionMode::Center,
        0,
        0,
    )
    .unwrap();
    let multiply = editor::blend(
        &image1,
        &image2,
        BlendMode::Multiply,
        1.0,
        PositionMode::Center,
        0,
        0,
    )
    .unwrap();
    let overlay = editor::blend(
        &image1,
        &image2,
        BlendMode::Overlay,
        1.0,
        PositionMode::Center,
        0,
        0,
    )
    .unwrap();
    let screen = editor::blend(
        &image1,
        &image2,
        BlendMode::Screen,
        1.0,
        PositionMode::Center,
        0,
        0,
    )
    .unwrap();
    raster::save(&normal, "test_blend_normal.png").unwrap();
    raster::save(&difference, "test_blend_difference.png").unwrap();
    raster::save(&multiply, "test_blend_multiply.png").unwrap();
    raster::save(&overlay, "test_blend_overlay.png").unwrap();
    raster::save(&screen, "test_blend_screen.png").unwrap();
}
