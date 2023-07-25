use crate::vector::Vec3D;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: Vec3D,
    pub direction: Vec3D,
}

impl Ray {
    pub fn new(origin: Vec3D, direction: Vec3D) -> Self {
        Self { origin, direction }
    }
}
