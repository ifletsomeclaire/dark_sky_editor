use std::{fs::File, io::BufReader, path::Path};

use bevy::{math::vec2, prelude::Texture, render::texture::TextureFormat};
use dds::DDS;

pub fn dds_to_texture(path: &str) -> Vec<Texture> {
    let mut textures = Vec::new();
    let file = File::open(Path::new(path)).unwrap();
    let mut reader = BufReader::new(file);

    let dds = DDS::decode(&mut reader).unwrap();

    let size = vec2(dds.header.width as f32, dds.header.height as f32);
    for i in 0..dds.header.mipmap_count {
        textures.push(Texture {
            data: dds.layers[i as usize].clone().into_iter().flat_map(|x| vec![x.r, x.b, x.g, x.a]).collect::<Vec<u8>>(),
            size: size / (i + 1) as f32,
            format: TextureFormat::Rgba8Unorm,
        })
    }

    textures
}
