use std::ops::Mul;

use crate::vector::Vec3D;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Quaternion {
    pub fn new(w: f32, x: f32, y: f32, z: f32) -> Self {
        Self { w, x, y, z }
    }

    pub fn from_axis_angle(axis: Vec3D, angle: f32) -> Self {
        let half_angle = angle / 2.;
        let sin_half_angle = half_angle.sin();
        Self {
            w: half_angle.cos(),
            x: axis.x * sin_half_angle,
            y: axis.y * sin_half_angle,
            z: axis.z * sin_half_angle,
        }
    }
}

impl Mul<Vec3D> for Quaternion {
    type Output = Vec3D;
    fn mul(self, rhs: Vec3D) -> Self::Output {
        rhs + self.w * (2. * Vec3D::new(self.x, self.y, self.z).cross(rhs))
            + Vec3D::new(self.x, self.y, self.z)
                .cross(2. * Vec3D::new(self.x, self.y, self.z).cross(rhs))
    }
}
