use super::Material;
use crate::hittable::HitRecord;
use crate::texture::{solid_color::SolidColor, Texture};
use crate::utility::*;

#[derive(Copy, Clone)]
pub struct Lambertian<T: Texture> {
    pub albedo: T,
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color) -> Option<Ray> {
        let mut scatter_direction = rec.normal + Vec3::rand_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        Some(Ray::new(rec.p, scatter_direction, r_in.tm))
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
