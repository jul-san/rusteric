use linear_rustgebra::matrix::Matrix;
use linear_rustgebra::operations::inverse::inverse;

fn assert_matrix_eq(result: &[f64], expected: &[f64]) {
    assert_eq!(result.len(), expected.len(), "Matrix sizes differ");
    for (i, (r, e)) in result.iter().zip(expected.iter()).enumerate() {
        assert!(
            (r - e).abs() < 1e-9,
            "Mismatch at index {}: got {}, expected {}", i, r, e);
    }
}

#[test]
fn test_inverse_2x2() {
    let m = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let inv = inverse(&m).unwrap();
    assert_matrix_eq(inv.read_matrix(), &[-2.0, 1.0, 1.5, -0.5]);
}

#[test]
fn test_inverse_identity_2x2() {
    let m = Matrix::new(2, 2, vec![1.0, 0.0, 0.0, 1.0]);
    let inv = inverse(&m).unwrap();
    assert_matrix_eq(inv.read_matrix(), &[1.0, 0.0, 0.0, 1.0]);
}

#[test]
fn test_inverse_identity_3x3() {
    let m = Matrix::new(3, 3, vec![
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 0.0, 1.0,
    ]);
    let inv = inverse(&m).unwrap();
    assert_matrix_eq(inv.read_matrix(), &[
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 0.0, 1.0,
    ]);
}

#[test]
fn test_inverse_3x3() {
    let m = Matrix::new(3, 3, vec![
        2.0, 1.0, 0.0,
        1.0, 3.0, 1.0,
        0.0, 1.0, 2.0,
    ]);
    let inv = inverse(&m).unwrap();
    let result = inv.read_matrix();
    let expected = [
         5.0/8.0, -2.0/8.0,  1.0/8.0,
        -2.0/8.0,  4.0/8.0, -2.0/8.0,
         1.0/8.0, -2.0/8.0,  5.0/8.0,
    ];
    assert_matrix_eq(result, &expected);
}

#[test]
fn test_inverse_singular_returns_err() {
    let m = Matrix::new(2, 2, vec![1.0, 2.0, 2.0, 4.0]);
    let result = inverse(&m);
    assert!(result.is_err());
}

#[test]
fn test_inverse_non_square_returns_err() {
    let m = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let result = inverse(&m);
    assert!(result.is_err());
}

#[test]
fn test_inverse_singular_3x3_returns_err() {
    let m = Matrix::new(3, 3, vec![
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        7.0, 8.0, 9.0,
    ]);
    let result = inverse(&m);
    assert!(result.is_err());
}

#[test]
fn test_inverse_multiplied_by_original_is_identity() {
    let m = Matrix::new(3, 3, vec![
        1.0, 2.0, 3.0,
        0.0, 1.0, 4.0,
        5.0, 6.0, 0.0,
    ]);
    let inv = inverse(&m).unwrap();
    let n = 3;
    let a = m.read_matrix();
    let b = inv.read_matrix();
    for row in 0..n {
        for col in 0..n {
            let dot: f64 = (0..n).map(|k| a[row * n + k] * b[k * n + col]).sum();
            let expected = if row == col { 1.0 } else { 0.0 };
            assert!((dot - expected).abs() < 1e-9,
                "Product[{},{}] = {}, expected {}", row, col, dot, expected);
        }
    }
}
