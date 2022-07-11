use super::{HitRecord, Hittable};
use crate::utility::*;
use std::sync::Arc;
use crate::bvh::aabb::AABB;

#[derive(Default, Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

#[allow(unused)]
impl HittableList {
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut opt_rec = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(temp_rec) = object.hit(&r, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                opt_rec = Some(temp_rec);
            }
        }

        opt_rec
    }

    fn bounding_box(&self, st_time: f64, ed_time: f64) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }

        let mut output_box = AABB::default();
        let mut first_box = true;
        for object in self.objects.iter() {
            match object.bounding_box(st_time, ed_time) {
                Some(tmp_box) => {
                    output_box = if first_box {
                        first_box = false;
                        tmp_box
                    } else {
                        AABB::surrounding_box(&output_box, &tmp_box)
                    }
                },
                None => return None,
            }
        }

        Some(output_box)
    }
}
