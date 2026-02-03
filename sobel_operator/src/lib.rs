use image::{GrayImage, Luma};
use rayon::prelude::*;
use std::{env, fs};

const IMAGE_PATH: &str = "images";
const PROCESSED_PATH: &str = "processed_images";

const SOBEL_X_FILTER: [i32; 9] = [-1, 0, 1, -2, 0, 2, -1, 0, 1];
const SOBEL_Y_FILTER: [i32; 9] = [-1, -2, -1, 0, 0, 0, 1, 2, 1];

pub fn sobel_operation() {
    let mut image_dir = env::current_dir().unwrap();
    let mut processed_dir = env::current_dir().unwrap();
    image_dir.push(IMAGE_PATH);
    processed_dir.push(PROCESSED_PATH);
    fs::read_dir(image_dir)
        .unwrap()
        .filter_map(|result| result.ok())
        .collect::<Vec<_>>()
        .into_par_iter()
        .for_each(|file| {
            let image: image::ImageBuffer<Luma<u8>, Vec<u8>> =
                image::open(file.path()).unwrap().into_luma8();
            let image = process_image(&image);
            let mut new_path = processed_dir.clone();
            new_path.push(file.file_name());
            image.save(&new_path).unwrap();
        })
}

pub fn process_image(image: &GrayImage) -> GrayImage {
    // left top shifted?
    let width = image.width() as usize;
    let height = image.height() as usize;
    let mut new_image = GrayImage::new(image.width(), image.height());
    let image = image.as_raw();
    new_image
        .par_chunks_mut(width)
        .enumerate()
        .skip(1)
        .take(height - 2)
        .for_each(|(row, row_data)| {
            for (col, pixel) in row_data.iter_mut().enumerate().skip(1).take(width - 2) {
                *pixel = process_pixel(image, width as u32, col as u32, row as u32)
            }
        });
    new_image
}

pub fn process_pixel(raw_image: &[u8], width: u32, x: u32, y: u32) -> u8 {
    let mut sum_x = 0;
    let mut sum_y = 0;
    for i in 0..3 {
        for j in 0..3 {
            // Convolution
            // always size 1 vec
            // send pointer to top left first
            let cur_pix = raw_image[(x + 1 - i + (y + 1 - j) * width) as usize] as i32;
            sum_x += cur_pix * SOBEL_X_FILTER[(j * 3 + i) as usize];
            sum_y += cur_pix * SOBEL_Y_FILTER[(j * 3 + i) as usize];
        }
    }

    (sum_x.pow(2) + sum_y.pow(2)).isqrt().min(255) as u8
}
