use crate::matrix::Matrix;

pub fn determinant(matrix: &Matrix) -> Result<f64, String> {
    let rows = matrix.get_rows();
    let cols = matrix.get_cols();

    if rows != cols {
        return Err(format!(
            "Determinant requires a square matrix, got {}x{}",
            rows, cols
        ));
    }

    let n = rows;
    let mut data = matrix.read_matrix().clone();
    let mut sign = 1.0_f64;

    for pivot in 0..n {
        let pivot_row = (pivot..n).find(|&r| data[r * n + pivot] != 0.0);

        let pivot_row = match pivot_row {
            Some(r) => r,
            None => return Ok(0.0),
        };

        if pivot_row != pivot {
            for col in 0..n {
                data.swap(pivot * n + col, pivot_row * n + col);
            }
            sign *= -1.0;
        }

        let pivot_val = data[pivot * n + pivot];

        for row in (pivot + 1)..n {
            let factor = data[row * n + pivot] / pivot_val;
            for col in pivot..n {
                data[row * n + col] -= factor * data[pivot * n + col];
            }
        }
    }

    let diagonal_product: f64 = (0..n).map(|i| data[i * n + i]).product();

    Ok(sign * diagonal_product)
}
