use crate::utility::*;
use super::Texture;

pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(color_value: Color) -> Self {
        Self { color_value }
    }

    pub fn new_rgb(r: f64, g: f64, b: f64) -> Self {
        Self { color_value: Color::new(r, g, b) }
    }
}


