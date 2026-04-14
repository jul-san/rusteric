use crate::matrix::Matrix;

pub fn row_echelon(matrix: &mut Matrix) {
  let data = matrix.mut_matrix();
  let row_count = data.len();
  let col_count = data[0].len();

  let mut pivot_row = 0;

  for pivot_col in 0..col_count {
    if pivot_row >= row_count {
      break;
    }

    // finding the pivot row
    let mut max_row = pivot_row;
    while max_row < row_count && data[max_row][pivot_col] == 0.0 {
      max_row += 1;
    }

    if max_row == row_count {
      continue;
    }

    // swap
    if max_row != pivot_row {
      data.swap(pivot_row, max_row);
    }

    // set pivot to 1
    let pivot = data[pivot_row][pivot_col];
    for col in pivot_col..col_count {
      data[pivot_row][col] /= pivot;
    }

    // remove below
    for row in (pivot_row + 1)..row_count {
      let factor = data[row][pivot_col];
      for col in pivot_col..col_count {
        data[row][col] -= factor * data[pivot_row][col];
      }
    }

    pivot_row += 1;
  }
}
