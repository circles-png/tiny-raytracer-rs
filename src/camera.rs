use crate::{vector::Vec3D, quaternion::Quaternion, ray::Ray};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    pub position: Vec3D,
    pub rotation: Quaternion,
    pub screen_distance: f32,
}

impl Camera {
    pub fn new(position: Vec3D, rotation: Quaternion, screen_distance: f32) -> Self {
        Self {
            position,
            rotation,
            screen_distance
        }
    }

    pub fn ray_from_position(&self, x: f32, y: f32) -> Ray {
        Ray::new(self.position, self.rotation * Vec3D::new(x, self.screen_distance, y))
    }
}
