use crate::utility::*;
use std::{fs::File, process::exit};
use image::{ImageBuffer, RgbImage};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};

pub fn write_color(pixel_color: &Color, sample_per_pixel: usize) -> [u8; 3] {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    // Divide the color by the number of samples
    let scale = 1.0 / sample_per_pixel as f64;
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

    // Return the [0, 255] value of each color component
    [(256.0 * r.clamp(0.0, 0.999)) as u8, 
    (256.0 * g.clamp(0.0, 0.999)) as u8, 
    (256.0 * b.clamp(0.0, 0.999)) as u8]

    // // Write the [0, 255] value of each color component
    // print!("{} {} {}\n", (256.0 * clamp(r, 0.0, 0.999)) as u8, 
    //                     (256.0 * clamp(g, 0.0, 0.999)) as u8, 
    //                     (256.0 * clamp(b, 0.0, 0.999)) as u8);
}

/*
fn main() {
    print!("{}[2J", 27 as char); // Clear screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Set cursor position as 1,1

    let height = 800;
    let width = 800;
    let quality = 60; // From 0 to 100
    let path = "output/output.jpg";

    println!(
        "Image size: {}\nJPEG quality: {}",
        style(width.to_string() + &"x".to_string() + &height.to_string()).yellow(),
        style(quality.to_string()).yellow(),
    );

    // Create image data
    let mut img: RgbImage = ImageBuffer::new(width, height);
    // Progress bar UI powered by library `indicatif`
    // Get environment variable CI, which is true for GitHub Action
    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };
    progress.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] [{pos}/{len}] ({eta})")
        .progress_chars("#>-"));

    // Generate image
    for y in 0..height {
        for x in 0..width {
            let pixel_color = [
                (y as f32 / height as f32 * 255.).floor() as u8,
                ((x + height - y) as f32 / (height + width) as f32 * 255.).floor() as u8,
                (x as f32 / height as f32 * 255.).floor() as u8,
            ];
            let pixel = img.get_pixel_mut(x, height - y - 1);
            *pixel = image::Rgb(pixel_color);
            progress.inc(1);
        }
    }
    progress.finish();

    // Output image to file
    println!("Ouput image as \"{}\"", style(path).yellow());
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        // Err(_) => panic!("Outputting image fails."),
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

    exit(0);
}
*/