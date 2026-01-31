use image::{GrayImage, Luma};
use std::{env, ffi::OsString, fs};

const IMAGE_PATH: &str = "images";
const PROCESSED_PATH: &str = "processed_images";

const SOBEL_X_FILTER: [[i32; 3]; 3] = [[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]];
const SOBEL_Y_FILTER: [[i32; 3]; 3] = [[-1, -2, -1], [0, 0, 0], [1, 2, 1]];

pub fn sobel_operation() {
    let mut image_dir = env::current_dir().unwrap();
    let mut processed_dir = env::current_dir().unwrap();
    image_dir.push(IMAGE_PATH);
    processed_dir.push(PROCESSED_PATH);
    let images: Vec<(OsString, GrayImage)> = fs::read_dir(image_dir)
        .unwrap()
        .map(|file| {
            let file = file.unwrap();
            (
                file.file_name(),
                image::open(file.path()).unwrap().into_luma8(),
            )
        })
        .collect();
    for image in images {
        let new_image = process_image(&image.1);
        let mut new_path = processed_dir.clone();
        new_path.push(&image.0);
        new_image.save(&new_path).unwrap();
    }
}

pub fn process_image(image: &GrayImage) -> GrayImage {
    // left top shifted?
    let mut new_image = GrayImage::new(image.width(), image.height());
    for i in 1..(image.width() - 1) {
        for j in 1..(image.height() - 1) {
            new_image.put_pixel(i, j, process_pixel(image, i, j));
        }
    }
    new_image
}

pub fn process_pixel(image: &GrayImage, x: u32, y: u32) -> Luma<u8> {
    let mut sum_x = 0;
    let mut sum_y = 0;
    for i in 0..3 {
        for j in 0..3 {
            // Convolution
            // always size 1 vec
            // send pointer to top left first
            let cur_pix = *image.get_pixel(x + 1 - i, y + 1 - j).0.first().unwrap() as i32;
            sum_x += cur_pix * SOBEL_X_FILTER[j as usize][i as usize];
            sum_y += cur_pix * SOBEL_Y_FILTER[j as usize][i as usize];
        }
    }
    //min max can be better?
    Luma::from([(sum_x.pow(2) + sum_y.pow(2)).isqrt().min(255) as u8])
}
