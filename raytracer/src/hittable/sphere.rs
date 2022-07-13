use super::*;
use crate::basic::onb::ONB;
use crate::bvh::aabb::AABB;
use crate::utility::*;

#[derive(Clone)]
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

    // p is a given point on the unit sphere
    pub fn get_sphere_uv(p: Point3) -> (f64, f64) {
        let theta = f64::acos(-p.y);
        let phi = f64::atan2(-p.z, p.x) + PI;

        (phi / (2.0 * PI), theta / PI)
    }
}

impl<T: Material + Sync + Send> Hittable for Sphere<T> {
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

        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;
        let (u, v) = Sphere::<T>::get_sphere_uv(outward_normal);

        let mut rec = HitRecord {
            t: root,
            p,
            mat_ptr: &self.mat,
            normal: Vec3::default(),
            u,
            v,
            front_face: bool::default(),
        };

        rec.set_face_normal(&r, outward_normal);

        Some(rec)
    }

    fn bounding_box(&self, st_time: f64, ed_time: f64) -> Option<AABB> {
        Some(AABB::new(
            self.center - self.radius,
            self.center + self.radius,
        ))
    }

    fn pdf_value(&self, orig: &Point3, v: &Vec3) -> f64 {
        if let Some(rec) = self.hit(&Ray::new(*orig, *v, 0.0), 0.001, INFINITY) {
            let cos_theta_max =
                (1.0 - self.radius.powi(2) / (self.center - *orig).length_squared()).sqrt();
            let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);
            1.0 / solid_angle
        } else {
            0.0
        }
    }

    fn random(&self, orig: Vec3) -> Vec3 {
        let direction = self.center - orig;
        let distance_squared = direction.length_squared();
        let uvw = ONB::build_from_w(direction);

        uvw.local(Vec3::rand_to_sphere(self.radius, distance_squared))
    }
}
