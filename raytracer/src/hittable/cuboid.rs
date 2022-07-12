use super::aarect::{Rect, XYRect, XZRect, YZRect};
use super::{HitRecord, Hittable};
use crate::bvh::aabb::AABB;
use crate::material::Material;
use crate::utility::*;

pub struct Cuboid<T: Material> {
    cb_min: Point3,
    cb_max: Point3,
    sides: [Rect<T>; 6],
}

impl<T: Material + Clone> Cuboid<T> {
    pub fn new(cb_min: Point3, cb_max: Point3, mat: T) -> Self {
        let mut sides = [
            Rect::XYRect(XYRect::new(
                cb_min.x,
                cb_max.x,
                cb_min.y,
                cb_max.y,
                cb_max.z,
                mat.clone(),
            )),
            Rect::XYRect(XYRect::new(
                cb_min.x,
                cb_max.x,
                cb_min.y,
                cb_max.y,
                cb_min.z,
                mat.clone(),
            )),
            Rect::XZRect(XZRect::new(
                cb_min.x,
                cb_max.x,
                cb_min.z,
                cb_max.z,
                cb_max.y,
                mat.clone(),
            )),
            Rect::XZRect(XZRect::new(
                cb_min.x,
                cb_max.x,
                cb_min.z,
                cb_max.z,
                cb_min.y,
                mat.clone(),
            )),
            Rect::YZRect(YZRect::new(
                cb_min.y,
                cb_max.y,
                cb_min.z,
                cb_max.z,
                cb_max.x,
                mat.clone(),
            )),
            Rect::YZRect(YZRect::new(
                cb_min.y,
                cb_max.y,
                cb_min.z,
                cb_max.z,
                cb_min.x,
                mat.clone(),
            )),
        ];
        Self {
            cb_min,
            cb_max,
            sides,
        }
    }
}

impl<T: Material + Sync + Send> Hittable for Cuboid<T> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut opt_rec = None;
        let mut closest_so_far = t_max;

        for side in self.sides.iter() {
            if let Some(temp_rec) = side.hit(&r, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                opt_rec = Some(temp_rec);
            }
        }

        opt_rec
    }

    fn bounding_box(&self, st_time: f64, ed_time: f64) -> Option<AABB> {
        Some(AABB::new(self.cb_min, self.cb_max))
    }
}
