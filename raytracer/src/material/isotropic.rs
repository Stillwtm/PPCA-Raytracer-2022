use super::{Material, ScatterRecord};
use crate::hittable::HitRecord;
use crate::pdf::cosine_pdf::CosinePDF;
use crate::texture::{solid_color::SolidColor, Texture};
use crate::utility::*;

pub struct Isotropic<T: Texture> {
    albedo: T,
}

impl<T: Texture> Isotropic<T> {
    pub fn new(albedo: T) -> Self {
        Self { albedo }
    }
}

impl Isotropic<SolidColor> {
    pub fn new_from_color(color: Color) -> Self {
        Self {
            albedo: SolidColor::new(color),
        }
    }
}

impl<T: Texture> Material for Isotropic<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord::new_diff(
            CosinePDF::new(Vec3::rand_in_unit_sphere()),
            self.albedo.value(rec.u, rec.v, rec.p),
        ))
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        // TODO
        let cosine = Vec3::dot(&rec.normal, &scattered.dir.unit_vector());
        if cosine.is_sign_negative() {
            0.0
        } else {
            cosine / PI
        }
    }
}
