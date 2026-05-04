use linear_rustgebra::matrix::Matrix;
use linear_rustgebra::operations::lu::lu;

fn assert_matrix_eq(result: &[f64], expected: &[f64]) {
    assert_eq!(result.len(), expected.len(), "Matrix sizes differ");
    for (i, (r, e)) in result.iter().zip(expected.iter()).enumerate() {
        assert!(
            (r - e).abs() < 1e-9,
            "Mismatch at index {}: got {}, expected {}",
            i, r, e
        );
    }
}

// Multiply two n×n matrices stored as flat Vec<f64>
fn mat_mul(a: &[f64], b: &[f64], n: usize) -> Vec<f64> {
    let mut c = vec![0.0_f64; n * n];
    for i in 0..n {
        for j in 0..n {
            c[i * n + j] = (0..n).map(|k| a[i * n + k] * b[k * n + j]).sum();
        }
    }
    c
}

// Verify the core property: P * A = L * U
fn assert_pa_eq_lu(a: &Matrix, n: usize) {
    let result = lu(a).unwrap();
    let pa = mat_mul(result.p.read_matrix(), a.read_matrix(), n);
    let lu_prod = mat_mul(result.l.read_matrix(), result.u.read_matrix(), n);
    assert_matrix_eq(&pa, &lu_prod);
}

#[test]
fn test_lu_2x2_pa_eq_lu() {
    let m = Matrix::new(2, 2, vec![2.0, 1.0, 1.0, 3.0]);
    assert_pa_eq_lu(&m, 2);
}

#[test]
fn test_lu_3x3_pa_eq_lu() {
    let m = Matrix::new(3, 3, vec![
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        7.0, 8.0, 10.0,
    ]);
    assert_pa_eq_lu(&m, 3);
}

#[test]
fn test_lu_identity_2x2() {
    let m = Matrix::new(2, 2, vec![1.0, 0.0, 0.0, 1.0]);
    let result = lu(&m).unwrap();
    // L and U should both equal identity
    assert_matrix_eq(result.l.read_matrix(), &[1.0, 0.0, 0.0, 1.0]);
    assert_matrix_eq(result.u.read_matrix(), &[1.0, 0.0, 0.0, 1.0]);
}

#[test]
fn test_lu_l_is_lower_triangular() {
    let m = Matrix::new(3, 3, vec![
        2.0, 3.0, 1.0,
        4.0, 7.0, 2.0,
        6.0, 18.0, 5.0,
    ]);
    let result = lu(&m).unwrap();
    let l = result.l.read_matrix();
    let n = 3;
    for i in 0..n {
        // diagonal must be 1
        assert!((l[i * n + i] - 1.0).abs() < 1e-9, "L diagonal[{}] != 1", i);
        // above diagonal must be 0
        for j in (i + 1)..n {
            assert!(l[i * n + j].abs() < 1e-9, "L[{},{}] should be 0", i, j);
        }
    }
}

#[test]
fn test_lu_u_is_upper_triangular() {
    let m = Matrix::new(3, 3, vec![
        2.0, 3.0, 1.0,
        4.0, 7.0, 2.0,
        6.0, 18.0, 5.0,
    ]);
    let result = lu(&m).unwrap();
    let u = result.u.read_matrix();
    let n = 3;
    for i in 0..n {
        for j in 0..i {
            assert!(u[i * n + j].abs() < 1e-9, "U[{},{}] should be 0", i, j);
        }
    }
}

#[test]
fn test_lu_requires_pivoting_pa_eq_lu() {
    // First element is 0, forcing a row swap
    let m = Matrix::new(3, 3, vec![
        0.0, 2.0, 1.0,
        3.0, 1.0, 4.0,
        6.0, 5.0, 2.0,
    ]);
    assert_pa_eq_lu(&m, 3);
}

#[test]
fn test_lu_does_not_mutate_input() {
    let m = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let original = m.read_matrix().clone();
    lu(&m).unwrap();
    assert_eq!(m.read_matrix(), &original);
}

#[test]
fn test_lu_singular_returns_err() {
    let m = Matrix::new(3, 3, vec![
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        7.0, 8.0, 9.0,
    ]);
    assert!(lu(&m).is_err());
}

#[test]
fn test_lu_non_square_returns_err() {
    let m = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    assert!(lu(&m).is_err());
}
