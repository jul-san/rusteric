use linear_rustgebra::matrix::Matrix;
use linear_rustgebra::operations::arithmetic::multiply;

#[test]
fn test_multiply_2x2() {
    let m1 = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let m2 = Matrix::new(2, 2, vec![5.0, 6.0, 7.0, 8.0]);
    let result = multiply(&m1, &m2).unwrap();
    assert_eq!(result.read_matrix(), &vec![19.0, 22.0, 43.0, 50.0]);
}

#[test]
fn test_multiply_identity() {
    let m1 = Matrix::new(2, 2, vec![3.0, 7.0, -1.0, 5.0]);
    let identity = Matrix::new(2, 2, vec![1.0, 0.0, 0.0, 1.0]);
    let result = multiply(&m1, &identity).unwrap();
    assert_eq!(result.read_matrix(), m1.read_matrix());
}

#[test]
fn test_multiply_non_square() {
    // 2x3 * 3x2 = 2x2
    let m1 = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let m2 = Matrix::new(3, 2, vec![7.0, 8.0, 9.0, 10.0, 11.0, 12.0]);
    let result = multiply(&m1, &m2).unwrap();
    assert_eq!(result.read_matrix(), &vec![58.0, 64.0, 139.0, 154.0]);
}

#[test]
fn test_multiply_dimension_mismatch_returns_err() {
    let m1 = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let m2 = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    assert!(multiply(&m1, &m2).is_err());
}
