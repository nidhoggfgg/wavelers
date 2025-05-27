use super::Padding;

#[derive(Debug, Default, PartialEq, Eq, Hash)]
pub struct Zero {}

impl Padding for Zero {
    fn padding(&self, data: &[f64], size: usize) -> Vec<f64> {
        let origin_size = data.len();
        assert!(size > origin_size);

        let mut result = vec![0.0; size];
        let start_index = (size - origin_size) / 2;
        let end_index = start_index + origin_size;
        result[start_index..end_index].copy_from_slice(data);

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::padding::Padding;

    use super::Zero;

    #[test]
    fn empty() {
        let pad = Zero {};
        let data = vec![];
        let result = pad.padding(&data, 4);
        assert_eq!(result, vec![0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn one() {
        let pad = Zero {};
        let data = vec![1.0];
        let result = pad.padding(&data, 4);
        assert_eq!(result, vec![0.0, 1.0, 0.0, 0.0]);
    }

    #[test]
    fn three() {
        let pad = Zero {};
        let data = vec![1.0, 2.0, 3.0];
        let result = pad.padding(&data, 4);
        assert_eq!(result, vec![1.0, 2.0, 3.0, 0.0]);
    }

    #[test]
    fn many() {
        let pad = Zero {};
        let data = vec![1.0, 2.0, 3.0];
        let result = pad.padding(&data, 10);
        assert_eq!(
            result,
            vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 0.0]
        );
    }
}
