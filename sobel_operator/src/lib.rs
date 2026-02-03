use image::{GrayImage, Luma};
use rayon::prelude::*;
use std::{env, fs};

const IMAGE_PATH: &str = "images";
const PROCESSED_PATH: &str = "processed_images";

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
    new_image
        .par_iter_mut()
        .enumerate()
        .for_each(|(index, pixel)| {
            if index > width
                && index < width * (height - 1)
                && index % width != 0
                && index % width != width - 1
            {
                *pixel = process_pixel(image, (index % width) as u32, (index / width) as u32)
            }
        });
    new_image
}

pub fn process_pixel(image: &GrayImage, x: u32, y: u32) -> u8 {
    let raw_image = image.as_raw();
    let width = image.width() as usize;
    let x = x as usize;
    let y = y as usize;
    let first_row = (y - 1) * width;
    let second_row = y * width;
    let third_row = (y + 1) * width;
    let first_col = x - 1;
    let second_col = x;
    let third_col = x + 1;
    let top_left = raw_image[first_row + first_col] as i32;
    let top_mid = raw_image[first_row + second_col] as i32;
    let top_right = raw_image[first_row + third_col] as i32;
    let middle_left = raw_image[second_row + first_col] as i32;
    // middle_middle is always 0;
    let middle_right = raw_image[second_row + third_col] as i32;
    let bot_left = raw_image[third_row + first_col] as i32;
    let bot_mid = raw_image[third_row + second_col] as i32;
    let bot_right = raw_image[third_row + third_col] as i32;

    //
    // const SOBEL_X_FILTER: [i32; 9] = [-1, 0, 1, -2, 0, 2, -1, 0, 1];
    // const SOBEL_Y_FILTER: [i32; 9] = [-1, -2, -1, 0, 0, 0, 1, 2, 1];
    let sum_x = top_right - top_left + 2 * middle_right - 2 * middle_left + bot_right - bot_left;
    let sum_y = bot_left + 2 * bot_mid + bot_right - top_left - 2 * top_mid - top_right;

    //min max can be better?
    // Luma::from([(sum_x.pow(2) + sum_y.pow(2)).isqrt().min(255) as u8])
    (sum_x.pow(2) + sum_y.pow(2)).isqrt().min(255) as u8
}
