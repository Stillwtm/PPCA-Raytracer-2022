pub mod dielectric;
pub mod diffuse_light;
pub mod isotropic;
pub mod lambertian;
pub mod metal;

use crate::hittable::HitRecord;
use crate::utility::*;
use lambertian::Lambertian;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color) -> Option<Ray>;

    fn emitted(&self, u: f64, v: f64, p: Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
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
