use linear_rustgebra::matrix::Matrix;
use linear_rustgebra::operations::echelon::{row_echelon, reduced_row_echelon};

fn assert_matrix_eq(result: &[f64], expected: &[f64]) {
    assert_eq!(result.len(), expected.len(), "Matrix sizes differ");
    for (i, (r, e)) in result.iter().zip(expected.iter()).enumerate() {
        assert!(
            (r - e).abs() < 1e-9,
            "Mismatch at index {}: got {}, expected {}", i, r, e );
    }
}

// normalize_row and eliminate_below on a 2x2
#[test]
fn test_row_echelon_2x2() {
    let mut m = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    row_echelon(&mut m);
    // row0 normalized: [1,2]; row1 eliminated: [0,-2] -> normalized: [0,1]
    assert_matrix_eq(m.read_matrix(), &[1.0, 2.0, 0.0, 1.0]);
}

// full pipeline on a 3x3 with a zero row produced during elimination
#[test]
fn test_row_echelon_3x3_rank_deficient() {
    let mut m = Matrix::new(3, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
    row_echelon(&mut m);
    // Third row becomes zero after elimination (rank 2 matrix).
    assert_matrix_eq(m.read_matrix(), &[1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 0.0, 0.0, 0.0]);
}

// find_pivot_row and swap_rows: leading zero forces a row swap.
#[test]
fn test_row_echelon_leading_zero_forces_swap() {
    let mut m = Matrix::new(3, 3, vec![0.0, 1.0, 2.0, 1.0, 2.0, 3.0, 3.0, 4.0, 5.0]);
    row_echelon(&mut m);
    // Row 0 has a zero in col 0, so rows 0 and 1 get swapped before normalizing.
    assert_matrix_eq(m.read_matrix(), &[1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 0.0, 0.0, 0.0]);
}

// find_pivot_row returning None: entire column is zero so pivot_row stays put
#[test]
fn test_row_echelon_singular_matrix() {
    let mut m = Matrix::new(2, 2, vec![1.0, 2.0, 2.0, 4.0]);
    row_echelon(&mut m);
    // After eliminating below row0, row1 becomes [0,0].
    assert_matrix_eq(m.read_matrix(), &[1.0, 2.0, 0.0, 0.0]);
}

// row_echelon on a non-square (2x3) matrix.
#[test]
fn test_row_echelon_non_square() {
    let mut m = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    row_echelon(&mut m);
    assert_matrix_eq(m.read_matrix(), &[1.0, 2.0, 3.0, 0.0, 1.0, 2.0]);
}

// tests eliminate_below with a non-unit leading coefficient.
#[test]
fn test_row_echelon_non_unit_pivot() {
    let mut m = Matrix::new(2, 2, vec![2.0, 6.0, 4.0, 10.0]);
    row_echelon(&mut m);
    // row0 normalized: [1,3]; row1: [4-4,10-12]=[0,-2] -> normalized: [0,1]
    assert_matrix_eq(m.read_matrix(), &[1.0, 3.0, 0.0, 1.0]);
}

// eliminate_above on a 2x2
#[test]
fn test_reduced_row_echelon_2x2() {
    let mut m = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    reduced_row_echelon(&mut m);
    assert_matrix_eq(m.read_matrix(), &[1.0, 0.0, 0.0, 1.0]);
}

// eliminate_above on a 3x3
#[test]
fn test_reduced_row_echelon_3x3_full_rank() {
    let mut m = Matrix::new(3, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 10.0]);
    reduced_row_echelon(&mut m);
    assert_matrix_eq(m.read_matrix(), &[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
}

// tests that reduced row echelon on a singular matrix stops back-substituting at the zero row
#[test]
fn test_reduced_row_echelon_singular() {
    let mut m = Matrix::new(2, 2, vec![1.0, 2.0, 2.0, 4.0]);
    reduced_row_echelon(&mut m);
    // Zero row has no pivot, so the free variable column (col 1) is untouched in row 0.
    assert_matrix_eq(m.read_matrix(), &[1.0, 2.0, 0.0, 0.0]);
}

// reduced row echelon on an augmented (2x3) matrix — the third column carries the RHS.
#[test]
fn test_reduced_row_echelon_augmented_2x3() {
    let mut m = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    reduced_row_echelon(&mut m);
    // Represents x + 2y = 3, 4x + 5y = 6 -> x = -1, y = 2
    assert_matrix_eq(m.read_matrix(), &[1.0, 0.0, -1.0, 0.0, 1.0, 2.0]);
}
