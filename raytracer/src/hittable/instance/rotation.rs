use crate::bvh::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::utility::*;

pub struct RotationY<T> {
    obj: T,
    sin_theta: f64,
    cos_theta: f64,
    opt_box: Option<AABB>,
}

impl<T: Hittable> RotationY<T> {
    pub fn new(obj: T, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let opt_box = if let Some(bbox) = obj.bounding_box(0.0, 1.0) {
            let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
            let mut max = Point3::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY);

            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let i = i as f64;
                        let j = j as f64;
                        let k = k as f64;
                        let x = i * bbox.maximum.x + (1. - i) * bbox.minimum.x;
                        let y = j * bbox.maximum.y + (1. - j) * bbox.minimum.y;
                        let z = k * bbox.maximum.z + (1. - k) * bbox.minimum.z;

                        let newx = cos_theta * x + sin_theta * z;
                        let newz = -sin_theta * x + cos_theta * z;

                        let tester = Vec3::new(newx, y, newz);
                        for c in 0..3 {
                            min[c] = min[c].min(tester[c]);
                            max[c] = max[c].max(tester[c]);
                        }
                    }
                }
            }
            Some(AABB::new(min, max))
        } else {
            None
        };

        Self {
            obj,
            sin_theta,
            cos_theta,
            opt_box,
        }
    }
}

impl<T: Hittable> Hittable for RotationY<T> {
    fn bounding_box(&self, st_time: f64, ed_time: f64) -> Option<AABB> {
        self.opt_box
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = r.orig;
        let mut direction = r.dir;

        origin[0] = self.cos_theta * r.orig[0] - self.sin_theta * r.orig[2];
        origin[2] = self.sin_theta * r.orig[0] + self.cos_theta * r.orig[2];

        direction[0] = self.cos_theta * r.dir[0] - self.sin_theta * r.dir[2];
        direction[2] = self.sin_theta * r.dir[0] + self.cos_theta * r.dir[2];

        let rotated_r = Ray::new(origin, direction, r.tm);

        if let Some(mut rec) = self.obj.hit(&rotated_r, t_min, t_max) {
            let mut p = rec.p;
            let mut normal = rec.normal;

            p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
            p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

            normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
            normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

            rec.p = p;
            rec.set_face_normal(&rotated_r, normal);

            Some(rec)
        } else {
            None
        }
    }
}

pub struct RotationX<T> {
    obj: T,
    sin_theta: f64,
    cos_theta: f64,
    opt_box: Option<AABB>,
}

impl<T: Hittable> RotationX<T> {
    pub fn new(obj: T, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let opt_box = if let Some(bbox) = obj.bounding_box(0.0, 1.0) {
            let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
            let mut max = Point3::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY);

            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let i = i as f64;
                        let j = j as f64;
                        let k = k as f64;
                        let x = i * bbox.maximum.x + (1. - i) * bbox.minimum.x;
                        let y = j * bbox.maximum.y + (1. - j) * bbox.minimum.y;
                        let z = k * bbox.maximum.z + (1. - k) * bbox.minimum.z;

                        let newz = cos_theta * z + sin_theta * y;
                        let newy = -sin_theta * z + cos_theta * y;

                        let tester = Vec3::new(x, newy, newz);
                        for c in 0..3 {
                            min[c] = min[c].min(tester[c]);
                            max[c] = max[c].max(tester[c]);
                        }
                    }
                }
            }
            Some(AABB::new(min, max))
        } else {
            None
        };

        Self {
            obj,
            sin_theta,
            cos_theta,
            opt_box,
        }
    }
}

impl<T: Hittable> Hittable for RotationX<T> {
    fn bounding_box(&self, st_time: f64, ed_time: f64) -> Option<AABB> {
        self.opt_box
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = r.orig;
        let mut direction = r.dir;

        origin[2] = self.cos_theta * r.orig[2] - self.sin_theta * r.orig[1];
        origin[1] = self.sin_theta * r.orig[2] + self.cos_theta * r.orig[1];

        direction[2] = self.cos_theta * r.dir[2] - self.sin_theta * r.dir[1];
        direction[1] = self.sin_theta * r.dir[2] + self.cos_theta * r.dir[1];

        let rotated_r = Ray::new(origin, direction, r.tm);

        if let Some(mut rec) = self.obj.hit(&rotated_r, t_min, t_max) {
            let mut p = rec.p;
            let mut normal = rec.normal;

            p[2] = self.cos_theta * rec.p[2] + self.sin_theta * rec.p[1];
            p[1] = -self.sin_theta * rec.p[2] + self.cos_theta * rec.p[1];

            normal[2] = self.cos_theta * rec.normal[2] + self.sin_theta * rec.normal[1];
            normal[1] = -self.sin_theta * rec.normal[2] + self.cos_theta * rec.normal[1];

            rec.p = p;
            rec.set_face_normal(&rotated_r, normal);

            Some(rec)
        } else {
            None
        }
    }
}
