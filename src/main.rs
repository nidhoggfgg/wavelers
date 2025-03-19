use wavelers::{haar_wavelet_transform, inverse_haar_wavelet_transform};

fn main() {
    let signal = vec![1.0, 3.0, 2.0, 0.0, 1.0, 4.0];

    let (approx, detail) = haar_wavelet_transform(&signal);
    println!("Approximation coefficients: {:?}", approx);
    println!("Detail coefficients: {:?}", detail);

    let reconstructed = inverse_haar_wavelet_transform(&approx, &detail);
    println!("Reconstructed signal: {:?}", reconstructed);
}
