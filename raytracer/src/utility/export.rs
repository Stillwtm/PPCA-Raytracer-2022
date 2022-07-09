use crate::utility::*;

use console::style;
use image::{ImageBuffer, RgbImage};
use std::fs::File;

pub fn export_jpg_to_file(path: &str, img_quality: u8, img: RgbImage) {
    println!("🕓 Export JEPG image to file...");
    // Output image to file
    println!("  FilePath: \"{}\"", style(path).yellow());
    println!("  JPEG Image Quality: {}", style(img_quality).yellow());
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(
        &mut output_file,
        image::ImageOutputFormat::Jpeg(img_quality),
    ) {
        Ok(_) => {}
        // Err(_) => panic!("Outputting image fails."),
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }
}

pub fn convert_vec_to_img(v: &Vec<Color>, width: u32, height: u32) -> RgbImage {
    println!("🕒 Converting results to image...");
    let mut img: RgbImage = ImageBuffer::new(width, height);
    let mut pixel_id = 0;

    for y in (0..height).rev() {
        for x in 0..width {
            let pixel_color = v[pixel_id];
            let pixel = img.get_pixel_mut(x, y);
            *pixel = image::Rgb(pixel_color.to_u8_array());
            pixel_id += 1;
        }
    }
    img
}
