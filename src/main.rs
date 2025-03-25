use wavelers::{dwt2, idwt2, wavelet::Haar};

fn main() {
    let data = vec![
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
    ];
    let size = (4, 4);

    let (ll, lh, hl, hh) = dwt2(&data, size, Haar);

    println!("LL: {:?}", ll);
    println!("LH: {:?}", lh);
    println!("HL: {:?}", hl);
    println!("HH: {:?}", hh);

    let r = idwt2(&ll, &lh, &hl, &hh, size, Haar);

    println!("idwt2 result: {:?}", r);
}
