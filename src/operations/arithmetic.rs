use crate::matrix::Matrix;

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
