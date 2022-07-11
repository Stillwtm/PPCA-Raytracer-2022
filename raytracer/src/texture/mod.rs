pub mod solid_color;

use crate::utility::*;

pub trait Texture {
    fn value(u: f64, v: f64, p: &Point3) -> Color;
}
