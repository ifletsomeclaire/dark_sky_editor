use bevy::math::*;
#[derive(Debug, Default, Copy, Clone)]
pub struct Momentum {
    pub thrust: f32,
    pub max_rotation: f32,
    pub inertia: Vec2,
}
#[derive(Debug, Default, Copy, Clone)]
pub struct Destination {
    pub d: Vec3
}

impl EquationsOfMotion for Momentum {
    fn max_rotation(&self) -> f32 {
        self.max_rotation
    }
    fn inertia(&self) -> Vec2 {
        self.inertia
    }
    fn thrust(&self) -> f32 {
        self.thrust
    }
}
pub trait EquationsOfMotion {
    fn max_rotation(&self) -> f32;
    fn inertia(&self) -> Vec2;
    fn thrust(&self) -> f32;
    fn distance(&self, a: &Vec3, b: &Vec3) -> f32 {
        self.hypot(&[b[0] - a[0], b[1] - a[1], b[2] - a[2]])
    }
    fn hypot(&self, vector: &[f32]) -> f32 {
        let mut y: f32 = 0_f32;
        for i in 0..vector.len() {
            y += vector[i].powi(2);
        }
        y.sqrt()
    }
    fn ticks_to_turn(&self, angle: f32) -> f32 {
        angle.abs() / self.max_rotation()
    }
    fn ticks_to_stop(&self) -> f32 {
        self.inertia().length().abs() / self.thrust()
    }
    fn ticks_to_turn_and_stop(&self, angle: f32) -> f32 {
        self.ticks_to_turn(angle) + self.ticks_to_stop()
    }
    fn ticks_to_dest(&self, current: Vec3, dest: Vec3) -> f32 {
        self.distance(&current, &dest) / self.inertia().length().abs()
    }
    fn ticks_to_point_of_no_return(&self, current: Vec3, dest: Vec3) -> f32 {
        let angle = current.angle_between(dest);
        self.ticks_to_dest(current, dest) - self.ticks_to_turn_and_stop(angle)
    }
    fn intercept(&self, current: Vec3, target: Vec3, target_momentum: &Momentum) -> Vec2 {
        let ticks_to_target = self.ticks_to_dest(current, target);
        ticks_to_target * target_momentum.inertia
    }
    fn turn_to(&self, current: Vec3, dest: Vec3) -> (Vec3, f32) {
        let angle = current.angle_between(dest);
        // let max = self.max_rotation();
        let cross = current.cross(dest).normalize();
        // if angle.abs() < max {
            (cross, angle)
        // } else {
        //     (cross, max)
        // }
    }
}


pub trait QuatMath {
    fn from_to_vec3(from: Vec3, to: Vec3) -> Quat;
    fn default_to_vec3(to: Vec3) -> Quat;
}

impl QuatMath for Quat {
    fn from_to_vec3(from: Vec3, to: Vec3) -> Quat {
        let from_vec = from.normalize();
        let to_vec = to.normalize();
        let dot = from_vec.dot(to_vec);
        if dot >= 1.0 {
            return Quat::identity();
        }
        if dot < 1e-6_f32 - 1.0 {
            let mut axis = Vec3::unit_x().cross(from);
            if axis.length() == 0.0 {
                axis = Vec3::unit_y().cross(from);
            }
            return Quat::from_axis_angle(axis.normalize(), std::f32::consts::PI);
        }
        let angle = dot.acos();
        Quat::from_axis_angle(from_vec.cross(to_vec).normalize(), angle).normalize()
    }
    fn default_to_vec3(forward: Vec3) -> Quat {
        Quat::from_to_vec3(Vec3::unit_y(), forward)
    }
}
