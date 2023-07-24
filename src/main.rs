use std::{fs::File, io::Write, process::Command};

use log::trace;
use simple_logger::SimpleLogger;

mod colour;
mod image;

use colour::Colour;

use crate::image::Image;

const IMAGE_SIZE: (usize, usize) = (256, 256);
const IMAGE_NAME: &str = "out.ppm";

fn main() {
    SimpleLogger::new().with_colors(true).init().unwrap();

    trace!("making an image");

    let (width, height) = IMAGE_SIZE;
    let mut image = Image::new(width, height);
    image.pixels = vec![Colour::default(); width * height];

    trace!("filling with pixels");
    for y in 0..height {
        for x in 0..width {
            image.pixels[x + y * width] = Colour::new(
                y as f32 / (height as f32 + 1.),
                x as f32 / (width as f32 + 1.),
                0.,
            );
        }
    }

    trace!("writing to file");
    write_image(&image);

    Command::new("open")
        .arg(IMAGE_NAME)
        .spawn()
        .unwrap();
}

fn write_image(image: &Image) {
    let mut image_file = File::create(IMAGE_NAME).unwrap();
    writeln!(image_file, "P6 {} {} 255", image.width, image.height).unwrap();
    for pixel in &image.pixels {
        for colour_value in pixel.as_rgb() {
            image_file
                .write_all(&[(255. * 0_f32.max(1_f32.min(colour_value))) as u8])
                .unwrap();
        }
    }
}
