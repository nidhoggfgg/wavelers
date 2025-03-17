use criterion::{Criterion, black_box, criterion_group, criterion_main};
use dwt_rs::{haar_wavelet_transform, inverse_haar_wavelet_transform};

fn haar_wavelet_benchmark(c: &mut Criterion) {
    let signal: Vec<f64> = (0..1024).map(|x| x as f64).collect();

    c.bench_function("haar_wavelet_transform", |b| {
        b.iter(|| haar_wavelet_transform(black_box(&signal)))
    });
}

fn inverse_haar_wavelet_benchmark(c: &mut Criterion) {
    let approx: Vec<f64> = (0..512).map(|x| x as f64).collect();
    let detail: Vec<f64> = (0..512).map(|x| x as f64).collect();

    c.bench_function("inverse_haar_wavelet_transform", |b| {
        b.iter(|| inverse_haar_wavelet_transform(black_box(&approx), black_box(&detail)))
    });
}

criterion_group!(
    benches,
    haar_wavelet_benchmark,
    inverse_haar_wavelet_benchmark
);
criterion_main!(benches);
