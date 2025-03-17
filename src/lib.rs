use std::f64::consts::SQRT_2;

pub fn haar_wavelet_transform(signal: &[f64]) -> (Vec<f64>, Vec<f64>) {
    let n = signal.len();

    assert!(n % 2 == 0, "Signal length must be even");

    let mut approx = Vec::with_capacity(n / 2);
    let mut detail = Vec::with_capacity(n / 2);

    for i in (0..n).step_by(2) {
        let a = (signal[i] + signal[i + 1]) / SQRT_2;
        let d = (signal[i] - signal[i + 1]) / SQRT_2;
        approx.push(a);
        detail.push(d);
    }

    (approx, detail)
}

pub fn inverse_haar_wavelet_transform(approx: &[f64], detail: &[f64]) -> Vec<f64> {
    let n = approx.len() * 2;
    let mut signal = vec![0.0; n];

    for i in 0..approx.len() {
        let a = approx[i] * SQRT_2;
        let d = detail[i] * SQRT_2;
        signal[2 * i] = (a + d) / 2.0;
        signal[2 * i + 1] = (a - d) / 2.0;
    }

    signal
}
