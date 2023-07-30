use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::{constants::EPSILON, quaternion::Quaternion};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3D {
    pub const ONE: Self = Self::triple(1.);
    pub const ZERO: Self = Self::triple(0.);
    pub const X: Self = Self::new(1., 0., 0.);
    pub const Y: Self = Self::new(0., 1., 0.);
    pub const Z: Self = Self::new(0., 0., 1.);

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub const fn triple(xyz: f32) -> Self {
        Self::new(xyz, xyz, xyz)
    }

    pub fn length(self) -> f32 {
        self.x.hypot(self.y).hypot(self.z)
    }

    pub fn normalise(self) -> Self {
        self / self.length()
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn reflect(self, plane_normal: Self) -> Self {
        self - plane_normal * 2. * self.dot(plane_normal)
    }
}

impl PartialEq for Vec3D {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < EPSILON
            && (self.y - other.y).abs() < EPSILON
            && (self.z - other.z).abs() < EPSILON
    }
}

impl PartialOrd for Vec3D {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.length().partial_cmp(&other.length()).unwrap())
    }
}

impl Add for Vec3D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add<f32> for Vec3D {
    type Output = Self;
    fn add(self, rhs: f32) -> Self::Output {
        Self::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

impl Add<Vec3D> for f32 {
    type Output = Vec3D;
    fn add(self, rhs: Vec3D) -> Self::Output {
        Vec3D::new(self + rhs.x, self + rhs.y, self + rhs.z)
    }
}

impl Sub for Vec3D {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<f32> for Vec3D {
    type Output = Self;
    fn sub(self, rhs: f32) -> Self::Output {
        Self::new(self.x - rhs, self.y - rhs, self.z - rhs)
    }
}

impl Mul<f32> for Vec3D {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vec3D> for f32 {
    type Output = Vec3D;
    fn mul(self, rhs: Vec3D) -> Self::Output {
        Vec3D::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl Div<f32> for Vec3D {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Neg for Vec3D {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl From<Quaternion> for Vec3D {
    fn from(value: Quaternion) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

impl From<[f32; 3]> for Vec3D {
    fn from(value: [f32; 3]) -> Self {
        let mut value = value.iter();
        Vec3D::new(
            *value.next().unwrap(),
            *value.next().unwrap(),
            *value.next().unwrap(),
        )
    }
}

impl From<Vec3D> for [f32; 3] {
    fn from(value: Vec3D) -> Self {
        [value.x, value.y, value.z]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        assert_eq!(
            Vec3D::new(1., 2., 3.),
            Vec3D {
                x: 1.,
                y: 2.,
                z: 3.
            }
        );
    }

    #[test]
    fn test_triple() {
        assert_eq!(
            Vec3D::triple(1.),
            Vec3D {
                x: 1.,
                y: 1.,
                z: 1.
            }
        );
    }

    #[test]
    fn test_length() {
        assert_eq!(Vec3D::new(1., 2., 3.).length(), 3.7416575);
    }

    #[test]
    fn test_normalise() {
        assert_eq!(
            Vec3D::new(1., 2., 3.).normalise(),
            Vec3D {
                x: 0.26726124,
                y: 0.5345225,
                z: 0.8017837
            }
        );
    }

    #[test]
    fn test_dot() {
        assert_eq!(Vec3D::new(1., 2., 3.).dot(Vec3D::new(4., 5., 6.)), 32.);
    }

    #[test]
    fn test_cross() {
        assert_eq!(
            Vec3D::new(1., 2., 3.).cross(Vec3D::new(4., 5., 6.)),
            Vec3D {
                x: -3.,
                y: 6.,
                z: -3.
            }
        );
    }

    #[test]
    fn test_reflection() {
        assert_eq!(
            Vec3D::new(1., 1., 0.).reflect(Vec3D::new(0., 1., 0.).normalise()),
            Vec3D::new(1., -1., 0.)
        );
        assert_eq!(
            Vec3D::new(1., 1., 0.).reflect(Vec3D::new(1., 1., 0.).normalise()),
            Vec3D::new(-1., -1., 0.)
        );
        assert_eq!(
            Vec3D::new(1., 1., 0.).reflect(Vec3D::new(0., 0., 1.).normalise()),
            Vec3D::new(1., 1., 0.)
        );
        assert_eq!(
            Vec3D::new(1., 1., 0.).reflect(Vec3D::new(1., 0., 0.).normalise()),
            Vec3D::new(-1., 1., 0.)
        );
        assert_eq!(
            Vec3D::new(1., 1., 0.).reflect(Vec3D::new(0., 1., 1.).normalise()),
            Vec3D::new(1., 0., -1.)
        );
        assert_eq!(
            Vec3D::new(1., 1., 0.).reflect(Vec3D::new(1., 1., 1.).normalise()),
            Vec3D::new(-1. / 3., -1. / 3., -4. / 3.)
        );
    }
}
