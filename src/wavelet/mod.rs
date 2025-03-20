mod haar;

pub use haar::Haar;

pub trait Wavelet {
    fn transform(&self, data: &[f64]) -> (Vec<f64>, Vec<f64>);
    fn inverse_transform(&self, approx: &[f64], detail: &[f64]) -> Vec<f64>;
}
