pub mod cosine_pdf;
pub mod hittable_pdf;
pub mod mixture_pdf;

use crate::utility::*;

pub trait PDF {
    fn value(&self, dir: &Vec3) -> f64;

    fn generate(&self) -> Vec3;
}
