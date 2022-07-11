use crate::hittable::sphere;
use crate::hittable::{hittable_list::HittableList, sphere::Sphere, Hittable};
use crate::hittable::instance::motion::Motion;
use crate::material::{self, lambertian};
use crate::material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal, Material};
use crate::bvh::bvh_node::BvhNode;
use crate::utility::*;

use std::sync::Arc;

use rand::Rng;

pub fn random_ball_scene() -> HittableList {
    let mut world = HittableList::default();
    let mut new_world = HittableList::default();

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    new_world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    let mut rng = rand::thread_rng();
    for i in -11..11 {
        for j in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Point3::new(
                i as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                j as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if (choose_mat < 0.8) {
                    // diffuse
                    let albedo = Color::rand_vec() * Color::rand_vec();
                    let sphere_material = Lambertian::new(albedo);
                    let mov = Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                    world.add(Arc::new(Motion::new(Sphere::new(center, 0.2, sphere_material), mov, 0.0, 1.0)));
                    // world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if (choose_mat < 0.95) {
                    // metal
                    let albedo = Color::rand_vec_range(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Dielectric::new(1.5);
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let bvh = Arc::new(BvhNode::new_from_list(&mut world, 0.0, 1.0));
    new_world.add(bvh);

    new_world
}
