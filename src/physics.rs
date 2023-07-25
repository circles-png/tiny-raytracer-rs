use std::fmt::Debug;

use crate::{colour::Colour, ray::Ray, vector::Vec3D};

pub trait Object: Debug {
    fn intersections(&self, ray: &Ray) -> Vec<Intersection>;
    fn extent(&self) -> f32;
    fn centre(&self) -> Vec3D;
    fn diffuse_colour(&self) -> Colour;
}

#[derive(Debug)]
pub struct Intersection {
    pub position: Vec3D,
    pub distance: f32,
    pub normal: Vec3D,
    pub object: Box<dyn Object>,
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
            && (self.object.as_ref() as *const dyn Object).to_raw_parts().0
                == (other.object.as_ref() as *const dyn Object)
                    .to_raw_parts()
                    .0
    }
}
