mod zero;

pub use zero::Zero;

pub trait Padding {
    fn padding(&self, data: &[f64], size: usize) -> Vec<f64>;
}
