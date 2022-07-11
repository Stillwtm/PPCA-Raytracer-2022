use super::aabb::AABB;
use crate::hittable::{hittable_list::HittableList, HitRecord, Hittable};
use crate::utility::*;

use rand::Rng;
use std::borrow::Borrow;
use std::ops::Deref;
use std::sync::Arc;

pub struct BvhNode {
    left: Option<Arc<dyn Hittable>>,
    right: Option<Arc<dyn Hittable>>,
    node_box: AABB,
}

impl BvhNode {
    pub fn new_from_list(list: &mut HittableList, st_time: f64, ed_time: f64) -> Self {
        BvhNode::new_from_vec(&mut list.objects, st_time, ed_time)
    }

    pub fn new_from_vec(objects: &mut [Arc<dyn Hittable>], st_time: f64, ed_time: f64) -> Self {
        // println!("st:{}  ed:{}   len:{}", start, end, objects.len());
        let axis = rand::thread_rng().gen_range(0..3);
        let comparator = |a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>| {
            f64::partial_cmp(
                &(a.bounding_box(st_time, ed_time).unwrap().minimum[axis]),
                &(b.bounding_box(st_time, ed_time).unwrap().maximum[axis]),
            )
            .unwrap()
        };

        let object_span = objects.len();
        if object_span == 1 {
            Self {
                left: Some(objects[0].clone()),
                right: None,
                node_box: objects[0].bounding_box(st_time, ed_time).unwrap(),
            }
        } else if object_span == 2 {
            let left = objects[0].clone();
            let right = objects[1].clone();
            let node_box = AABB::surrounding_box(
                &left.bounding_box(st_time, ed_time).unwrap(),
                &right.bounding_box(st_time, ed_time).unwrap(),
            );
            match comparator(&left, &right) {
                std::cmp::Ordering::Less => Self {
                    left: Some(left),
                    right: Some(right),
                    node_box,
                },
                _ => Self {
                    left: Some(right),
                    right: Some(left),
                    node_box,
                },
            }
        } else {
            objects.sort_unstable_by(comparator);
            let mid = object_span / 2;
            let left = Arc::new(BvhNode::new_from_vec(
                &mut objects[0..mid],
                st_time,
                ed_time,
            ));
            let right = Arc::new(BvhNode::new_from_vec(
                &mut objects[mid..object_span],
                st_time,
                ed_time,
            ));
            Self {
                node_box: AABB::surrounding_box(
                    &left.bounding_box(st_time, ed_time).unwrap(),
                    &right.bounding_box(st_time, ed_time).unwrap(),
                ),
                left: Some(left),
                right: Some(right),
            }
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.node_box.hit(&r, t_min, t_max) {
            return None;
        }

        let mut hit_record = None;
        let mut closest_so_far = t_max;
        if self.left.is_some() {
            if let Some(rec) = self.left.as_ref().unwrap().hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                hit_record = Some(rec);
            }
        }
        if self.right.is_some() {
            if let Some(rec) = self.right.as_ref().unwrap().hit(r, t_min, closest_so_far) {
                hit_record = Some(rec);
            }
        }

        hit_record
    }

    fn bounding_box(&self, st_time: f64, ed_time: f64) -> Option<AABB> {
        Some(self.node_box)
    }
}
