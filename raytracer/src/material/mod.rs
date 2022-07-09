pub mod dielectric;
pub mod lambertian;
pub mod metal;

use crate::hittable::HitRecord;
use crate::utility::*;
use lambertian::Lambertian;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color) -> Option<Ray>;
}

// // Make default material Lambertian
// impl Default for Box<dyn Material> {
//     fn default() -> Self {
//         Box::new(Lambertian::new(Color::new(0.0, 0.0, 0.0)))
//     }
// }

// impl Default for &dyn Material {
//     fn default() -> Self {
//         Lambertian::new(Color::new(0.0, 0.0, 0.0))
//     }
// }
