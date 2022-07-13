use rand::Rng;

pub use super::vec3::{Color, Point3, Vec3};
use crate::{
    hittable::{hittable_list::HittableList, Hittable},
    material::DiffOrSpec,
    pdf::{
        cosine_pdf::CosinePDF,
        hittable_pdf::HittablePDF,
        mixture_pdf::{self, MixturePDF},
        PDF,
    },
};

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
    pub tm: f64,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3, tm: f64) -> Self {
        Self { orig, dir, tm }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}

pub fn ray_color(
    r: &Ray,
    background: &Color,
    world: &HittableList,
    lights: &HittableList,
    depth: usize,
) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        // let mut attenuation = Vec3::default();
        let emmited = rec.mat_ptr.emitted(&r, &rec, rec.u, rec.v, rec.p);
        return if let Some(scat_rec) = rec.mat_ptr.scatter(&r, &rec) {
            match scat_rec.ray_type {
                DiffOrSpec::Specular(scattered) => {
                    scat_rec.attenuation
                        * ray_color(&scattered, background, world, lights, depth - 1)
                }
                DiffOrSpec::Diffuse(cos_pdf) => {
                    let light_pdf = HittablePDF::new(lights, rec.p);
                    let mix_pdf = MixturePDF::new(light_pdf, cos_pdf);

                    let scattered = Ray::new(rec.p, mix_pdf.generate(), r.tm);
                    let pdf = mix_pdf.value(&scattered.dir);

                    emmited
                        + scat_rec.attenuation
                            * rec.mat_ptr.scattering_pdf(r, &rec, &scattered)
                            * ray_color(&scattered, background, world, lights, depth - 1)
                            / pdf
                }
            }
        } else {
            emmited
        };
    }

    *background

    // A sky background
    // let unit_direction: Vec3 = r.dir.unit_vector();
    // let t = 0.5 * (unit_direction.y + 1.0);
    // (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
