use bevy::{math::vec3, prelude::*};

pub struct PlayerShip;

impl Plugin for PlayerShip {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_player.system())
        .add_system(move_player.system());
    }
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/flycatcher.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    // let aspect = texture.aspect();

    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        texture.size.x(),
        texture.size.y(),
    ))));
    let material_handle = materials.add(StandardMaterial {
        albedo_texture: Some(texture_handle),
        shaded: false,
        ..Default::default()
    });
    commands
        // textured quad - normal
        .spawn(PbrComponents {
            mesh: quad_handle,
            material: material_handle,
            transform: Transform::from_translation_rotation(
                Vec3::new(0.0, -1000.01, 8000.),
                Quat::from_rotation_x(-std::f32::consts::PI / 5.0),
            ),
            // draw: Draw {
            //     is_transparent: true,
            //     ..Default::default()
            // },
            ..Default::default()
        })
        .with(Player);
}

struct Player;

fn move_player(key: Res<Input<KeyCode>>, mut query: Query<(&Player, &mut Transform)>) {
    for (_, mut trans) in &mut query.iter() {
        if key.pressed(KeyCode::W) {
            trans.translate(vec3(0.0, 2.0, 0.0))
        }
        if key.pressed(KeyCode::A) {
            trans.translate(vec3(-2.0, 0.0, 0.0))
        }
        if key.pressed(KeyCode::S) {
            trans.translate(vec3(0.0, -2.0, 0.0))
        }
        if key.pressed(KeyCode::D) {
            trans.translate(vec3(2.0, 0.0, 0.0))
        }
    }
}