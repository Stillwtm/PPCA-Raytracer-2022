use super::{HitRecord, Hittable};
use crate::utility::*;
use std::sync::Arc;

#[derive(Default, Clone)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
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
}
