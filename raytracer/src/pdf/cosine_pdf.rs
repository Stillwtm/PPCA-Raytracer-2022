use super::PDF;
use crate::basic::onb::ONB;
use crate::utility::*;

#[derive(Copy, Clone)]
pub struct CosinePDF {
    uvw: ONB,
}

impl CosinePDF {
    pub fn new(w: Vec3) -> Self {
        Self {
            uvw: ONB::build_from_w(w),
        }
    }
}

impl PDF for CosinePDF {
    fn value(&self, dir: &Vec3) -> f64 {
        let cosine = Vec3::dot(&dir.unit_vector(), &self.uvw.w);
        if cosine.is_sign_negative() {
            0.0
        } else {
            cosine / PI
        }
    }

    fn generate(&self) -> Vec3 {
        self.uvw.local(Vec3::rand_cos_dir())
    }
}
