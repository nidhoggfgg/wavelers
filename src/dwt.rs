use crate::wavelet::Wavelet;

pub fn dwt<W>(data: &[f64], wavelet: W) -> (Vec<f64>, Vec<f64>)
where
    W: Wavelet,
{
    wavelet.transform(data)
}

pub fn idwt<W>(approx: &[f64], detail: &[f64], wavelet: W) -> Vec<f64>
where
    W: Wavelet,
{
    wavelet.inverse_transform(approx, detail)
}
