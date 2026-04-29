use linear_rustgebra::matrix::Matrix;
use linear_rustgebra::operations::transpose::transpose;

fn assert_matrix_eq(result: &[f64], expected: &[f64]) {
    assert_eq!(result.len(), expected.len(), "Matrix sizes differ");
    for (i, (r, e)) in result.iter().zip(expected.iter()).enumerate() {
        assert!(
            (r - e).abs() < 1e-9,
            "Mismatch at index {}: got {}, expected {}", i, r, e);
    }
}

#[test]
fn test_transpose_square() {
    let m = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let t = transpose(&m);
    assert_matrix_eq(t.read_matrix(), &[1.0, 3.0, 2.0, 4.0]);
}

#[test]
fn test_transpose_3x3() {
    let m = Matrix::new(3, 3, vec![
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        7.0, 8.0, 9.0,
    ]);
    let t = transpose(&m);
    assert_matrix_eq(t.read_matrix(), &[
        1.0, 4.0, 7.0,
        2.0, 5.0, 8.0,
        3.0, 6.0, 9.0,
    ]);
}

#[test]
fn test_transpose_row_vector() {
    let m = Matrix::new(1, 3, vec![1.0, 2.0, 3.0]);
    let t = transpose(&m);
    assert_eq!(t.get_rows(), 3);
    assert_eq!(t.get_cols(), 1);
    assert_matrix_eq(t.read_matrix(), &[1.0, 2.0, 3.0]);
}

#[test]
fn test_transpose_col_vector() {
    let m = Matrix::new(3, 1, vec![1.0, 2.0, 3.0]);
    let t = transpose(&m);
    assert_eq!(t.get_rows(), 1);
    assert_eq!(t.get_cols(), 3);
    assert_matrix_eq(t.read_matrix(), &[1.0, 2.0, 3.0]);
}

#[test]
fn test_transpose_2x3() {
    let m = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let t = transpose(&m);
    assert_eq!(t.get_rows(), 3);
    assert_eq!(t.get_cols(), 2);
    assert_matrix_eq(t.read_matrix(), &[1.0, 4.0, 2.0, 5.0, 3.0, 6.0]);
}

#[test]
fn test_transpose_3x2() {
    let m = Matrix::new(3, 2, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let t = transpose(&m);
    assert_eq!(t.get_rows(), 2);
    assert_eq!(t.get_cols(), 3);
    assert_matrix_eq(t.read_matrix(), &[1.0, 3.0, 5.0, 2.0, 4.0, 6.0]);
}

#[test]
fn test_transpose_twice_is_original() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let m = Matrix::new(2, 3, data.clone());
    let t = transpose(&transpose(&m));
    assert_eq!(t.get_rows(), 2);
    assert_eq!(t.get_cols(), 3);
    assert_matrix_eq(t.read_matrix(), &data);
}

#[test]
fn test_transpose_identity() {
    let m = Matrix::new(3, 3, vec![
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 0.0, 1.0,
    ]);
    let t = transpose(&m);
    assert_matrix_eq(t.read_matrix(), m.read_matrix());
}
