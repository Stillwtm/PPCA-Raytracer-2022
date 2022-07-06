pub mod sphere;
pub mod hittable_list;

use super::ray::{Ray, Vec3, Point3};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub fornt_face: bool,
}

impl HitRecord {
    fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3){
        self.fornt_face = Vec3::dot(&r.dir, outward_normal) < 0.0;
        self.normal = if self.fornt_face { *outward_normal } else { -*outward_normal };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}