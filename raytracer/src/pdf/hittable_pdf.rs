use super::PDF;
use crate::basic::onb::ONB;
use crate::{hittable::Hittable, utility::*};

pub struct HittablePDF<'a, T: Hittable> {
    obj: &'a T,
    orig: Point3,
}

impl<'a, T: Hittable> HittablePDF<'a, T> {
    pub fn new(obj: &'a T, orig: Point3) -> Self {
        Self { obj, orig }
    }
}

impl<'a, T: Hittable> PDF for HittablePDF<'a, T> {
    fn value(&self, dir: &Vec3) -> f64 {
        self.obj.pdf_value(&self.orig, dir)
    }

    fn generate(&self) -> Vec3 {
        self.obj.random(self.orig)
    }
}
