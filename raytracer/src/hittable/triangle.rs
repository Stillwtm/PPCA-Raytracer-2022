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

        Self {
            ver,
            normal,
            mat,
            v,
            w,
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
        let t = Vec3::dot(&r.dir, &&self.normal);
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
}
