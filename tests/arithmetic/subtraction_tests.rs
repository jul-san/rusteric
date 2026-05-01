use linear_rustgebra::matrix::Matrix;
use linear_rustgebra::operations::arithmetic::subtract;

#[test]
fn test_subtract_2x2() {
    let m1 = Matrix::new(2, 2, vec![5.0, 6.0, 7.0, 8.0]);
    let m2 = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let result = subtract(&m1, &m2).unwrap();
    assert_eq!(result.read_matrix(), &vec![4.0, 4.0, 4.0, 4.0]);
}

#[test]
fn test_subtract_self_is_zero() {
    let m1 = Matrix::new(2, 2, vec![3.0, 7.0, -1.0, 5.0]);
    let m2 = Matrix::new(2, 2, vec![3.0, 7.0, -1.0, 5.0]);
    let result = subtract(&m1, &m2).unwrap();
    assert_eq!(result.read_matrix(), &vec![0.0, 0.0, 0.0, 0.0]);
}

#[test]
fn test_subtract_zero_matrix() {
    let m1 = Matrix::new(2, 2, vec![3.0, 7.0, -1.0, 5.0]);
    let zero = Matrix::new(2, 2, vec![0.0, 0.0, 0.0, 0.0]);
    let result = subtract(&m1, &zero).unwrap();
    assert_eq!(result.read_matrix(), m1.read_matrix());
}

#[test]
fn test_subtract_non_square() {
    let m1 = Matrix::new(2, 3, vec![6.0, 5.0, 4.0, 3.0, 2.0, 1.0]);
    let m2 = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let result = subtract(&m1, &m2).unwrap();
    assert_eq!(result.read_matrix(), &vec![5.0, 3.0, 1.0, -1.0, -3.0, -5.0]);
}

#[test]
fn test_subtract_dimension_mismatch_returns_err() {
    let m1 = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let m2 = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    assert!(subtract(&m1, &m2).is_err());
}
