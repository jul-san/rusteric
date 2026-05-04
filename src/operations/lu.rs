use crate::matrix::Matrix;

pub struct LUDecomposition {
    pub l: Matrix,
    pub u: Matrix,
    pub p: Matrix,
}

pub fn lu(matrix: &Matrix) -> Result<LUDecomposition, String> {
    let rows = matrix.get_rows();
    let cols = matrix.get_cols();

    if rows != cols {
        return Err(format!(
            "LU decomposition requires a square matrix, got {}x{}",
            rows, cols
        ));
    }

    let n = rows;
    let mut u_data = matrix.read_matrix().clone();
    let mut l_data = vec![0.0_f64; n * n];
    let mut p_data = vec![0.0_f64; n * n];

    // L starts as identity, P starts as identity
    for i in 0..n {
        l_data[i * n + i] = 1.0;
        p_data[i * n + i] = 1.0;
    }

    for k in 0..n {
        // Partial pivoting: find row with max absolute value in column k
        let pivot_row = (k..n)
            .max_by(|&a, &b| {
                u_data[a * n + k]
                    .abs()
                    .partial_cmp(&u_data[b * n + k].abs())
                    .unwrap()
            })
            .unwrap();

        if u_data[pivot_row * n + k].abs() < 1e-12 {
            return Err("Matrix is singular, LU decomposition failed".to_string());
        }

        if pivot_row != k {
            // Swap rows in U
            for col in 0..n {
                u_data.swap(k * n + col, pivot_row * n + col);
            }
            // Swap rows in P
            for col in 0..n {
                p_data.swap(k * n + col, pivot_row * n + col);
            }
            // Swap already-computed multipliers in L (only columns 0..k are filled)
            for col in 0..k {
                l_data.swap(k * n + col, pivot_row * n + col);
            }
        }

        // Compute multipliers and eliminate below the pivot
        for i in (k + 1)..n {
            let factor = u_data[i * n + k] / u_data[k * n + k];
            l_data[i * n + k] = factor;
            for j in k..n {
                u_data[i * n + j] -= factor * u_data[k * n + j];
            }
        }
    }

    Ok(LUDecomposition {
        l: Matrix::new(n, n, l_data),
        u: Matrix::new(n, n, u_data),
        p: Matrix::new(n, n, p_data),
    })
}
