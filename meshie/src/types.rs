pub(crate) struct Meshie {
    pub(crate) vertices: Vec<Vertex>,
    pub(crate) indices: Vec<Triangle>,
}

pub(crate) struct Triangle {

}

pub(crate) struct Vertex {
    pub(crate) position: Vec3,
    pub(crate) normal: Vec3,
    pub(crate) uv: Vec2,
}