use std::mem::swap;

use crate::{
    colour::Colour,
    physics::{Intersection, Object},
    ray::Ray,
    vector::Vec3D,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    pub centre: Vec3D,
    pub radius: f32,
    pub diffuse_colour: Colour,
}

impl Sphere {
    pub fn new(centre: Vec3D, radius: f32, diffuse_colour: Colour) -> Self {
        Self {
            centre,
            radius,
            diffuse_colour,
        }
    }

    pub fn unit(diffuse_colour: Colour) -> Self {
        Self {
            diffuse_colour,
            ..Default::default()
        }
    }
}

impl Object for Sphere {
    fn intersections(&self, ray: &Ray) -> Vec<Intersection> {
        let oc = ray.origin - self.centre;
        let a = ray.direction.dot(ray.direction);
        let b = 2. * oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4. * a * c;
        if discriminant < 0. {
            return Vec::new();
        }
        let mut t1 = (-b - discriminant.sqrt()) / (2. * a);
        let mut t2 = (-b + discriminant.sqrt()) / (2. * a);
        if t1 > t2 {
            swap(&mut t1, &mut t2);
        }
        let mut intersections = Vec::new();
        if t1 > 0. {
            intersections.push(ray.origin + ray.direction * t1);
        }
        if t2 > 0. {
            intersections.push(ray.origin + ray.direction * t2);
        }
        intersections.dedup();
        intersections
            .iter()
            .map(|intersection| Intersection {
                distance: (*intersection - ray.origin).length(),
                position: *intersection,
                normal: (*intersection - self.centre).normalise(),
                object: Box::new(*self),
            })
            .collect::<Vec<Intersection>>()
    }

    fn extent(&self) -> f32 {
        self.radius * 2.
    }

    fn centre(&self) -> Vec3D {
        self.centre
    }

    fn diffuse_colour(&self) -> Colour {
        self.diffuse_colour
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            centre: Vec3D::default(),
            radius: 1.,
            diffuse_colour: Colour::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sphere_intersections() {
        let sphere = Sphere::default();
        let ray = Ray::new(Vec3D::new(0., 0., 5.), Vec3D::new(0., 0., -1.));
        let intersections = sphere.intersections(&ray);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].position, Vec3D::new(0., 0., 1.));
        assert_eq!(intersections[1].position, Vec3D::new(0., 0., -1.));
        assert_eq!(intersections[0].normal, Vec3D::new(0., 0., 1.));
        assert_eq!(intersections[1].normal, Vec3D::new(0., 0., -1.));
        assert_eq!(intersections[0].distance, 4.);
        assert_eq!(intersections[1].distance, 6.);
    }

    #[test]
    fn test_sphere_intersections_tangent() {
        let sphere = Sphere::default();
        let ray = Ray::new(Vec3D::new(0., 1., 1.), Vec3D::new(0., -1., 0.));
        let intersections = sphere.intersections(&ray);
        assert_eq!(intersections.len(), 1);
        assert_eq!(intersections[0].position, Vec3D::new(0., 0., 1.));
        assert_eq!(intersections[0].normal, Vec3D::new(0., 0., 1.));
        assert_eq!(intersections[0].distance, 1.);
    }

    #[test]
    fn test_sphere_intersections_miss() {
        let sphere = Sphere::default();
        let ray = Ray::new(Vec3D::new(0., 0., 2.), Vec3D::new(0., 1., 0.));
        let intersections = sphere.intersections(&ray);
        assert!(intersections.is_empty());
    }

    #[test]
    fn test_sphere_intersections_inside() {
        let sphere = Sphere::default();
        let ray = Ray::new(Vec3D::new(0., 0., 0.), Vec3D::new(0., 0., -1.));
        let intersections = sphere.intersections(&ray);
        assert_eq!(intersections.len(), 1);
        assert_eq!(intersections[0].position, Vec3D::new(0., 0., -1.));
        assert_eq!(intersections[0].normal, Vec3D::new(0., 0., -1.));
        assert_eq!(intersections[0].distance, 1.);
    }
}
