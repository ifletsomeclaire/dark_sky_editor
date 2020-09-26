use ron::{de::from_reader, ser::to_string_pretty, ser::PrettyConfig};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write, path::Path};

use bevy::{
    asset::HandleId, asset::LoadState, math::vec2, prelude::*, sprite::TextureAtlasBuilder,
};

// Almost literal copy of Bevy Texture Atlas example.....

#[derive(Default)]
pub struct AtlasSpriteHandles {
    handles: Vec<HandleId>,
    atlas_loaded: bool,
}

pub fn ta_setup(
    mut rpg_sprite_handles: ResMut<AtlasSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    rpg_sprite_handles.handles = asset_server.load_asset_folder("assets/ship").unwrap();
}

pub fn load_atlas(
    // mut commands: Commands,
    mut rpg_sprite_handles: ResMut<AtlasSpriteHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if rpg_sprite_handles.atlas_loaded {
        return;
    }

    let mut texture_atlas_builder = TextureAtlasBuilder {
        max_size: vec2(5000., 6000.),
        ..Default::default()
    };
    if let Some(LoadState::Loaded(_)) =
        asset_server.get_group_load_state(&rpg_sprite_handles.handles)
    {
        for texture_id in rpg_sprite_handles.handles.iter() {
            let handle = Handle::from_id(*texture_id);
            let texture = textures.get(&handle).unwrap();
            texture_atlas_builder.add_texture(handle, &texture);
        }

        let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
        let texture_atlas_texture = texture_atlas.texture;
        let atlas_handle = texture_atlases.add(texture_atlas);

        // set up a scene to display our texture atlas
        // let vendor_handle = asset_server
        //     .get_handle("assets/flycatcher.png")
        //     .unwrap();
        // let vendor_index = texture_atlas.get_texture_index(vendor_handle).unwrap();
        // commands
        //     .spawn(SpriteSheetComponents {
        //         transform: Transform::from_scale(4.0).with_translation(Vec3::new(150.0, 0.0, 0.0)),
        //         sprite: TextureAtlasSprite::new(vendor_index as u32),
        //         texture_atlas: atlas_handle,
        //         ..Default::default()
        //     })
        //     // draw the atlas itself
        //     .spawn(SpriteComponents {
        //         material: materials.add(texture_atlas_texture.into()),
        //         transform: Transform::from_translation(Vec3::new(-300.0, 0.0, 0.0)),
        //         ..Default::default()
        //     });

        rpg_sprite_handles.atlas_loaded = true;

        let texas = textures.get(&texture_atlas_texture).unwrap();
        let texatlas = texture_atlases.get(&atlas_handle).unwrap();
        print_texture_to_file(texas, "texture_atlas.png");

        let mut atlas_infos = AtlasInfo::new();
        let rects = &texatlas.textures;
        // for (h, u) in texatlas.texture_handles.as_ref().unwrap() {
        //     atlas_infos.push(AtlasInfo { filepath: format!("{:?}", h), rect: rects[u.to_owned()].clone()})
        // }
        for entry in walkdir::WalkDir::new("assets")
            .into_iter()
            .filter_map(|e| e.ok())
        {
            println!("{}", entry.path().display());
            if let Some(v_handle) = asset_server.get_handle(entry.path()) {
                if let Some(v_index) = texatlas.get_texture_index(v_handle) {
                    atlas_infos.textures.push(TextureDetails {
                        filepath: format!("{}", entry.path().display()),
                        rect: Rectangle {
                            min: rects[v_index].min,
                            max: rects[v_index].max,
                        },
                    })
                }
            }
        }
        atlas_infos.print_to_file("texture_atlas.ron");
    }
}

fn print_texture_to_file(texture: &Texture, path: &str) {
    let fout = &mut File::create(&Path::new(path)).unwrap();
    let encoder = image::png::PngEncoder::new(fout);
    let _ok = encoder.encode(
        &texture.data,
        texture.size.x() as u32,
        texture.size.y() as u32,
        image::ColorType::Rgba8,
    );
}

// import from Ron
#[derive(Debug, Deserialize, Serialize)]
pub struct AtlasInfo {
    pub textures: Vec<TextureDetails>,
}
impl AtlasInfo {
    fn new() -> Self {
        AtlasInfo {
            textures: Vec::new(),
        }
    }
    pub fn import_from_file(path: &str) -> Self {
        let f = File::open(path).expect("Failed opening file");
        let atlas_info: AtlasInfo = match from_reader(f) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load atlas info: {}", e);
                std::process::exit(1);
            }
        };
        atlas_info
    }
    fn print_to_file(&self, path: &str) {
        let pretty = PrettyConfig::new();
        let s = to_string_pretty(self, pretty).expect("Serialization failed");
        let outputfile = &mut File::create(&Path::new(&format!("texture_atlas.ron"))).unwrap();
        outputfile.write_all(s.as_bytes()).expect("else");
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct TextureDetails {
    pub filepath: String,
    pub rect: Rectangle,
}
#[derive(Debug, Default, Copy, Clone, Deserialize, Serialize)]
pub struct Rectangle {
    pub min: Vec2,
    pub max: Vec2,
}
