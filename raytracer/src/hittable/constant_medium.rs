use rand::Rng;

use super::{HitRecord, Hittable};
use crate::bvh::aabb::AABB;
use crate::material::{isotropic::Isotropic, Material};
use crate::texture::solid_color::SolidColor;
use crate::texture::Texture;
use crate::utility::*;

pub struct ConstantMedium<T, U>
where
    T: Material,
    U: Hittable,
{
    boundary: U,
    phase_function: T,
    neg_inv_density: f64,
}

impl<V: Texture, U: Hittable> ConstantMedium<Isotropic<V>, U> {
    pub fn new(boundary: U, d: f64, text: V) -> Self {
        Self {
            boundary,
            phase_function: Isotropic::new(text),
            neg_inv_density: -1.0 / d,
        }
    }
}

impl<U: Hittable> ConstantMedium<Isotropic<SolidColor>, U> {
    pub fn new_from_color(boundary: U, d: f64, color: Color) -> Self {
        Self {
            boundary,
            phase_function: Isotropic::new_from_color(color),
            neg_inv_density: -1.0 / d,
        }
    }
}

impl<T, U> Hittable for ConstantMedium<T, U>
where
    T: Material + Sync + Send,
    U: Hittable,
{
    fn bounding_box(&self, st_time: f64, ed_time: f64) -> Option<AABB> {
        self.boundary.bounding_box(st_time, ed_time)
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(mut rec1) = self.boundary.hit(r, NEG_INFINITY, INFINITY) {
            if let Some(mut rec2) = self.boundary.hit(r, rec1.t + 0.0001, INFINITY) {
                rec1.t = rec1.t.max(t_min);
                rec2.t = rec2.t.min(t_max);

                if rec1.t >= rec2.t {
                    return None;
                }

                rec1.t = rec1.t.max(0.0);

                let mut rng = rand::thread_rng();

                let ray_length = r.dir.length();
                let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                let hit_distance = self.neg_inv_density * f64::ln(rng.gen());

                if (hit_distance > distance_inside_boundary) {
                    return None;
                }

                let t = rec1.t + hit_distance / ray_length;
                let rec = HitRecord {
                    t,
                    p: r.at(t),
                    normal: Vec3::new(1.0, 0.0, 0.0),
                    fornt_face: true,
                    mat_ptr: &self.phase_function,
                    u: f64::default(),
                    v: f64::default(),
                };

                return Some(rec);
            }
        }

        None
    }
}
