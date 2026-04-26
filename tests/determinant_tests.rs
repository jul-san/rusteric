use linear_rustgebra::matrix::Matrix;
use linear_rustgebra::operations::determinant::determinant;

fn assert_det_eq(result: f64, expected: f64) {
    assert!(
        (result - expected).abs() < 1e-9,
        "got {}, expected {}", result, expected
    );
}

#[test]
fn test_determinant_2x2() {
    // det([[1,2],[3,4]]) = 1*4 - 2*3 = -2
    let m = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    assert_det_eq(determinant(&m).unwrap(), -2.0);
}

#[test]
fn test_determinant_3x3() {
    // det([[1,2,3],[4,5,6],[7,8,10]]) = -3
    let m = Matrix::new(3, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 10.0]);
    assert_det_eq(determinant(&m).unwrap(), -3.0);
}

#[test]
fn test_determinant_identity_2x2() {
    let m = Matrix::new(2, 2, vec![1.0, 0.0, 0.0, 1.0]);
    assert_det_eq(determinant(&m).unwrap(), 1.0);
}

#[test]
fn test_determinant_identity_3x3() {
    let m = Matrix::new(3, 3, vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
    assert_det_eq(determinant(&m).unwrap(), 1.0);
}

#[test]
fn test_determinant_singular_is_zero() {
    // rows are linearly dependent
    let m = Matrix::new(3, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
    assert_det_eq(determinant(&m).unwrap(), 0.0);
}

#[test]
fn test_determinant_zero_matrix_is_zero() {
    let m = Matrix::new(2, 2, vec![0.0, 0.0, 0.0, 0.0]);
    assert_det_eq(determinant(&m).unwrap(), 0.0);
}

#[test]
fn test_determinant_row_swap_changes_sign() {
    // swapping rows of a matrix negates the determinant
    let m1 = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let m2 = Matrix::new(2, 2, vec![3.0, 4.0, 1.0, 2.0]);
    let d1 = determinant(&m1).unwrap();
    let d2 = determinant(&m2).unwrap();
    assert_det_eq(d1, -d2);
}

#[test]
fn test_determinant_does_not_mutate_matrix() {
    let m = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let original = m.read_matrix().clone();
    determinant(&m).unwrap();
    assert_eq!(m.read_matrix(), &original);
}

#[test]
fn test_determinant_non_square_returns_err() {
    let m = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    assert!(determinant(&m).is_err());
}
