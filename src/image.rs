use std::fmt::Display;

use crate::colour::Colour;

#[derive(Debug, Clone)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Colour>,
}

impl Image {
    pub fn new(width: usize, height: usize, fill: Colour) -> Self {
        Self {
            width,
            height,
            pixels: vec![fill; width * height],
        }
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{} image", self.width, self.height)
    }
}

impl<'a> IntoIterator for &'a mut Image {
    type Item = (usize, usize, &'a mut Colour);
    type IntoIter = PixelIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        PixelIterator {
            image: self,
            index: 0,
        }
    }
}

pub struct PixelIterator<'a> {
    image: &'a mut Image,
    index: usize,
}

impl<'a> Iterator for PixelIterator<'a> {
    type Item = (usize, usize, &'a mut Colour);
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.image.pixels.len() {
            None
        } else {
            let index = self.index;
            self.index += 1;

            let pointer = self.image.pixels.as_mut_ptr();
            unsafe {
                Some((
                    (index) % self.image.width,
                    (index) / self.image.width,
                    &mut *pointer.add(index),
                ))
            }
        }
    }
}
