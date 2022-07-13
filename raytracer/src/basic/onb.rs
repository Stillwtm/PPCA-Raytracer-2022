use crate::utility::*;

pub struct ONB {
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl ONB {
    pub fn build_from_w(n: Vec3) -> Self {
        let w = n.unit_vector();
        let a = if w.x.abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let v = Vec3::cross(&w, &a).unit_vector();
        let u = Vec3::cross(&w, &v);
        Self { u, v, w }
    }

    pub fn local(&self, a: Vec3) -> Vec3 {
        a.x * self.u + a.y * self.v + a.z * self.w
    }
}
