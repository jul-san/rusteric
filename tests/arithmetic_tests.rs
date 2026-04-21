use linear_rustgebra::matrix::Matrix;
use linear_rustgebra::operations::arithmetic::add;

#[test]
fn test_add_2x2() {
    let m1 = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let m2 = Matrix::new(2, 2, vec![5.0, 6.0, 7.0, 8.0]);
    let result = add(&m1, &m2).unwrap();
    assert_eq!(result.read_matrix(), &vec![6.0, 8.0, 10.0, 12.0]);
}

#[test]
fn test_add_identity_zero() {
    let m1 = Matrix::new(2, 2, vec![3.0, 7.0, -1.0, 5.0]);
    let zero = Matrix::new(2, 2, vec![0.0, 0.0, 0.0, 0.0]);
    let result = add(&m1, &zero).unwrap();
    assert_eq!(result.read_matrix(), m1.read_matrix());
}

#[test]
fn test_add_negative_values() {
    let m1 = Matrix::new(2, 2, vec![-1.0, -2.0, -3.0, -4.0]);
    let m2 = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let result = add(&m1, &m2).unwrap();
    assert_eq!(result.read_matrix(), &vec![0.0, 0.0, 0.0, 0.0]);
}

#[test]
fn test_add_non_square() {
    let m1 = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let m2 = Matrix::new(2, 3, vec![6.0, 5.0, 4.0, 3.0, 2.0, 1.0]);
    let result = add(&m1, &m2).unwrap();
    assert_eq!(result.read_matrix(), &vec![7.0, 7.0, 7.0, 7.0, 7.0, 7.0]);
}

#[test]
fn test_add_dimension_mismatch_returns_err() {
    let m1 = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let m2 = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    assert!(add(&m1, &m2).is_err());
}
