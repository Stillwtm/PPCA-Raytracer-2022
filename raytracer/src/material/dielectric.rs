use super::{Material, ScatterRecord};
use crate::hittable::HitRecord;
use crate::utility::*;

use rand::Rng;

#[derive(Copy, Clone)]
pub struct Dielectric {
    ir: f64, // Index of Refraction
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let refraction_ratio = if (rec.front_face) {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.dir.unit_vector();
        let cos_theta = Vec3::dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = (refraction_ratio * sin_theta > 1.0);
        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > rand::thread_rng().gen()
        {
            Vec3::reflect(&unit_direction, &rec.normal)
        } else {
            Vec3::refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        Some(ScatterRecord::new_spec(
            Ray::new(rec.p, direction, r_in.tm),
            Color::new(1., 1., 1.),
        ))
    }
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}
