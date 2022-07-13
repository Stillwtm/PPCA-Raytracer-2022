use crate::bvh::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::utility::*;

pub struct Translation<T: Hittable> {
    obj: T,
    offset: Vec3, // motion vector
}

impl<T: Hittable> Translation<T> {
    pub fn new(obj: T, offset: Vec3) -> Self {
        Self { obj, offset }
    }
}

impl<T: Hittable> Hittable for Translation<T> {
    fn bounding_box(&self, st_time: f64, ed_time: f64) -> Option<AABB> {
        self.obj
            .bounding_box(st_time, ed_time)
            .map(|obj_box| AABB::new(obj_box.minimum + self.offset, obj_box.maximum + self.offset))
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray::new(r.orig - self.offset, r.dir, r.tm);
        if let Some(mut rec) = self.obj.hit(&moved_r, t_min, t_max) {
            rec.p += self.offset;
            rec.set_face_normal(&moved_r, rec.normal);
            Some(rec)
        } else {
            None
        }
    }
}
