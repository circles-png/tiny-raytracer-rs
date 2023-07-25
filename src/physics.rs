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
    pub object: Box<dyn Physics>,
}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
            && self.position == other.position
            && self.normal == other.normal
            && self.object.as_ref() as *const dyn Physics
                == other.object.as_ref() as *const dyn Physics
    }
}
