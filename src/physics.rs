use std::fmt::Debug;

use crate::{ray::Ray, vector::Vec3D};

pub trait Physics: Debug {
    fn intersections(&self, ray: &Ray) -> Vec<Intersection>;
    fn extent(&self) -> f32;
    fn centre(&self) -> Vec3D;
}

#[derive(Debug)]
pub struct Intersection {
    pub position: Vec3D,
    pub distance: f32,
    pub normal: Vec3D,
    pub object: Box<dyn Physics>
}
