use crate::vector::Vec3D;

pub struct PointLight {
    pub position: Vec3D,
    pub intensity: f32,
}

impl PointLight {
    pub fn new(position: Vec3D, intensity: f32) -> Self {
        Self {
            position,
            intensity,
        }
    }
}
