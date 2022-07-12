use super::Material;
use crate::texture::solid_color::SolidColor;
use crate::texture::Texture;
use crate::utility::*;

#[derive(Clone)]
pub struct DiffuseLight<T: Texture> {
    emit: T,
}

impl<T: Texture> DiffuseLight<T> {
    pub fn new(emit: T) -> Self {
        Self { emit }
    }
}

impl DiffuseLight<SolidColor> {
    pub fn new_form_color(color: Color) -> Self {
        Self {
            emit: SolidColor::new(color),
        }
    }
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &crate::hittable::HitRecord,
        attenuation: &mut Color,
    ) -> Option<Ray> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: Point3) -> Color {
        self.emit.value(u, v, p)
    }
}
