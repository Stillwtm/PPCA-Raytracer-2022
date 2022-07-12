use image::pnm::SampleEncoding;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use crate::basic::camera::Camera;
use crate::basic::ray;
use crate::hittable::hittable_list::HittableList;
use crate::scene;
use crate::utility::*;
use std::sync::mpsc::{self, Receiver};
use std::thread::{self, JoinHandle};

use console::style;

use rand::Rng;

pub fn gen_img_with_multi_threads(
    thread_num: usize,
    img_width: usize,
    img_height: usize,
    sample_per_pixel: usize,
    max_depth: usize,
    camera: Camera,
    world: HittableList,
) -> Vec<(JoinHandle<()>, Receiver<Vec<Color>>)> {
    println!("🕐 Generating image...");
    println!(
        "   Image size: {}",
        style(img_width.to_string() + &"x".to_string() + &img_height.to_string()).yellow(),
    );

    // create multiprogress bar
    let multiprogress = MultiProgress::new();
    multiprogress.set_move_cursor(true); // turn on this to reduce flickering

    let mut thread_pool = Vec::new();

    let section_row_num = img_height / thread_num;

    for thread_id in 0..thread_num {
        // one thread handles row: [row_beg, row_end)
        let row_beg = thread_id * section_row_num;
        let row_end = if thread_id == thread_num - 1 {
            img_height
        } else {
            row_beg + section_row_num
        };

        let (tx, rx) = mpsc::channel();
        let section_world = world.clone();
        let background = Color::new(0.0, 0.0, 0.0);
        let cam = camera.clone();
        let progress = multiprogress.add(create_progress_bar(
            (img_width * (row_end - row_beg)) as u64,
        ));

        thread_pool.push((
            thread::spawn(move || {
                let mut section_pixel_color = Vec::<Color>::new();

                let mut rng = rand::thread_rng();
                for y in row_beg..row_end {
                    for x in 0..img_width {
                        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                        for s in 0..sample_per_pixel {
                            let u = (x as f64 + rng.gen::<f64>()) / (img_width - 1) as f64;
                            let v = (y as f64 + rng.gen::<f64>()) / (img_height - 1) as f64;
                            let r = cam.get_ray(u, v);
                            pixel_color +=
                                ray::ray_color(&r, &background, &section_world, max_depth);
                        }
                        section_pixel_color.push(pixel_color.calc_color(sample_per_pixel));
                        progress.inc(1);
                    }
                }

                tx.send(section_pixel_color).unwrap();
                progress.finish();
            }),
            rx,
        ));
    }

    multiprogress.join();

    thread_pool
}

pub fn collect_thread_results(
    thread_pool: Vec<(JoinHandle<()>, Receiver<Vec<Color>>)>,
) -> Vec<Color> {
    println!("🕑 Collecting thread results...");

    let mut output_pixel_color = Vec::new();
    let progress = create_progress_bar(thread_pool.len() as u64);

    for thread in thread_pool {
        match thread.0.join() {
            Ok(_) => {
                let mut received = thread.1.recv().unwrap();
                output_pixel_color.append(&mut received);
                progress.inc(1);
            }
            Err(_) => {
                println!("  Error: {}", style("Joining the thread failed!").red());
            }
        }
    }

    progress.finish();
    output_pixel_color
}

fn create_progress_bar(len: u64) -> ProgressBar {
    // Progress bar UI powered by library `indicatif`
    // Get environment variable CI, which is true for GitHub Action
    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new(len as u64)
    };
    progress.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] [{pos}/{len}] ({eta})")
        .progress_chars("#>-"));
    progress
}
