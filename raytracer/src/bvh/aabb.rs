use crate::utility::*;
use std::mem;

#[derive(Default, Clone, Copy)]
pub struct AABB {
    pub minimum: Point3,
    pub maximum: Point3,
}

impl AABB {
    pub fn new(minimum: Point3, maximum: Point3) -> Self {
        Self { minimum, maximum }
    }

    pub fn hit(&self, r: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for i in 0..3 {
            let inv_d = 1.0 / r.dir[i];
            let mut t0 = (self.minimum[i] - r.orig[i]) * inv_d;
            let mut t1 = (self.maximum[i] - r.orig[i]) * inv_d;
            if (inv_d < 0.0) {
                mem::swap(&mut t0, &mut t1);
            }
            t_min = t_min.max(t0);
            t_max = t_max.min(t1);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
        AABB::new(
            Point3::new(
                box0.minimum.x.min(box1.minimum.x),
                box0.minimum.y.min(box1.minimum.y),
                box0.minimum.z.min(box1.minimum.z),
            ),
            Point3::new(
                box0.maximum.x.max(box1.maximum.x),
                box0.maximum.y.max(box1.maximum.y),
                box0.maximum.z.max(box1.maximum.z),
            ),
        )
    }
}