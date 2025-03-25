use criterion::{Criterion, black_box, criterion_group, criterion_main};
use wavelers::{dwt, dwt2, idwt, idwt2, wavelet::Haar};

fn haar_wavelet_benchmark(c: &mut Criterion) {
    let signal: Vec<f64> = (0..1024).map(|x| x as f64).collect();

    c.bench_function("haar_wavelet_transform", |b| {
        b.iter(|| dwt(black_box(&signal), Haar))
    });
}

fn inverse_haar_wavelet_benchmark(c: &mut Criterion) {
    let approx: Vec<f64> = (0..512).map(|x| x as f64).collect();
    let detail: Vec<f64> = (0..512).map(|x| x as f64).collect();

    c.bench_function("inverse_haar_wavelet_transform", |b| {
        b.iter(|| idwt(black_box(&approx), black_box(&detail), Haar))
    });
}

fn haar_wavelet_benchmark_2d(c: &mut Criterion) {
    let signal: Vec<f64> = (0..1024 * 1024).map(|x| x as f64).collect();

    c.bench_function("haar_wavelet_transform_2d", |b| {
        b.iter(|| dwt2(black_box(&signal), (10, 10), Haar))
    });
}

fn inverse_haar_wavelet_benchmark_2d(c: &mut Criterion) {
    let ll: Vec<f64> = (0..512).map(|x| x as f64).collect();
    let lh: Vec<f64> = (0..512).map(|x| x as f64).collect();
    let hl: Vec<f64> = (0..512).map(|x| x as f64).collect();
    let hh: Vec<f64> = (0..512).map(|x| x as f64).collect();

    c.bench_function("inverse_haar_wavelet_transform_2d", |b| {
        b.iter(|| {
            idwt2(
                black_box(&ll),
                black_box(&lh),
                black_box(&hl),
                black_box(&hh),
                (1024, 1024),
                Haar,
            )
        })
    });
}

criterion_group!(
    benches,
    haar_wavelet_benchmark,
    inverse_haar_wavelet_benchmark,
    haar_wavelet_benchmark_2d,
    inverse_haar_wavelet_benchmark_2d,
);
criterion_main!(benches);
