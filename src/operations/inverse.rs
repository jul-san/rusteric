use crate::matrix::Matrix;
use crate::operations::echelon::reduced_row_echelon;

pub fn inverse(matrix: &Matrix) -> Result<Matrix, String> {
    let rows = matrix.get_rows();
    let cols = matrix.get_cols();

    if rows != cols {
        return Err(format!(
            "Inverse requires a square matrix, got {}x{}",
            rows, cols
        ));
    }

    let n = rows;

    // build augmented [A | I] with n rows and 2n cols
    let mut augmented_data = vec![0.0_f64; n * 2 * n];
    for row in 0..n {
        for col in 0..n {
            augmented_data[row * 2 * n + col] = matrix.read_matrix()[row * n + col];
        }
        // identity block on the right
        augmented_data[row * 2 * n + (n + row)] = 1.0;
    }

    let mut augmented = Matrix::new(n, 2 * n, augmented_data);
    reduced_row_echelon(&mut augmented);

    // if any diagonal on the left block is not 1, the matrix is singular
    for i in 0..n {
        if (augmented.read_matrix()[i * 2 * n + i] - 1.0).abs() > 1e-9 {
            return Err("Matrix is singular and cannot be inverted".to_string());
        }
    }

    // extract the right n×n block as the inverse
    let mut inv_data = vec![0.0_f64; n * n];
    for row in 0..n {
        for col in 0..n {
            inv_data[row * n + col] = augmented.read_matrix()[row * 2 * n + (n + col)];
        }
    }

    Ok(Matrix::new(n, n, inv_data))
}
