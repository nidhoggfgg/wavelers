use crate::wavelet::Wavelet;

pub fn dwt2<W>(
    data: &[f64],
    size: (usize, usize),
    wavelet: W,
) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)
where
    W: Wavelet,
{
    let (rows, cols) = size;

    let mut row_transformed = vec![0.0; data.len()];
    for i in 0..rows {
        let row_start = i * cols;
        let row_end = row_start + cols;
        let (approx, detail) = wavelet.transform(&data[row_start..row_end]);
        row_transformed[row_start..row_start + cols / 2].copy_from_slice(&approx);
        row_transformed[row_start + cols / 2..row_end].copy_from_slice(&detail);
    }

    let mut col_transformed = vec![0.0; data.len()];
    for j in 0..cols {
        let mut column = vec![0.0; rows];
        for i in 0..rows {
            column[i] = row_transformed[i * cols + j];
        }
        let (approx, detail) = wavelet.transform(&column);
        for i in 0..rows / 2 {
            col_transformed[i * cols + j] = approx[i];
            col_transformed[(i + rows / 2) * cols + j] = detail[i];
        }
    }

    let half_rows = rows / 2;
    let half_cols = cols / 2;

    let mut ll = vec![0.0; half_rows * half_cols];
    let mut lh = vec![0.0; half_rows * half_cols];
    let mut hl = vec![0.0; half_rows * half_cols];
    let mut hh = vec![0.0; half_rows * half_cols];

    for i in 0..half_rows {
        for j in 0..half_cols {
            ll[i * half_cols + j] = col_transformed[i * cols + j];
            lh[i * half_cols + j] = col_transformed[i * cols + (j + half_cols)];
            hl[i * half_cols + j] = col_transformed[(i + half_rows) * cols + j];
            hh[i * half_cols + j] = col_transformed[(i + half_rows) * cols + (j + half_cols)];
        }
    }

    (ll, lh, hl, hh)
}

pub fn idwt2<W>(
    ll: &[f64],
    lh: &[f64],
    hl: &[f64],
    hh: &[f64],
    size: (usize, usize),
    wavelet: W,
) -> Vec<f64>
where
    W: Wavelet,
{
    let (rows, cols) = size;
    let half_rows = rows / 2;
    let half_cols = cols / 2;

    let mut col_transformed = vec![0.0; rows * cols];
    for i in 0..half_rows {
        for j in 0..half_cols {
            let idx = i * half_cols + j;
            col_transformed[i * cols + j] = ll[idx];
            col_transformed[i * cols + j + half_cols] = lh[idx];
            col_transformed[(i + half_rows) * cols + j] = hl[idx];
            col_transformed[(i + half_rows) * cols + j + half_cols] = hh[idx];
        }
    }

    let mut transposed_col = vec![0.0; rows * cols];
    transpose(&col_transformed, &mut transposed_col, rows, cols);

    let mut transposed_col_processed = vec![0.0; rows * cols];
    for i in 0..cols {
        let start = i * rows;
        let (approx, detail) = transposed_col[start..start + rows].split_at(half_rows);
        let reconstructed = wavelet.inverse_transform(approx, detail);
        transposed_col_processed[start..start + rows].copy_from_slice(&reconstructed);
    }

    let mut row_transformed = vec![0.0; rows * cols];
    transpose(&transposed_col_processed, &mut row_transformed, cols, rows);

    let mut data = vec![0.0; rows * cols];
    for i in 0..rows {
        let row_start = i * cols;
        let (approx, detail) = row_transformed[row_start..row_start + cols].split_at(half_cols);
        let reconstructed = wavelet.inverse_transform(approx, detail);
        data[row_start..row_start + cols].copy_from_slice(&reconstructed);
    }

    data
}

fn transpose(input: &[f64], output: &mut [f64], rows: usize, cols: usize) {
    let block_size = 16;
    for row_chunk in (0..rows).step_by(block_size) {
        for col_chunk in (0..cols).step_by(block_size) {
            for row in row_chunk..(row_chunk + block_size).min(rows) {
                for col in col_chunk..(col_chunk + block_size).min(cols) {
                    output[col * rows + row] = input[row * cols + col];
                }
            }
        }
    }
}
