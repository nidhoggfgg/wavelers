use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn dwt2(
    data: &[f64],
    width: usize,
    height: usize,
    ll: &mut [f64],
    lh: &mut [f64],
    hl: &mut [f64],
    hh: &mut [f64],
    wave: &str,
) {
    let wave = match wave.to_lowercase().as_str() {
        "haar" => wavelers::wavelet::Haar,
        _ => wavelers::wavelet::Haar,
    };
    let (rll, rlh, rhl, rhh) = wavelers::dwt2(data, (width, height), wave);
    ll.copy_from_slice(&rll);
    lh.copy_from_slice(&rlh);
    hl.copy_from_slice(&rhl);
    hh.copy_from_slice(&rhh);
}

#[wasm_bindgen]
pub fn idwt2(
    ll: &[f64],
    lh: &[f64],
    hl: &[f64],
    hh: &[f64],
    width: usize,
    height: usize,
    data: &mut [f64],
    wave: &str,
) {
    let wave = match wave.to_lowercase().as_str() {
        "haar" => wavelers::wavelet::Haar,
        _ => wavelers::wavelet::Haar,
    };
    let result = wavelers::idwt2(ll, lh, hl, hh, (width, height), wave);
    data.copy_from_slice(&result);
}
