use std::{f32::consts::PI, ops::Mul};

use crate::{constants::EPSILON, vector::Vec3D};

#[derive(Debug, Clone, Copy)]
pub struct Quaternion {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Quaternion {
    pub const IDENTITY: Self = Self {
        w: 1.,
        x: 0.,
        y: 0.,
        z: 0.,
    };
    pub fn from_axis_angle(axis: Vec3D, angle_radians: f32) -> Self {
        let axis = axis.normalise();
        let half_angle = angle_radians / 2.;
        let sin_half_angle = half_angle.sin();
        Self {
            w: half_angle.cos(),
            x: axis.x * sin_half_angle,
            y: axis.y * sin_half_angle,
            z: axis.z * sin_half_angle,
        }
    }

    pub fn rotate(from: Vec3D, to: Vec3D, up: Vec3D) -> Self {
        let from = from.normalise();
        let to = to.normalise();

        let dot = from.dot(to);
        if (dot - -1.).abs() < EPSILON {
            return Self::from_axis_angle(up, PI);
        } else if (dot - 1.).abs() < EPSILON {
            return Self::IDENTITY;
        }

        Self::from_axis_angle(from.cross(to).normalise(), dot.acos())
    }
}

impl Default for Quaternion {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl PartialEq for Quaternion {
    fn eq(&self, other: &Self) -> bool {
        (self.w - other.w).abs() < EPSILON
            && (self.x - other.x).abs() < EPSILON
            && (self.y - other.y).abs() < EPSILON
            && (self.z - other.z).abs() < EPSILON
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

impl Mul for Quaternion {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
            x: self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            y: self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
            z: self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use super::*;

    #[test]
    fn test_quaternion_vector_multiplication() {
        let quaternion = Quaternion::from_axis_angle(Vec3D::X, PI / 2.);
        let vector = Vec3D::Y;
        let result = quaternion * vector;
        assert_eq!(result, Vec3D::Z);
    }

    #[test]
    fn test_quaternion_quaternion_multiplication() {
        let quaternion = Quaternion::from_axis_angle(Vec3D::X, PI / 2.);
        let result = quaternion * quaternion;
        assert_eq!(result, Quaternion::from_axis_angle(Vec3D::X, PI));
    }

    #[test]
    fn test_from_axis_angle() {}

    #[test]
    fn test_rotate() {
        let quaternion = Quaternion::rotate(Vec3D::X, Vec3D::Y, Vec3D::Z);
        assert_eq!(quaternion, Quaternion::from_axis_angle(Vec3D::Z, PI / 2.));
        let quaternion = Quaternion::rotate(Vec3D::Y, Vec3D::Z, Vec3D::Z);
        assert_eq!(quaternion, Quaternion::from_axis_angle(Vec3D::X, PI / 2.));
        let quaternion = Quaternion::rotate(Vec3D::Z, Vec3D::X, Vec3D::Z);
        assert_eq!(quaternion, Quaternion::from_axis_angle(Vec3D::Y, PI / 2.));
    }
}
