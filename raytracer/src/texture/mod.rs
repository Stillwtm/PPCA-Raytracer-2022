pub mod checker_texture;
pub mod image_texture;
pub mod noise_texture;
pub mod obj_texture;
pub mod solid_color;

use crate::utility::*;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}
