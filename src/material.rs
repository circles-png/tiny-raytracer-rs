use std::ops::Range;

use crate::colour::Colour;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Material {
    pub diffuse_colour: Colour,
    pub specular_exponent: f32,
    pub albedo: Range<f32>
}
