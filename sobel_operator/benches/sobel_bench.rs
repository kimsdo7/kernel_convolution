use criterion::{Criterion, criterion_group, criterion_main};
use sobel_operator::{process_image, process_pixel, sobel_operation};

fn sobel_bench(c: &mut Criterion) {
    c.bench_function("sobel bench", |b| b.iter(|| sobel_operation()));
}

criterion_group!(benches, sobel_bench);
criterion_main!(benches);
