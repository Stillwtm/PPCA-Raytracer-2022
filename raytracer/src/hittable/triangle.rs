use rand::Rng;

use super::{HitRecord, Hittable};
use crate::bvh::aabb::AABB;
use crate::material::Material;
use crate::utility::*;

pub struct Triangle<T: Material> {
    ver: [Point3; 3],
    normal: Vec3,
    mat: T,
    v: Vec3,
    w: Vec3,
    area: f64,
}

impl<T: Material> Triangle<T> {
    pub fn new(ver: [Point3; 3], mat: T) -> Self {
        let normal = (Vec3::cross(&ver[0], &ver[1])
            + Vec3::cross(&ver[1], &ver[2])
            + Vec3::cross(&ver[2], &ver[0]))
        .unit_vector();

        let mut v = Vec3::cross(&normal, &(ver[1] - ver[0]));
        v /= Vec3::dot(&(ver[2] - ver[0]), &v);
        let mut w = Vec3::cross(&normal, &(ver[2] - ver[0]));
        w /= Vec3::dot(&(ver[1] - ver[0]), &w);

        let a = ((ver[0].x - ver[1].x).powi(2)
            + (ver[0].y - ver[1].y).powi(2)
            + (ver[0].z - ver[1].z).powi(2))
        .sqrt();
        let b = ((ver[1].x - ver[2].x).powi(2)
            + (ver[1].y - ver[2].y).powi(2)
            + (ver[1].z - ver[2].z).powi(2))
        .sqrt();
        let c = ((ver[2].x - ver[0].x).powi(2)
            + (ver[2].y - ver[0].y).powi(2)
            + (ver[2].z - ver[0].z).powi(2))
        .sqrt();
        let p = (a + b + c) / 2.0;
        let area = (p * (p - a) * (p - b) * (p - c)).sqrt();

        Self {
            ver,
            normal,
            mat,
            v,
            w,
            area,
        }
    }
}

impl<T: Material + Sync + Send> Hittable for Triangle<T> {
    fn bounding_box(&self, st_time: f64, ed_time: f64) -> Option<AABB> {
        let eps = 0.0001;
        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY);

        for v in self.ver.iter() {
            for i in 0..3 {
                min[i] = min[i].min(v[i]);
                max[i] = max[i].max(v[i])
            }
        }
        // eps是为了防止和坐标平面平行的三角形
        Some(AABB::new(min - eps, max + eps))
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // let origin = r.orig;
        let t = Vec3::dot(&r.dir, &self.normal);
        if t == 0.0 {
            // 光线和三角形平行
            return None;
        }
        let t = Vec3::dot(&(self.ver[0] - r.orig), &self.normal) / t;
        if t < t_min || t > t_max {
            return None;
        }

        let p = r.orig + t * r.dir;
        let ap = p - self.ver[0];
        let gamma = Vec3::dot(&ap, &self.v);
        if gamma >= 0.0 {
            let beta = Vec3::dot(&ap, &self.w);
            if beta >= 0.0 && gamma + beta <= 1.0 {
                let mut rec = HitRecord {
                    p,
                    t,
                    u: beta,
                    v: gamma,
                    mat_ptr: &self.mat,
                    normal: Vec3::default(),
                    front_face: bool::default(),
                };
                rec.set_face_normal(r, self.normal);

                return Some(rec);
            }
        }
        None
    }

    fn pdf_value(&self, orig: &Point3, v: &Vec3) -> f64 {
        if let Some(rec) = self.hit(&Ray::new(*orig, *v, 0.0), 0.001, INFINITY) {
            let distance_squared = rec.t.powi(2) * v.length_squared();
            let cosine = Vec3::dot(&v, &rec.normal).abs() / v.length();

            distance_squared / (cosine * self.area)
        } else {
            0.0
        }
    }

    fn random(&self, orig: Vec3) -> Vec3 {
        let mut rng = rand::thread_rng();
        let mut k1 = rng.gen::<f64>();
        let mut k2 = rng.gen::<f64>();
        if k1 + k2 > 1. {
            k1 = 1. - k1;
            k2 = 1. - k2;
        }

        k1 * (self.ver[1] - self.ver[0]) + k1 * (self.ver[2] - self.ver[0]) - orig
    }
}
