use super::Material;
use crate::hittable::HitRecord;
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color) -> Option<Ray> {
        *attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        Some(Ray::new(rec.p, Vec3::rand_in_unit_sphere(), r_in.tm))
    }
}
