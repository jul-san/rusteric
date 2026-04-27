pub fn swap_rows(data: &mut [f64], cols: usize, row_a: usize, row_b: usize) {
    for col in 0..cols {
        data.swap(row_a * cols + col, row_b * cols + col);
    }
}

pub fn scale_row(data: &mut [f64], cols: usize, row: usize, scalar: f64) {
    for col in 0..cols {
        data[row * cols + col] *= scalar;
    }
}

// row replacement
pub fn add_scaled_row(data: &mut [f64], cols: usize, target: usize, source: usize, factor: f64) {
    for col in 0..cols {
        data[target * cols + col] += factor * data[source * cols + col];
    }
}
