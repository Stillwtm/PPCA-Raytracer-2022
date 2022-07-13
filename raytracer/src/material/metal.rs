use super::{Material, ScatterRecord};
use crate::hittable::HitRecord;
use crate::utility::*;

#[derive(Copy, Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = Vec3::reflect(&r_in.dir.unit_vector(), &rec.normal);

        Some(ScatterRecord::new_spec(
            Ray::new(
                rec.p,
                reflected + self.fuzz * Vec3::rand_in_unit_sphere(),
                r_in.tm,
            ),
            self.albedo,
        ))
        // let scattered = Ray::new(
        //     rec.p,
        //     reflected + self.fuzz * Vec3::rand_in_unit_sphere(),
        //     r_in.tm,
        // );
        // if Vec3::dot(&scattered.dir, &rec.normal) > 0.0 {
        //     Some(ScatterRecord::new_spec(scattered, self.albedo))
        // } else {
        //     None
        // }
    }
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}
