use std::{fs::File, io::Write, process::Command};

use log::trace;
use simple_logger::SimpleLogger;

mod camera;
mod colour;
mod image;
mod math;
mod physics;
mod quaternion;
mod ray;
mod sphere;
mod vector;

use colour::Colour;

use crate::{
    camera::Camera, image::Image, math::map_range, physics::Physics, quaternion::Quaternion,
    sphere::Sphere, vector::Vec3D,
};

const IMAGE_SIZE: (usize, usize) = (256, 256);
const IMAGE_NAME: &str = "out.ppm";
const PIXEL_TO_WORLD: f32 = 0.008;

fn main() {
    SimpleLogger::new().with_colors(true).init().unwrap();

    trace!("making an image");
    let (width, height) = IMAGE_SIZE;
    let mut image = Image::new(width, height, Colour::from_hex(0x000000));

    trace!("creating objects");
    let camera = Camera::new(
        Vec3D::Y * -5.,
        Quaternion::from_axis_angle(Vec3D::Y, 0.),
        1.,
    );
    let objects = [Sphere::default(), Sphere::new(Vec3D::X, 1.), Sphere::new(Vec3D::X * -2., 0.5)];

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
            *pixel = Colour::gray({
                let closest = intersections.first().unwrap();
                map_range(
                    closest.distance,
                    ((camera.position - closest.object.centre()).length() - closest.object.extent() / 2.)
                        ..((camera.position - closest.object.centre()).length() + closest.object.extent() / 2.),
                    (1.)..0.,
                )
            })
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
    for (.., pixel) in &mut *image {
        for colour_value in pixel.as_rgb() {
            image_file
                .write_all(&[(255. * 0_f32.max(1_f32.min(colour_value))) as u8])
                .unwrap();
        }
    }
}
