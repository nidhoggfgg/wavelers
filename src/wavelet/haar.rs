use std::f64::consts::SQRT_2;

use super::Wavelet;

#[derive(Debug)]
pub struct Haar;

impl Wavelet for Haar {
    fn transform(&self, data: &[f64]) -> (Vec<f64>, Vec<f64>) {
        let n = data.len();

        let mut approx = Vec::with_capacity(n / 2);
        let mut detail = Vec::with_capacity(n / 2);

        let sqrt1 = 1.0 / SQRT_2;
        for i in (0..n).step_by(2) {
            let a = (data[i] + data[i + 1]) * sqrt1;
            let d = (data[i] - data[i + 1]) * sqrt1;
            approx.push(a);
            detail.push(d);
        }

        (approx, detail)
    }

    fn inverse_transform(&self, approx: &[f64], detail: &[f64]) -> Vec<f64> {
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
}
