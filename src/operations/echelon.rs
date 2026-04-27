use crate::matrix::Matrix;
use crate::operations::row_ops::{swap_rows, scale_row, add_scaled_row};

fn find_pivot_row(
    data: &[f64],
    col_count: usize,
    from_row: usize,
    row_count: usize,
    col: usize) -> Option<usize> {

    for row in from_row..row_count {
        if data[row * col_count + col] != 0.0 {
            return Some(row);
        }
    }
    None
}

pub fn row_echelon(matrix: &mut Matrix) {
    let row_count = matrix.get_rows();
    let col_count = matrix.get_cols();
    let data = matrix.mut_matrix();

    let mut pivot_row = 0;

    for pivot_col in 0..col_count {
        if pivot_row >= row_count {
            break;
        }

        let Some(found_row) = find_pivot_row(data, col_count, pivot_row, row_count, pivot_col)
        else {
            continue;
        };

        if found_row != pivot_row {
            swap_rows(data, col_count, pivot_row, found_row);
        }

        let pivot_val = data[pivot_row * col_count + pivot_col];
        scale_row(data, col_count, pivot_row, 1.0 / pivot_val);

        for row in (pivot_row + 1)..row_count {
            let factor = data[row * col_count + pivot_col];
            add_scaled_row(data, col_count, row, pivot_row, -factor);
        }

        pivot_row += 1;
    }
}

pub fn reduced_row_echelon(matrix: &mut Matrix) {
    row_echelon(matrix);

    let row_count = matrix.get_rows();
    let col_count = matrix.get_cols();
    let data = matrix.mut_matrix();

    let mut pivot_col = 0;
    for pivot_row in 0..row_count {
        while pivot_col < col_count && data[pivot_row * col_count + pivot_col] == 0.0 {
            pivot_col += 1;
        }
        if pivot_col >= col_count {
            break;
        }

        for row in 0..pivot_row {
            let factor = data[row * col_count + pivot_col];
            add_scaled_row(data, col_count, row, pivot_row, -factor);
        }

        pivot_col += 1;
    }
}
