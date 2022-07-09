use crate::ray::{Ray, Vec3, Point3};
use super::*;

pub struct Sphere<T: Material> {
    center: Point3,
    radius: f64,
    mat: T,
}

impl<T: Material> Sphere<T> {
    pub fn new(center: Point3, radius: f64, mat: T) -> Self {
        Sphere {
            center,
            radius,
            mat,
        }
    }
}

impl<T: Material + 'static + Clone> Hittable for Sphere<T> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig - self.center;
        let a = r.dir.length_squared();
        let half_b = Vec3::dot(&oc, &r.dir);
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        // Find the neareat root that lies in the acceptable range
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }
        
        let mut rec = HitRecord {
            t: root,
            p: r.at(root),
            mat_ptr: &self.mat,
            normal: Vec3::default(),
            fornt_face: bool::default(),
        };
        
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(&r, &outward_normal);

        Some(rec)
    }
}