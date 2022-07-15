use crate::basic::camera::Camera;
use crate::bvh::bvh_node::BvhNode;
use crate::hittable::constant_medium::ConstantMedium;
use crate::hittable::hittable_list;
use crate::hittable::instance::flip_face::FlipFace;
use crate::hittable::instance::motion::Motion;
use crate::hittable::instance::rotation::{RotationX, RotationZ};
use crate::hittable::instance::{rotation::RotationY, translation::Translation};
use crate::hittable::{
    aarect::{XYRect, XZRect, YZRect},
    cuboid::Cuboid,
    obj_model::ObjModel,
    sphere::Sphere,
    triangle::Triangle,
};
use crate::hittable::{hittable_list::HittableList, Hittable};
use crate::material::{self, lambertian};
use crate::material::{
    dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian, metal::Metal,
    Material,
};
use crate::texture::checker_texture::CheckerTexture;
use crate::texture::image_texture::{ImageTexture, RandImageTexture};
use crate::texture::noise_texture::NoiseTexture;
use crate::texture::solid_color::SolidColor;
use crate::utility::*;

use std::sync::Arc;

use rand::Rng;

pub fn random_ball_scene(aspect_ratio: f64) -> (HittableList, Camera) {
    // World
    let mut world = HittableList::default();
    let mut new_world = HittableList::default();

    let checker =
        CheckerTexture::new_form_color(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    new_world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(checker),
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
                    let sphere_material = Lambertian::new(SolidColor::new(albedo));
                    let mov = Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                    world.add(Arc::new(Motion::new(
                        Sphere::new(center, 0.2, sphere_material),
                        mov,
                        0.0,
                        1.0,
                    )));
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

    let material2 = Lambertian::new(SolidColor::new_rgb(0.4, 0.2, 0.1));
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

    // Camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    (new_world, cam)
}

pub fn two_spheres(aspect_ratio: f64) -> (HittableList, Camera) {
    // World
    let mut objects = HittableList::default();
    let mut world = HittableList::default();
    let checker =
        CheckerTexture::new_form_color(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    let mat = Lambertian::new(checker);
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        mat,
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        mat,
    )));

    let bvh = BvhNode::new_from_list(&mut objects, 0.0, 1.0);
    world.add(Arc::new(bvh));

    // Camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    (world, cam)
}

pub fn two_perlin_spheres(aspect_ratio: f64) -> (HittableList, Camera) {
    // World
    let mut world = HittableList::default();
    let pertext = NoiseTexture::new(4.0);
    let mat = Lambertian::new(pertext);
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        mat,
    )));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, mat)));

    // Camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    (world, cam)
}

pub fn earth(aspect_ratio: f64) -> (HittableList, Camera) {
    // World
    let mut objects = HittableList::default();
    let earth_texture = ImageTexture::new_form_file("images/earthmap.jpg");
    let mat = Lambertian::new(earth_texture);
    objects.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, mat)));

    // Camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    (objects, cam)
}

pub fn cornell_box(aspect_ratio: f64) -> (HittableList, HittableList, Camera) {
    // World
    let mut objects = HittableList::default();
    let mut lights = HittableList::default();
    let mut world = HittableList::default();

    let red = Lambertian::new_form_color(Color::new(0.65, 0.05, 0.05));
    let white = Lambertian::new_form_color(Color::new(0.73, 0.73, 0.73));
    let green = Lambertian::new_form_color(Color::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new_form_color(Color::new(7., 7., 7.));

    objects.add(Arc::new(YZRect::new(0., 555., 0., 555., 555., green)));
    objects.add(Arc::new(YZRect::new(0., 555., 0., 555., 0., red)));
    objects.add(Arc::new(FlipFace::new(XZRect::new(
        113.,
        443.,
        127.,
        432.,
        554.,
        light.clone(),
    ))));
    objects.add(Arc::new(XZRect::new(0., 555., 0., 555., 0., white)));
    objects.add(Arc::new(XZRect::new(0., 555., 0., 555., 555., white)));
    objects.add(Arc::new(XYRect::new(0., 555., 0., 555., 555., white)));

    let aluminum = Metal::new(Color::new(0.8, 0.85, 0.88), 0.0);
    let box1 = Cuboid::new(
        Point3::new(0., 0., 0.),
        Point3::new(160., 330., 165.),
        white,
    );
    let box1 = RotationY::new(box1, 15.);
    let box1 = Translation::new(box1, Point3::new(265., 0., 295.));
    // objects.add(Arc::new(box1));

    // let glass = Dielectric::new(1.5);
    // objects.add(Arc::new(Sphere::new(
    //     Point3::new(190., 90., 190.),
    //     90.,
    //     glass,
    // )));

    let box2 = Cuboid::new(
        Point3::new(0., 0., 0.),
        Point3::new(165., 165., 165.),
        white,
    );
    let box2 = RotationY::new(box2, -18.);
    let box2 = Translation::new(box2, Point3::new(130., 0., 65.));
    // objects.add(Arc::new(box2));

    // Volume Smoke
    objects.add(Arc::new(ConstantMedium::new_from_color(
        box1,
        0.01,
        Color::new(0.0, 0.0, 0.0),
    )));
    objects.add(Arc::new(ConstantMedium::new_from_color(
        box2,
        0.01,
        Color::new(1.0, 1.0, 1.0),
    )));

    // let bvh = Arc::new(BvhNode::new_from_list(&mut objects, 0.0, 1.0));
    // world.add(bvh);

    // Lights
    lights.add(Arc::new(XZRect::new(213., 343., 227., 332., 554., light)));
    // lights.add(Arc::new(Sphere::new(
    //     Point3::new(190., 90., 190.),
    //     90.,
    //     glass,
    // )));

    // Camera
    let look_from = Point3::new(278.0, 278.0, -800.0);
    let look_at = Point3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        40.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    (objects, lights, cam)
}

pub fn book2_final_scene(aspect_ratio: f64) -> (HittableList, Camera) {
    // World
    let mut boxes1 = HittableList::default();
    let ground = Lambertian::new_form_color(Color::new(0.48, 0.83, 0.53));

    let boses_per_side = 20;
    let mut rng = rand::thread_rng();
    for i in 0..boses_per_side {
        for j in 0..boses_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rng.gen_range(1.0..101.0);
            let z1 = z0 + w;

            boxes1.add(Arc::new(Cuboid::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground,
            )));
        }
    }

    let mut objects = HittableList::default();

    objects.add(Arc::new(BvhNode::new_from_list(&mut boxes1, 0.0, 1.0)));

    let light = DiffuseLight::new_form_color(Color::new(7., 7., 7.));
    objects.add(Arc::new(XZRect::new(123., 423., 147., 412., 554., light)));

    let center = Point3::new(400., 400., 400.);
    let mov = Vec3::new(30., 0., 0.);
    let moving_sphere_mat = Lambertian::new_form_color(Color::new(0.7, 0.3, 0.1));
    let sph = Sphere::new(center, 50., moving_sphere_mat);
    objects.add(Arc::new(Motion::new(sph, mov, 0.0, 1.0)));

    objects.add(Arc::new(Sphere::new(
        Point3::new(260., 150., 45.),
        50.,
        Dielectric::new(1.5),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0., 150., 145.),
        50.,
        Metal::new(Color::new(0.8, 0.8, 0.9), 1.0),
    )));

    let boundary = Sphere::new(Point3::new(360., 150., 145.), 70., Dielectric::new(1.5));
    objects.add(Arc::new(boundary.clone()));
    objects.add(Arc::new(ConstantMedium::new_from_color(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));
    let boundary = Sphere::new(Point3::new(0., 0., 0.), 5000., Dielectric::new(1.5));
    objects.add(Arc::new(ConstantMedium::new_from_color(
        boundary,
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    )));

    let emat = Lambertian::new(ImageTexture::new_form_file("images/earthmap.jpg"));
    objects.add(Arc::new(Sphere::new(
        Point3::new(400., 200., 400.),
        100.,
        emat,
    )));

    let pertext = NoiseTexture::new(0.1);
    objects.add(Arc::new(Sphere::new(
        Point3::new(220., 280., 300.),
        80.,
        Lambertian::new(pertext),
    )));

    let mut boxes2 = HittableList::default();
    let white = Lambertian::new_form_color(Color::new(0.73, 0.73, 0.73));
    let ns = 1000;
    for i in 0..ns {
        boxes2.add(Arc::new(Sphere::new(
            Point3::rand_vec_range(0., 165.),
            10.,
            white,
        )));
    }

    let bvh = BvhNode::new_from_list(&mut boxes2, 0.0, 1.0);
    objects.add(Arc::new(Translation::new(
        RotationY::new(bvh, 15.),
        Vec3::new(-100., 270., 395.),
    )));

    // Camera
    let look_from = Point3::new(478.0, 278.0, -600.0);
    let look_at = Point3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        40.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    (objects, cam)
}

pub fn final_scene(aspect_ratio: f64) -> (HittableList, HittableList, Camera) {
    // World
    let mut objects = HittableList::default();
    let mut world = HittableList::default();
    let mut lights = HittableList::default();

    // 灯光
    let light = DiffuseLight::new_form_color(Color::new(10., 10., 10.));
    let light_ball = XZRect::new(400., 1400., -200., 1400., 1800., light);
    objects.add(Arc::new(FlipFace::new(light_ball.clone())));
    // let light_ball = Sphere::new(
    //     Point3::new(200., 1800., 200.),
    //     500.,
    //     light.clone(),
    // );
    // objects.add(Arc::new(light_ball.clone()));

    // 地面
    // let white = Lambertian::new_form_color(Color::new(0.73, 0.73, 0.73));
    let light_yellow = Lambertian::new_form_color(Color::new(197., 188., 164.) / 255.);
    world.add(Arc::new(XZRect::new(
        -10000.,
        10000.,
        -10000.,
        25000.,
        0.,
        light_yellow,
    )));

    // 三个柱子
    let cube_diff_mat = Lambertian::new_form_color(Color::new(0.73, 0.73, 0.73));
    let cube_diff = ObjModel::new_from_file("models/cube_diff.obj", 10., cube_diff_mat);
    let offset = Vec3::new(1488., 0., 800.);
    let cube_diff = Translation::new(RotationY::new(cube_diff, 180.), offset);
    objects.add(Arc::new(cube_diff));

    // 一个金属柱子
    let cube_spec_mat = Metal::new(Color::new(0.8, 0.85, 0.88), 0.0);
    let cube_spec = Cuboid::new(
        Point3::new(-150., 0., -150.),
        Point3::new(150., 400., 150.),
        cube_spec_mat,
    );
    let cube_spec = RotationY::new(cube_spec, 0.);
    let cube_spec = Translation::new(cube_spec, Point3::new(1800., 0., 1000.));
    objects.add(Arc::new(cube_spec));

    // 一个金属柱子
    let cube_spec_mat = Metal::new(Color::new(0.8, 0.85, 0.88), 0.0);
    let cube_spec = Cuboid::new(
        Point3::new(-150., 0., -150.),
        Point3::new(150., 550., 150.),
        cube_spec_mat,
    );
    let cube_spec = RotationY::new(cube_spec, -18.);
    let cube_spec = Translation::new(cube_spec, Point3::new(800., 0., 500.));
    objects.add(Arc::new(cube_spec));

    // 镜面
    let mirr_mat = Metal::new(Color::new(0.8, 0.85, 0.88), 0.0);
    let mirror = XYRect::new(-10000., 10000., -10000., 10000., 2500., mirr_mat);
    objects.add(Arc::new(mirror));

    let green_mat = Lambertian::new(SolidColor::new_rgb(0.7, 0.85, 0.5));
    let green_mat1 = Metal::new(Color::new(0.5, 0.8, 0.4), 1.0);
    let yellow_mat = Lambertian::new(SolidColor::new(Color::new(198., 145., 69.) / 255.));
    let yellow_mat1 = Metal::new(Color::new(198., 145., 69.) / 255., 1.0);
    let grey_mat = Lambertian::new(SolidColor::new(Color::new(98., 99., 100.) / 255.));
    let grey_mat1 = Metal::new(Color::new(98., 99., 100.) / 255., 1.0);
    let glass = Dielectric::new(1.5);
    let bronze_mat = Lambertian::new(RandImageTexture::new_form_file("models/bronze.jpg"));
    let gold_mat = Lambertian::new(RandImageTexture::new_form_file("models/gold.jpeg"));

    // 1: rat
    let head = ObjModel::new_from_file("models/1.obj", 10., green_mat1);
    let offset1 = Vec3::new(0., 0., 0.) - head.center;
    let offset2 = Vec3::new(1788., 150., 700.);
    let head = Translation::new(
        RotationY::new(RotationX::new(Translation::new(head, offset1), -90.), 180.),
        offset2,
    );
    objects.add(Arc::new(head));

    // 2: cattle
    let head = ObjModel::new_from_file("models/2.obj", 10., green_mat1);
    let offset1 = Vec3::new(0., 0., 0.) - head.center;
    let offset2 = Vec3::new(1388., 550., 400.);
    let head = Translation::new(
        RotationY::new(RotationX::new(Translation::new(head, offset1), -90.), -90.),
        offset2,
    );
    objects.add(Arc::new(head));

    // 3: tiger
    let head = ObjModel::new_from_file("models/3.obj", 10., grey_mat1);
    let offset1 = Vec3::new(0., 0., 0.) - head.center;
    let offset2 = Vec3::new(150., 150., 150.);
    let head = Translation::new(
        RotationZ::new(
            RotationY::new(RotationX::new(Translation::new(head, offset1), 0.), -90.),
            -90.,
        ),
        offset2,
    );
    objects.add(Arc::new(head));

    // 4: rabbit
    let head = ObjModel::new_from_file("models/4.obj", 10., grey_mat1);
    let offset1 = Vec3::new(0., 0., 0.) - head.center;
    let offset2 = Vec3::new(1800., 120., 200.);
    let head = Translation::new(
        RotationY::new(RotationX::new(Translation::new(head, offset1), -90.), 80.),
        offset2,
    );
    objects.add(Arc::new(head));

    // 5: dragon
    let head = ObjModel::new_from_file("models/5.obj", 10., yellow_mat1);
    let offset1 = Vec3::new(0., 0., 0.) - head.center;
    let offset2 = Vec3::new(1188., 850., 700.);
    let head = Translation::new(
        RotationY::new(RotationX::new(Translation::new(head, offset1), -90.), 180.),
        offset2,
    );
    objects.add(Arc::new(head));

    // 6: snake
    let head = ObjModel::new_from_file("models/6.obj", 10., yellow_mat1);
    let offset1 = Vec3::new(0., 0., 0.) - head.center;
    let offset2 = Vec3::new(460., 350., 400.);
    let head = Translation::new(
        RotationY::new(RotationX::new(Translation::new(head, offset1), -90.), 0.),
        offset2,
    );
    objects.add(Arc::new(head));

    // 7: horse
    let head = ObjModel::new_from_file("models/7.obj", 10., green_mat1);
    let offset1 = Vec3::new(0., 0., 0.) - head.center;
    let offset2 = Vec3::new(1000., 100., 350.);
    let head = Translation::new(
        RotationY::new(RotationX::new(Translation::new(head, offset1), -90.), -30.),
        offset2,
    );
    objects.add(Arc::new(head));

    // 8: goat
    let head = ObjModel::new_from_file("models/8.obj", 10., green_mat1);
    let offset1 = Vec3::new(0., 0., 0.) - head.center;
    let offset2 = Vec3::new(1800., 570., 1000.);
    let head = Translation::new(
        RotationY::new(RotationX::new(Translation::new(head, offset1), -90.), -100.),
        offset2,
    );
    objects.add(Arc::new(head));

    // 9: monkey
    let head = ObjModel::new_from_file("models/9.obj", 10., yellow_mat1);
    let offset1 = Vec3::new(0., 0., 0.) - head.center;
    let offset2 = Vec3::new(1450., 50., 200.);
    let head = Translation::new(
        RotationY::new(RotationX::new(Translation::new(head, offset1), -90.), -140.),
        offset2,
    );
    objects.add(Arc::new(head));

    // 10: chicken
    let head = ObjModel::new_from_file("models/10.obj", 10., green_mat1);
    let offset1 = Vec3::new(0., 0., 0.) - head.center;
    let offset2 = Vec3::new(2100., 150., 800.);
    let head = Translation::new(
        RotationZ::new(
            RotationY::new(
                RotationZ::new(RotationX::new(Translation::new(head, offset1), -90.), -90.),
                180.,
            ),
            180.,
        ),
        offset2,
    );
    objects.add(Arc::new(head));

    // 11: dog
    let head = ObjModel::new_from_file("models/11.obj", 10., grey_mat1);
    let offset1 = Vec3::new(0., 0., 0.) - head.center;
    let offset2 = Vec3::new(150., 180., 800.);
    let head = Translation::new(
        RotationY::new(RotationX::new(Translation::new(head, offset1), -90.), 200.),
        offset2,
    );
    objects.add(Arc::new(head));

    // 12: pig
    let head = ObjModel::new_from_file("models/12.obj", 10., grey_mat1);
    let offset1 = Vec3::new(0., 0., 0.) - head.center;
    let offset2 = Vec3::new(800., 600., 500.);
    let head = Translation::new(
        RotationY::new(RotationX::new(Translation::new(head, offset1), -90.), -90.),
        offset2,
    );
    objects.add(Arc::new(head));

    // Add a bit fog
    // let boundary = Sphere::new(Point3::new(0., 0., 0.), 5000., Dielectric::new(1.5));
    // world.add(Arc::new(ConstantMedium::new_from_color(
    //     boundary,
    //     0.0001,
    //     Color::new(1.0, 1.0, 1.0),
    // )));

    // Bvh
    world.add(Arc::new(BvhNode::new_from_list(&mut objects, 0.0, 1.0)));

    // Lights
    lights.add(Arc::new(light_ball));

    // Camera
    let look_from = Point3::new(900.0, 1250.0, -1300.0);
    let look_at = Point3::new(1000.0, 600.0, 150.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        40.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    (world, lights, cam)
}
