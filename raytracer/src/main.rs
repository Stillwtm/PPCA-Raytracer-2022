#![allow(unused)]
mod basic;
mod bvh;
mod hittable;
pub mod material;
mod scene;
mod texture;
mod utility;

use crate::hittable::{hittable_list::HittableList, sphere::Sphere, Hittable};
use crate::material::lambertian;
use crate::material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal, Material};
use crate::utility::export::export_jpg_to_file;
use crate::utility::*;
use crate::utility::{export, multi_thread};
use basic::camera::Camera;
use scene::*;

use std::char::MAX;
use std::{fs::File, process::exit};

use image::{ImageBuffer, RgbImage};

use console::style;
use indicatif::{ProgressBar, ProgressStyle};

use rand::Rng;

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 1.0; //16.0 / 9;
    const IMAGE_WIDTH: usize = 1920;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    const IMGAE_QUALITY: u8 = 100;
    const SAMPLE_PER_PIXEL: usize = 1000;
    const MAX_DEPTH: usize = 50;
    const THREAD_NUM: usize = 8;
    let path = "output/output.jpg";

    // Clear screen
    print!("{}[2J", 27 as char); // Clear screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Set cursor position as 1,1

    // Generate scene
    let (world, cam) = scene::book2_final_scene(ASPECT_RATIO);

    // Generate image
    let thread_pool = multi_thread::gen_img_with_multi_threads(
        THREAD_NUM,
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        SAMPLE_PER_PIXEL,
        MAX_DEPTH,
        cam,
        world,
    );
    let output_pixel_color = multi_thread::collect_thread_results(thread_pool);

    // Output image to file
    let img =
        export::convert_vec_to_img(&output_pixel_color, IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);
    export::export_jpg_to_file(path, IMGAE_QUALITY, img);
}
