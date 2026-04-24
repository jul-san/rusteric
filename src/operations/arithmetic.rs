use crate::matrix::Matrix;
use rayon::prelude::*;

pub fn add(m1: &Matrix, m2: &Matrix) -> Result<Matrix, String>{
    // check that the dimensions match before trying to add
    if m1.get_rows() != m2.get_rows() || m1.get_cols() != m2.get_cols(){
        return Err(format!(
            "Dimension mismatch: ({}x{}) vs ({}x{})",
            m1.get_rows(), m1.get_cols(),
            m2.get_rows(), m2.get_cols()
        ));
    }

    // add using iterators (trying to get used to this lol)
    let data: Vec<f64> = m1.read_matrix()
        .iter()
        .zip(m2.read_matrix().iter())
        .map(|(a, b)| a + b)
        .collect();

    Ok(Matrix::new(m1.get_rows(), m1.get_cols(), data))
}

pub fn subtract(m1: &Matrix, m2: &Matrix) -> Result<Matrix, String>{
    if m1.get_rows() != m2.get_rows() || m1.get_cols() != m2.get_cols(){
        return Err(format!(
            "Dimension mismatch: ({}x{}) vs ({}x{})",
            m1.get_rows(), m1.get_cols(),
            m2.get_rows(), m2.get_cols()
        ));
    }

    let data: Vec<f64> = m1.read_matrix()
        .iter()
        .zip(m2.read_matrix().iter())
        .map(|(a, b)| a - b)    // replaced with '-' for subtraction
        .collect();

    Ok(Matrix::new(m1.get_rows(), m1.get_cols(), data))
}

pub fn multiply(m1: &Matrix, m2: &Matrix) -> Result<Matrix, String>{
    // the number of columns in m1 have to equal the number of rows in m2
    if m1.get_cols() != m2.get_rows(){
        return Err(format!(
            "Dimension mismatch: ({}x{}) vs ({}x{})",
            m1.get_rows(), m1.get_cols(),
            m2.get_rows(), m2.get_cols()
        ));
    }

    let rows = m1.get_rows();
    let cols = m2.get_cols();
    let inner = m1.get_cols();

    let data: Vec<f64> = (0..rows)
        .into_par_iter()
        .flat_map(|i| (0..cols).into_par_iter().map(move |j| (i, j)))
        .map(|(i, j)| {
            (0..inner).map(|k| m1.read_matrix()[i * inner + k] * m2.read_matrix()[k * cols + j]).sum()
        })
        .collect();

    Ok(Matrix::new(rows, cols, data))
}
