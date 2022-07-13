pub mod aarect;
pub mod constant_medium;
pub mod cuboid;
pub mod hittable_list;
pub mod instance;
pub mod obj_model;
pub mod sphere;
pub mod triangle;

use crate::bvh::aabb::AABB;
use crate::material::{lambertian::Lambertian, Material};
use crate::utility::*;

pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    // pub mat_ptr: Box<dyn Material>,
    pub mat_ptr: &'a dyn Material,
    pub t: f64,
    pub u: f64, // texture uv
    pub v: f64,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(&r.dir, &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, st_time: f64, ed_time: f64) -> Option<AABB>;
    fn pdf_value(&self, orig: &Point3, v: &Vec3) -> f64 {
        0.0
    }
    fn random(&self, orig: Vec3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}
