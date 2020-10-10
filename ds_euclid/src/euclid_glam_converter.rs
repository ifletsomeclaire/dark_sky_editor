use bevy::math::{Vec3, vec3};

// pub trait EuclidFrom<T> {
//     fn from(val: T) -> Self;
// }

// pub trait EuclidInto<U> {
//     fn euclid_into(self) -> U;
// }

// impl<T, U> EuclidInto<U> for T
// where
//     U: EuclidFrom<T>,
// {
//     fn euclid_into(self) -> U {
//         U::from(self)
//     }
// }

// impl EuclidFrom<Vec3> for euclid::Vector3D<f32, euclid::UnknownUnit> {
//     fn from(val: Vec3) -> Self {
//         euclid::Vector3D::new(val.x(), val.y(), val.z())
//     }
// }
// impl EuclidFrom<euclid::Vector3D<f32, euclid::UnknownUnit>> for Vec3 {
//     fn from(val: euclid::Vector3D<f32, euclid::UnknownUnit>) -> Self {
//         vec3(val.x, val.y, val.z)
//     }
// }
// impl EuclidFrom<Vec3> for euclid::Point3D<f32, euclid::UnknownUnit> {
//     fn from(val: Vec3) -> Self {
//         euclid::Point3D::new(val.x(), val.y(), val.z())
//     }
// }


pub struct EuclidVec3(euclid::Vector3D<f32, euclid::UnknownUnit>);
pub struct GlamVec3(Vec3);

impl From<GlamVec3> for EuclidVec3 {
    fn from(val: GlamVec3) -> Self {
        EuclidVec3(euclid::Vector3D::new(val.0.x(), val.0.y(), val.0.z()))
    }
}
impl From<EuclidVec3> for GlamVec3 {
    fn from(val: EuclidVec3) -> Self {
        GlamVec3(vec3(val.0.x, val.0.y, val.0.z))
    }
}