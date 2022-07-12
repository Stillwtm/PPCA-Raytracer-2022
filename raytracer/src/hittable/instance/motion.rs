use crate::bvh::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::utility::*;

pub struct Motion<T: Hittable> {
    obj: T,
    mov: Vec3,    // motion vector
    st_time: f64, // start moving time
    ed_time: f64, // end moving time
}

impl<T: Hittable> Motion<T> {
    pub fn new(obj: T, mov: Vec3, st_time: f64, ed_time: f64) -> Self {
        Self {
            obj,
            mov,
            st_time,
            ed_time,
        }
    }
}

impl<T: Hittable> Hittable for Motion<T> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let offset = (r.tm - self.st_time) / (self.ed_time - self.st_time) * self.mov;
        // 根据相对运动，移动光线来替代移动物体
        let tmp_ray = Ray::new(r.orig - offset, r.dir, r.tm);
        if let Some(mut rec) = self.obj.hit(&tmp_ray, t_min, t_max) {
            rec.p += offset; // 将光线的移动还原
            rec.set_face_normal(&tmp_ray, rec.normal);
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, st_time: f64, ed_time: f64) -> Option<AABB> {
        let box0 = self.obj.bounding_box(st_time, ed_time).unwrap();
        let box1 = AABB::new(box0.minimum + self.mov, box0.maximum + self.mov);
        Some(AABB::surrounding_box(&box0, &box1))
    }
}
