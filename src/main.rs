use dwt_rs::{haar_wavelet_transform, inverse_haar_wavelet_transform};

fn main() {
    // 输入信号（长度必须是 2 的幂）
    let signal = vec![1.0, 3.0, 2.0, 0.0, 1.0, 4.0, 100.0, 0.0];

    // 执行 Haar 小波变换
    let (approx, detail) = haar_wavelet_transform(&signal);
    println!("Approximation coefficients: {:?}", approx);
    println!("Detail coefficients: {:?}", detail);

    // 执行逆 Haar 小波变换
    let reconstructed = inverse_haar_wavelet_transform(&approx, &detail);
    println!("Reconstructed signal: {:?}", reconstructed);
}
