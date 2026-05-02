use linear_rustgebra::matrix::Matrix;
use linear_rustgebra::operations::arithmetic::{scalar_multiply, scalar_add};

#[test]
fn test_scalar_multiply() {
    let m = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let result = scalar_multiply(&m, 3.0);
    assert_eq!(result.read_matrix(), &vec![3.0, 6.0, 9.0, 12.0]);
}

#[test]
fn test_scalar_multiply_by_zero() {
    let m = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let result = scalar_multiply(&m, 0.0);
    assert_eq!(result.read_matrix(), &vec![0.0, 0.0, 0.0, 0.0]);
}

#[test]
fn test_scalar_multiply_by_negative() {
    let m = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let result = scalar_multiply(&m, -1.0);
    assert_eq!(result.read_matrix(), &vec![-1.0, -2.0, -3.0, -4.0]);
}

#[test]
fn test_scalar_add() {
    let m = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let result = scalar_add(&m, 10.0);
    assert_eq!(result.read_matrix(), &vec![11.0, 12.0, 13.0, 14.0]);
}

#[test]
fn test_scalar_add_negative() {
    let m = Matrix::new(2, 2, vec![5.0, 10.0, 15.0, 20.0]);
    let result = scalar_add(&m, -5.0);
    assert_eq!(result.read_matrix(), &vec![0.0, 5.0, 10.0, 15.0]);
}
