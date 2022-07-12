use super::{HitRecord, Hittable};
use crate::bvh::aabb::AABB;
use crate::material::Material;
use crate::utility::*;

pub enum Rect<T: Material> {
    XYRect(XYRect<T>),
    XZRect(XZRect<T>),
    YZRect(YZRect<T>),
}

impl<T: Material + Sync + Send> Hittable for Rect<T> {
    fn bounding_box(&self, st_time: f64, ed_time: f64) -> Option<AABB> {
        match self {
            Rect::XYRect(rect) => rect.bounding_box(st_time, ed_time),
            Rect::XZRect(rect) => rect.bounding_box(st_time, ed_time),
            Rect::YZRect(rect) => rect.bounding_box(st_time, ed_time),
        }
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Rect::XYRect(rect) => rect.hit(r, t_min, t_max),
            Rect::XZRect(rect) => rect.hit(r, t_min, t_max),
            Rect::YZRect(rect) => rect.hit(r, t_min, t_max),
        }
    }
}

////////////////////////////////xy_rect////////////////////////////////

pub struct XYRect<T: Material> {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    z: f64,
    mat: T,
}

impl<T: Material> XYRect<T> {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, z: f64, mat: T) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            z,
            mat,
        }
    }
}

impl<T: Material + Sync + Send> Hittable for XYRect<T> {
    fn bounding_box(&self, st_time: f64, ed_time: f64) -> Option<AABB> {
        let eps = 0.0001;
        Some(AABB::new(
            Point3::new(self.x0, self.y0, self.z - eps),
            Point3::new(self.x1, self.y1, self.z + eps),
        ))
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.z - r.orig.z) / r.dir.z;
        if (t < t_min || t > t_max) {
            return None;
        }
        let x = r.orig.x + t * r.dir.x;
        let y = r.orig.y + t * r.dir.y;
        if (x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1) {
            return None;
        }
        let mut rec = HitRecord {
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (y - self.y0) / (self.y1 - self.y0),
            p: r.at(t),
            mat_ptr: &self.mat,
            t,
            fornt_face: bool::default(),
            normal: Vec3::default(),
        };
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }
}

////////////////////////////////xz_rect//////////////////////////////

pub struct XZRect<T: Material> {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    y: f64,
    mat: T,
}

impl<T: Material> XZRect<T> {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, y: f64, mat: T) -> Self {
        Self {
            x0,
            x1,
            z0,
            z1,
            y,
            mat,
        }
    }
}

impl<T: Material + Sync + Send> Hittable for XZRect<T> {
    fn bounding_box(&self, st_time: f64, ed_time: f64) -> Option<AABB> {
        let eps = 0.0001;
        Some(AABB::new(
            Point3::new(self.x0, self.y - eps, self.z0),
            Point3::new(self.x1, self.y + eps, self.z1),
        ))
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.y - r.orig.y) / r.dir.y;
        if (t < t_min || t > t_max) {
            return None;
        }
        let x = r.orig.x + t * r.dir.x;
        let z = r.orig.z + t * r.dir.z;
        if (x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1) {
            return None;
        }
        let mut rec = HitRecord {
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (z - self.z0) / (self.z1 - self.z0),
            p: r.at(t),
            mat_ptr: &self.mat,
            t,
            fornt_face: bool::default(),
            normal: Vec3::default(),
        };
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }
}

////////////////////////////////yz_rect//////////////////////////////

pub struct YZRect<T: Material> {
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    x: f64,
    mat: T,
}

impl<T: Material> YZRect<T> {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, x: f64, mat: T) -> Self {
        Self {
            y0,
            y1,
            z0,
            z1,
            x,
            mat,
        }
    }
}

impl<T: Material + Sync + Send> Hittable for YZRect<T> {
    fn bounding_box(&self, st_time: f64, ed_time: f64) -> Option<AABB> {
        let eps = 0.0001;
        Some(AABB::new(
            Point3::new(self.x - eps, self.y0, self.z0),
            Point3::new(self.x + eps, self.y1, self.z1),
        ))
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.x - r.orig.x) / r.dir.x;
        if (t < t_min || t > t_max) {
            return None;
        }
        let y = r.orig.y + t * r.dir.y;
        let z = r.orig.z + t * r.dir.z;
        if (y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1) {
            return None;
        }
        let mut rec = HitRecord {
            u: (y - self.y0) / (self.y1 - self.y0),
            v: (z - self.z0) / (self.z1 - self.z0),
            p: r.at(t),
            mat_ptr: &self.mat,
            t,
            fornt_face: bool::default(),
            normal: Vec3::default(),
        };
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }
}
