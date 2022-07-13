use super::{Material, ScatterRecord};
use crate::basic::onb::ONB;
use crate::hittable::HitRecord;
use crate::pdf::cosine_pdf::CosinePDF;
use crate::texture::{solid_color::SolidColor, Texture};
use crate::utility::*;

#[derive(Copy, Clone)]
pub struct Lambertian<T: Texture> {
    albedo: T,
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord::new_diff(
            CosinePDF::new(rec.normal),
            self.albedo.value(rec.u, rec.v, rec.p),
        ))
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cosine = Vec3::dot(&rec.normal, &scattered.dir.unit_vector());
        if cosine.is_sign_negative() {
            0.0
        } else {
            cosine / PI
        }
    }
}

impl<T: Texture> Lambertian<T> {
    pub fn new(albedo: T) -> Self {
        Self { albedo }
    }
}

impl Lambertian<SolidColor> {
    pub fn new_form_color(albedo: Color) -> Self {
        Self {
            albedo: SolidColor::new(albedo),
        }
    }
}
