#![feature(ptr_metadata)]
use std::{f32::consts::PI, fs::File, io::Write, process::Command};

use log::trace;
use simple_logger::SimpleLogger;

mod camera;
mod colour;
mod constants;
mod image;
mod lighting;
mod material;
mod math;
mod physics;
mod quaternion;
mod ray;
mod sphere;
mod vector;

use colour::Colour;

use crate::{
    camera::Camera, image::Image, lighting::PointLight, material::Material, physics::Object,
    quaternion::Quaternion, sphere::Sphere, vector::Vec3D,
};

const IMAGE_SIZE: (usize, usize) = (1024, 768);
const IMAGE_NAME: &str = "out.ppm";
const PIXEL_TO_WORLD: f32 = 0.008;

fn main() {
    SimpleLogger::new().with_colors(true).init().unwrap();

    trace!("making an image");
    let (width, height) = IMAGE_SIZE;
    let mut image = Image::new(width, height, Colour::new(0.2, 0.7, 0.8));

    trace!("creating objects");
    let camera = Camera::new(
        Vec3D::default(),
        Quaternion::from_axis_angle(Vec3D::X, -PI / 2.),
        5.,
    );
    let ivory = Material {
        albedo: 0.3..0.6,
        diffuse_colour: Colour::new(0.4, 0.4, 0.3),
        specular_exponent: 50.,
    };
    let red_rubber = Material {
        albedo: 0.1..0.9,
        diffuse_colour: Colour::new(0.3, 0.1, 0.1),
        specular_exponent: 10.,
    };
    let objects = [
        Sphere::new(Vec3D::new(-3., 0., -16.), 2., ivory.clone()),
        Sphere::new(Vec3D::new(-1.0, -1.5, -12.), 2., red_rubber.clone()),
        Sphere::new(Vec3D::new(1.5, -0.5, -18.), 3., red_rubber.clone()),
        Sphere::new(Vec3D::new(7., 5., -18.), 4., ivory.clone()),
    ];
    let lights = [
        PointLight::new(Vec3D::new(-20., 20., 20.), 1.5),
        PointLight::new(Vec3D::new(30., -50., -25.), 1.8),
        PointLight::new(Vec3D::new(30., -20., 30.), 1.7),
    ];

    trace!("rendering");
    for (x, y, pixel) in &mut image {
        let ray = camera.ray_from_position(
            (x as i32 - width as i32 / 2) as f32 * PIXEL_TO_WORLD,
            (y as i32 - height as i32 / 2) as f32 * PIXEL_TO_WORLD,
        );
        let mut intersections = objects.iter().fold(Vec::new(), |previous, object| {
            let mut previous = previous;
            previous.extend(object.intersections(&ray));
            previous
        });
        intersections.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        if !intersections.is_empty() {
            *pixel = {
                let first_hit = intersections.first().unwrap();
                let material = first_hit.object.material();
                let (diffuse_light_intensity, specular_light_intensity) =
                    lights.iter().fold((0., 0.), |previous, light| {
                        let light_direction = (light.position - first_hit.position).normalise();
                        (
                            previous.0
                                + 0_f32.max(light_direction.dot(first_hit.normal))
                                    * light.intensity,
                            previous.1
                                + 0_f32
                                    .max(
                                        light_direction
                                            .reflect(first_hit.normal)
                                            .dot(first_hit.ray.direction),
                                    )
                                    .powf(material.specular_exponent)
                                    * light.intensity,
                        )
                    });
                (material.diffuse_colour * diffuse_light_intensity * material.albedo.end)
                    + (Colour::from_hex(0xffffff)
                        * specular_light_intensity
                        * material.albedo.start)
            }
        }
    }

    trace!("writing to file");
    write_image(&mut image);

    trace!("opening");
    Command::new("open").arg(IMAGE_NAME).spawn().unwrap();
}

fn write_image(image: &mut Image) {
    let mut image_file = File::create(IMAGE_NAME).unwrap();
    writeln!(image_file, "P6 {} {} 255", image.width, image.height).unwrap();
    image_file
        .write_all(
            image
                .into_iter()
                .flat_map(|(.., pixel)| {
                    let max = pixel.as_rgb().into_iter().reduce(f32::max).unwrap();
                    if max > 1. {
                        *pixel = *pixel * (1. / max);
                    }
                    pixel
                        .as_rgb()
                        .into_iter()
                        .map(|colour_value| (255. * 0_f32.max(1_f32.min(colour_value))) as u8)
                })
                .collect::<Vec<u8>>()
                .as_slice(),
        )
        .unwrap();
}
