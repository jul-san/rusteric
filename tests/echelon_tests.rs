use linear_rustgebra::matrix::Matrix;
use linear_rustgebra::operations::echelon::{row_echelon, reduced_row_echelon};

fn assert_matrix_eq(result: &[f64], expected: &[f64]) {
    assert_eq!(result.len(), expected.len(), "Matrix sizes differ");
    for (i, (r, e)) in result.iter().zip(expected.iter()).enumerate() {
        assert!(
            (r - e).abs() < 1e-9,
            "Mismatch at index {}: got {}, expected {}", i, r, e);
    }
}

#[test]
fn test_row_echelon_2x2() {
    let mut m = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    row_echelon(&mut m);
    assert_matrix_eq(m.read_matrix(), &[1.0, 2.0, 0.0, 1.0]);
}

#[test]
fn test_row_echelon_3x3_rank_deficient() {
    let mut m = Matrix::new(3, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
    row_echelon(&mut m);
    assert_matrix_eq(m.read_matrix(), &[1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 0.0, 0.0, 0.0]);
}

#[test]
fn test_row_echelon_leading_zero_forces_swap() {
    let mut m = Matrix::new(3, 3, vec![0.0, 1.0, 2.0, 1.0, 2.0, 3.0, 3.0, 4.0, 5.0]);
    row_echelon(&mut m);
    assert_matrix_eq(m.read_matrix(), &[1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 0.0, 0.0, 0.0]);
}

#[test]
fn test_row_echelon_singular_matrix() {
    let mut m = Matrix::new(2, 2, vec![1.0, 2.0, 2.0, 4.0]);
    row_echelon(&mut m);
    assert_matrix_eq(m.read_matrix(), &[1.0, 2.0, 0.0, 0.0]);
}

#[test]
fn test_row_echelon_non_square() {
    let mut m = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    row_echelon(&mut m);
    assert_matrix_eq(m.read_matrix(), &[1.0, 2.0, 3.0, 0.0, 1.0, 2.0]);
}

#[test]
fn test_row_echelon_non_unit_pivot() {
    let mut m = Matrix::new(2, 2, vec![2.0, 6.0, 4.0, 10.0]);
    row_echelon(&mut m);
    assert_matrix_eq(m.read_matrix(), &[1.0, 3.0, 0.0, 1.0]);
}

#[test]
fn test_reduced_row_echelon_2x2() {
    let mut m = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    reduced_row_echelon(&mut m);
    assert_matrix_eq(m.read_matrix(), &[1.0, 0.0, 0.0, 1.0]);
}

#[test]
fn test_reduced_row_echelon_3x3_full_rank() {
    let mut m = Matrix::new(3, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 10.0]);
    reduced_row_echelon(&mut m);
    assert_matrix_eq(m.read_matrix(), &[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
}

#[test]
fn test_reduced_row_echelon_singular() {
    let mut m = Matrix::new(2, 2, vec![1.0, 2.0, 2.0, 4.0]);
    reduced_row_echelon(&mut m);
    assert_matrix_eq(m.read_matrix(), &[1.0, 2.0, 0.0, 0.0]);
}

#[test]
fn test_reduced_row_echelon_augmented_2x3() {
    let mut m = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    reduced_row_echelon(&mut m);
    assert_matrix_eq(m.read_matrix(), &[1.0, 0.0, -1.0, 0.0, 1.0, 2.0]);
}
