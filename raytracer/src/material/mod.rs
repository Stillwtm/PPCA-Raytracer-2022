pub mod dielectric;
pub mod diffuse_light;
pub mod isotropic;
pub mod lambertian;
pub mod metal;

use crate::hittable::HitRecord;
use crate::pdf::cosine_pdf::CosinePDF;
use crate::utility::*;
use lambertian::Lambertian;

pub enum DiffOrSpec {
    Specular(Ray),
    Diffuse(CosinePDF),
}

pub struct ScatterRecord {
    pub ray_type: DiffOrSpec,
    pub attenuation: Color,
}

impl ScatterRecord {
    pub fn new_diff(pdf: CosinePDF, attenuation: Color) -> Self {
        Self {
            ray_type: DiffOrSpec::Diffuse(pdf),
            attenuation,
        }
    }

    pub fn new_spec(ray: Ray, attenuation: Color) -> Self {
        Self {
            ray_type: DiffOrSpec::Specular(ray),
            attenuation,
        }
    }
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        0.0
    }

    fn emitted(&self, r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}
