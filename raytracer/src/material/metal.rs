use super::Material;
use crate::hittable::HitRecord;
use crate::utility::*;

#[derive(Copy, Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color) -> Option<Ray> {
        let reflected = Vec3::reflect(&r_in.dir.unit_vector(), &rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::rand_in_unit_sphere());
        *attenuation = self.albedo;
        if Vec3::dot(&scattered.dir, &rec.normal) > 0.0 {
            Some(scattered)
        } else {
            None
        }
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
