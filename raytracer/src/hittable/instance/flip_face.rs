use crate::hittable::{HitRecord, Hittable};
use crate::utility::*;

pub struct FlipFace<T: Hittable> {
    obj: T,
}

impl<T: Hittable> FlipFace<T> {
    pub fn new(obj: T) -> Self {
        Self { obj }
    }
}

impl<T: Hittable> Hittable for FlipFace<T> {
    fn bounding_box(&self, st_time: f64, ed_time: f64) -> Option<crate::bvh::aabb::AABB> {
        self.obj.bounding_box(st_time, ed_time)
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(mut rec) = self.obj.hit(r, t_min, t_max) {
            rec.front_face = !rec.front_face;
            Some(rec)
        } else {
            None
        }
    }
}
