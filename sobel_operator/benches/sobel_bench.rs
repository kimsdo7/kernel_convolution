use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use sobel_operator::{process_image, process_pixel, sobel_operation};

fn sobel_bench(c: &mut Criterion) {
    c.bench_function("Sobel Bench", |b| b.iter(sobel_operation));
}

fn image_bench(c: &mut Criterion) {
    let img = image::open("images/monet.jpeg").unwrap().into_luma8();
    c.bench_function("Image Bench", |b| {
        b.iter(|| black_box(process_image(black_box(&img))))
    });
}

fn pixel_bench(c: &mut Criterion) {
    let img = image::open("images/monet.jpeg").unwrap().into_luma8();
    c.bench_function("Pixel Bench", |b| {
        b.iter(|| {
            black_box(process_pixel(
                black_box(&img),
                black_box(100),
                black_box(100),
            ))
        })
    });
}

criterion_group!(benches, sobel_bench, image_bench, pixel_bench);
criterion_main!(benches);
