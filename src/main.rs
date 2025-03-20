use wavelers::{dwt, idwt, wavelet::Haar};

fn main() {
    let signal = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];

    let (approx, detail) = dwt(&signal, Haar);
    println!("Approximation coefficients: {:?}", approx);
    println!("Detail coefficients: {:?}", detail);

    let reconstructed = idwt(&approx, &detail, Haar);
    println!("Reconstructed signal: {:?}", reconstructed);
}
