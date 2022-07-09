#![allow(unused)]
mod vec3;
mod ray;
mod hittable;
mod utility;
mod basic;
pub mod material;
mod scene;

use crate::material::lambertian;
use crate::utility::*;
use basic::{
    camera::Camera,
    color,
};
use crate::hittable::{
    hittable_list::HittableList,
    Hittable,
    sphere::Sphere,
};
use crate::material::{
    Material,
    lambertian::Lambertian,
    metal::Metal,
    dielectric::Dielectric,
};
use scene::*;

use std::{fs::File, process::exit};

use image::{ImageBuffer, RgbImage};

use console::style;
use indicatif::{ProgressBar, ProgressStyle};

use rand::Rng;

fn ray_color(r: &Ray, world: &HittableList, depth: usize) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {        
        let mut attenuation = Vec3::default();
        return if let Some(scattered) = rec.mat_ptr.scatter(&r, &rec, &mut attenuation) {
            attenuation * ray_color(&scattered, &world, depth - 1)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    }

    let unit_direction: Vec3 = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: usize = 1200;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    const IMGAE_QUALITY: u8 = 60;
    const SAMPLE_PER_PIXEL:usize = 500;
    const MAX_DEPTH: usize = 50;
    let path = "output/book1.jpg";

    // World

    let world = random_ball_scene();

    // let mut world = HittableList::default();
    
    // let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    // let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    // let material_left = Dielectric::new(1.5);
    // let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);

    // world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    // world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center)));
    // world.add(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    // world.add(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.45, material_left)));
    // world.add(Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));
    
    // Camera

    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    
    let cam = Camera::new(look_from, look_at, vup, 20.0, ASPECT_RATIO, aperture, dist_to_focus);

    // Render and Output
    
    print!("{}[2J", 27 as char); // Clear screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Set cursor position as 1,1

    println!(
        "Image size: {}\nJPEG quality: {}",
        style(IMAGE_WIDTH.to_string() + &"x".to_string() + &IMAGE_HEIGHT.to_string()).yellow(),
        style(IMGAE_QUALITY.to_string()).yellow(),
    );

    // Create image data
    let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);

    // Progress bar UI powered by library `indicatif`
    // Get environment variable CI, which is true for GitHub Action
    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((IMAGE_HEIGHT * IMAGE_WIDTH) as u64)
    };
    progress.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] [{pos}/{len}] ({eta})")
        .progress_chars("#>-"));

    // Generate image
    let mut rng = rand::thread_rng();
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for s in 0..SAMPLE_PER_PIXEL {
                let u = (i as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            // color::write_color(&pixel_color, SAMPLE_PER_PIXEL);
            // let pixel_color = [
            //     (y as f32 / height as f32 * 255.).floor() as u8,
            //     ((x + height - y) as f32 / (height + width) as f32 * 255.).floor() as u8,
            //     (x as f32 / height as f32 * 255.).floor() as u8,
            // ];
            let pixel = img.get_pixel_mut(i as u32, (IMAGE_HEIGHT - j - 1) as u32);
            *pixel = image::Rgb(color::write_color(&pixel_color, SAMPLE_PER_PIXEL));
            progress.inc(1);
        }
    }
    progress.finish();

    // Output image to file
    println!("Ouput image as \"{}\"", style(path).yellow());
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(IMGAE_QUALITY)) {
        Ok(_) => {}
        // Err(_) => panic!("Outputting image fails."),
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

    exit(0);

    /*
    let mut rng = rand::thread_rng();
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for s in 0..SAMPLE_PER_PIXEL {
                let u = (i as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            color::write_color(&pixel_color, SAMPLE_PER_PIXEL);
        }
    }
    */
}