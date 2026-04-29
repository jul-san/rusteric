use linear_rustgebra::operations::row_ops::{swap_rows, scale_row, add_scaled_row};

#[test]
fn test_swap_rows_2x2() {
    let mut data = vec![1.0, 2.0, 3.0, 4.0];
    swap_rows(&mut data, 2, 0, 1);
    assert_eq!(data, vec![3.0, 4.0, 1.0, 2.0]);
}

#[test]
fn test_swap_rows_3x3() {
    let mut data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    swap_rows(&mut data, 3, 0, 2);
    assert_eq!(data, vec![7.0, 8.0, 9.0, 4.0, 5.0, 6.0, 1.0, 2.0, 3.0]);
}

#[test]
fn test_swap_rows_same_row() {
    let mut data = vec![1.0, 2.0, 3.0, 4.0];
    swap_rows(&mut data, 2, 1, 1);
    assert_eq!(data, vec![1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn test_swap_rows_non_square() {
    let mut data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    swap_rows(&mut data, 3, 0, 1);
    assert_eq!(data, vec![4.0, 5.0, 6.0, 1.0, 2.0, 3.0]);
}

#[test]
fn test_scale_row_by_scalar() {
    let mut data = vec![1.0, 2.0, 3.0, 4.0];
    scale_row(&mut data, 2, 0, 3.0);
    assert_eq!(data, vec![3.0, 6.0, 3.0, 4.0]);
}

#[test]
fn test_scale_row_by_zero() {
    let mut data = vec![1.0, 2.0, 3.0, 4.0];
    scale_row(&mut data, 2, 1, 0.0);
    assert_eq!(data, vec![1.0, 2.0, 0.0, 0.0]);
}

#[test]
fn test_scale_row_by_negative() {
    let mut data = vec![1.0, 2.0, 3.0, 4.0];
    scale_row(&mut data, 2, 0, -1.0);
    assert_eq!(data, vec![-1.0, -2.0, 3.0, 4.0]);
}

#[test]
fn test_scale_row_by_fraction() {
    let mut data = vec![2.0, 4.0, 6.0, 8.0];
    scale_row(&mut data, 2, 1, 0.5);
    assert_eq!(data, vec![2.0, 4.0, 3.0, 4.0]);
}

#[test]
fn test_add_scaled_row_basic() {
    let mut data = vec![1.0, 2.0, 3.0, 4.0];
    add_scaled_row(&mut data, 2, 1, 0, 2.0);
    assert_eq!(data, vec![1.0, 2.0, 5.0, 8.0]);
}

#[test]
fn test_add_scaled_row_eliminate() {
    let mut data = vec![2.0, 4.0, 6.0, 12.0];
    add_scaled_row(&mut data, 2, 1, 0, -3.0);
    assert_eq!(data, vec![2.0, 4.0, 0.0, 0.0]);
}

#[test]
fn test_add_scaled_row_negative_factor() {
    let mut data = vec![1.0, 0.0, 2.0, 3.0, 0.0, 6.0];
    add_scaled_row(&mut data, 3, 0, 1, -1.0);
    assert_eq!(data, vec![-2.0, 0.0, -4.0, 3.0, 0.0, 6.0]);
}

#[test]
fn test_add_scaled_row_3x3() {
    let mut data = vec![1.0, 2.0, 3.0, 0.0, 1.0, 4.0, 5.0, 6.0, 0.0];
    add_scaled_row(&mut data, 3, 2, 0, -5.0);
    assert_eq!(data, vec![1.0, 2.0, 3.0, 0.0, 1.0, 4.0, 0.0, -4.0, -15.0]);
}
