use super::PDF;
use crate::basic::onb::ONB;
use crate::utility::*;

use rand::Rng;

pub struct MixturePDF<U: PDF, V: PDF> {
    p0: U,
    p1: V,
}

impl<U: PDF, V: PDF> MixturePDF<U, V> {
    pub fn new(p0: U, p1: V) -> Self {
        Self { p0, p1 }
    }
}

impl<U: PDF, V: PDF> PDF for MixturePDF<U, V> {
    fn value(&self, dir: &Vec3) -> f64 {
        0.5 * self.p0.value(dir) + 0.5 * self.p1.value(dir)
    }

    fn generate(&self) -> Vec3 {
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < 0.5 {
            self.p0.generate()
        } else {
            self.p1.generate()
        }
    }
}
