use bevy::{
    math::vec3,
    prelude::*,
    render::camera::Camera,
    render::mesh::VertexAttribute,
    render::mesh::VertexAttributeValues,
    window::{CursorMoved, WindowId},
};
use rays::Ray3d;

use crate::main_3d_camera::CameraMarker;
#[derive(Debug)]
struct ObjectInfoBox {
    text: String,
}

impl Default for ObjectInfoBox {
    fn default() -> Self {
        Self {
            text: String::from("none selected"),
        }
    }
}
struct RayBall;
#[derive(Debug)]
struct InteractionDisplay {
    text: String,
}
impl Default for InteractionDisplay {
    fn default() -> Self {
        Self {
            text: String::from("placeholder"),
        }
    }
}

#[derive(Default)]
pub struct CursorState {
    cursor_event_reader: EventReader<CursorMoved>,
}
pub struct MeshieShipTest;

impl Plugin for MeshieShipTest {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ObjectInfoBox>()
            .init_resource::<CursorState>()
            .add_startup_system(start.system())
            .add_system(selecty.system());
    }
}

struct Selectable;

fn start(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ray_ball = Mesh::from(shape::Cube { size: 100. });

    let mut cube_mesh = Mesh::from(shape::Quad {
        flip: false,
        size: Vec2::new(1000., 1000.),
    });
    let other = Mesh::from(shape::Quad {
        flip: false,
        size: Vec2::new(1000., 1000.),
    });
    meshie::translate_mesh(&mut cube_mesh, Vec3::new(1200., 0.0, 0.0));
    meshie::add_mesh(&mut cube_mesh, &other);
    let cube_handle = meshes.add(cube_mesh);
    let ray_ball_handle = meshes.add(ray_ball);

    commands
        .spawn(PbrComponents {
            mesh: cube_handle,
            material: materials.add(StandardMaterial {
                ..Default::default()
            }),
            transform: Transform::from_translation(Vec3::new(10.0, 10.0, 1000.0)),
            ..Default::default()
        })
        .with(Selectable)
        .with(InteractionDisplay {
            text: String::from("MESHIE!!!"),
        });
    commands
        .spawn(PbrComponents {
            mesh: ray_ball_handle,

            material: materials.add(StandardMaterial {
                albedo: Color::WHITE,
                ..Default::default()
            }),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .with(RayBall);
}

fn selecty(
    time: Res<Time>,
    click: Res<Input<MouseButton>>,
    mouse_pos: Res<Events<CursorMoved>>,
    mut state: ResMut<CursorState>,
    mut obj_info_state: ResMut<ObjectInfoBox>,
    meshes: Res<Assets<Mesh>>,
    windows: Res<Windows>,
    mut query: Query<(
        &Handle<Mesh>,
        &mut InteractionDisplay,
        &Transform,
        &mut Selectable,
    )>,
    mut cam_query: Query<(&CameraMarker, &Camera, &Transform)>,
    mut ray_ball_query: Query<(&RayBall, &mut Transform)>,
) {
    for (mesh_handle, mut interaction, transform, _) in &mut query.iter() {
        if click.pressed(MouseButton::Left) {
            for event in state.cursor_event_reader.iter(&mouse_pos) {
                for (_, cam, ctran) in &mut cam_query.iter() {
                    let window = windows.get(WindowId::primary()).unwrap();
                    let cursor_pos_ndc: Vec3 = ((event.position
                        / Vec2::from([window.width as f32, window.height as f32]))
                        * 2.0
                        - Vec2::from([1.0, 1.0]))
                    .extend(1.0);

                    let ndc_to_world: Mat4 = *ctran.value() * cam.projection_matrix.inverse();
                    let cursor_position: Vec3 = ndc_to_world.transform_point3(cursor_pos_ndc);

                    let ray_direction = cursor_position - ctran.translation();

                    let pick_ray = Ray3d::new(ctran.translation(), ray_direction);

                    let dist_from_cam_to_mesh =
                        distance(&ctran.translation(), &transform.translation());

                    let m_pos = ctran.translation()
                        + (pick_ray.direction().normalize() * dist_from_cam_to_mesh);
                    let dist_from_mouse_to_mesh_center = distance(&m_pos, &transform.translation());
                    if dist_from_mouse_to_mesh_center < 500. {
                        println!("{:#?}", obj_info_state.text);

                        obj_info_state.text = interaction.text.clone();
                    }

                    for (_, mut ray_ball_trans) in &mut ray_ball_query.iter() {
                        ray_ball_trans.set_translation(
                            ctran.translation()
                                + (pick_ray.direction().normalize() * dist_from_cam_to_mesh),
                        );
                    }
                }
            }
        }
    }
}

pub fn distance(a: &Vec3, b: &Vec3) -> f32 {
    let dist = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    hypot(&dist)
}
pub fn hypot(arguments: &[f32]) -> f32 {
    let mut y: f32 = 0_f32;
    let len = arguments.len();

    for i in 0..len {
        y += arguments[i].powi(2);
    }

    y.sqrt()
}

// if let Some(mesh) = meshes.get(mesh_handle) {
//                         let vertex_positions: Vec<[f32; 3]> = mesh
//                             .attributes
//                             .iter()
//                             .filter(|attribute| attribute.name == VertexAttribute::POSITION)
//                             .filter_map(|attribute| match &attribute.values {
//                                 VertexAttributeValues::Float3(positions) => Some(positions.clone()),
//                                 _ => panic!("Unexpected vertex types in VertexAttribute::POSITION"),
//                             })
//                             .last()
//                             .unwrap();

//                         if let Some(indices) = &mesh.indices {
//                             let mut min_pick_distance = f32::MAX;

//                             let mesh_to_world = transform.translation();
//                             let ind = match indices {
//                                 bevy::render::mesh::Indices::U32(i) => Some(i),
//                                 _ => None,
//                             }
//                             .unwrap();
//                             for index in ind.chunks(3) {
//                                 // Construct a triangle in world space using the mesh data
//                                 let mut world_vertices: [Vec3; 3] =
//                                     [Vec3::zero(), Vec3::zero(), Vec3::zero()];
//                                 for i in 0..3 {
//                                     world_vertices[i] = mesh_to_world
//                                         + (Vec3::from(vertex_positions[index[i] as usize]));
//                                 }
//                                 let world_triangle = Triangle::from(world_vertices);

//                                 // Run the raycast on the ray and triangle
//                                 if let Some(intersection) =
//                                     raycast_geometric(&pick_ray, &world_triangle)
//                                 {
//                                     let distance: f32 = (*intersection.origin()
//                                         - *pick_ray.origin())
//                                     .length()
//                                     .abs();

//                                     if distance < min_pick_distance {
//                                         // println!(
//                                         //     "raycast_geometric \n{:#?}\n{:#?}",
//                                         //     pick_ray, world_triangle
//                                         // );
//                                         println!(
//                                             "INTERACTION!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!"
//                                         );
//                                     }
//                                 }
//                             }
//                         }
//                     }
pub fn raycast_geometric(ray: &Ray3d, triangle: &Triangle) -> Option<Ray3d> {
    // Source: https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/ray-triangle-intersection-geometric-solution
    let epsilon = 0.0001;

    // compute plane's normal
    let vector_v0_to_v1: Vec3 = triangle.v1 - triangle.v0;
    let vector_v0_to_v2: Vec3 = triangle.v2 - triangle.v0;
    // no need to normalize
    let triangle_normal = vector_v0_to_v1.cross(vector_v0_to_v2); // N

    // Step 1: finding P

    // check if ray and plane are parallel ?
    let n_dot_ray_direction = triangle_normal.dot(*ray.direction());
    if n_dot_ray_direction.abs() < epsilon {
        return None;
    }

    // compute d parameter using equation 2
    let d = triangle_normal.dot(triangle.v0);

    // compute t (equation 3)
    let t = (triangle_normal.dot(*ray.origin()) + d) / n_dot_ray_direction;
    // check if the triangle is in behind the ray
    if t < 0.0 {
        return None;
    } // the triangle is behind

    // compute the intersection point using equation 1
    let point_intersection = *ray.origin() + t * *ray.direction();

    // Step 2: inside-outside test

    // edge 0
    let edge0 = triangle.v1 - triangle.v0;
    let vp0 = point_intersection - triangle.v0;
    let cross = edge0.cross(vp0);
    if triangle_normal.dot(cross) < 0.0 {
        return None;
    } // P is on the right side

    // edge 1
    let edge1 = triangle.v2 - triangle.v1;
    let vp1 = point_intersection - triangle.v1;
    let cross = edge1.cross(vp1);
    if triangle_normal.dot(cross) < 0.0 {
        return None;
    } // P is on the right side

    // edge 2
    let edge2 = triangle.v0 - triangle.v2;
    let vp2 = point_intersection - triangle.v2;
    let cross = edge2.cross(vp2);
    if triangle_normal.dot(cross) < 0.0 {
        return None;
    } // P is on the right side;

    Some(Ray3d::new(point_intersection, triangle_normal))
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
}

impl From<(Vec3, Vec3, Vec3)> for Triangle {
    fn from(vertices: (Vec3, Vec3, Vec3)) -> Self {
        Triangle {
            v0: vertices.0,
            v1: vertices.1,
            v2: vertices.2,
        }
    }
}

impl From<Vec<Vec3>> for Triangle {
    fn from(vertices: Vec<Vec3>) -> Self {
        Triangle {
            v0: *vertices.get(0).unwrap(),
            v1: *vertices.get(1).unwrap(),
            v2: *vertices.get(2).unwrap(),
        }
    }
}

impl From<[Vec3; 3]> for Triangle {
    fn from(vertices: [Vec3; 3]) -> Self {
        Triangle {
            v0: vertices[0],
            v1: vertices[1],
            v2: vertices[2],
        }
    }
}

pub mod rays {
    use bevy::prelude::*;

    /// A 3D ray, with an origin and direction. The direction is guaranteed to be normalized.
    #[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
    pub struct Ray3d {
        origin: Vec3,
        direction: Vec3,
    }

    impl Ray3d {
        /// Constructs a `Ray3d`, normalizing the direction vector.
        pub fn new(origin: Vec3, direction: Vec3) -> Self {
            Ray3d {
                origin,
                direction: direction.normalize(),
            }
        }
        /// Position vector describing the ray origin
        pub fn origin(&self) -> &Vec3 {
            &self.origin
        }
        /// Unit vector describing the ray direction
        pub fn direction(&self) -> &Vec3 {
            &self.direction
        }
        pub fn to_transform(&self) -> Mat4 {
            let position = self.origin;
            let normal = self.direction;
            let up = Vec3::from([0.0, 1.0, 0.0]);
            let axis = up.cross(normal).normalize();
            let angle = up.dot(normal).acos();
            let epsilon = 0.0001;
            let new_rotation = if angle.abs() > epsilon {
                Quat::from_axis_angle(axis, angle)
            } else {
                Quat::default()
            };
            Mat4::from_rotation_translation(new_rotation, position)
        }
    }
}
