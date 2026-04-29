use crate::matrix::Matrix;

pub fn transpose(matrix: &Matrix) -> Matrix {
    let rows = matrix.get_rows();
    let cols = matrix.get_cols();
    let data = matrix.read_matrix();

    let mut result = vec![0.0_f64; rows * cols];

    for row in 0..rows {
        for col in 0..cols {
            result[col * rows + row] = data[row * cols + col];
        }
    }

    Matrix::new(cols, rows, result)
}
