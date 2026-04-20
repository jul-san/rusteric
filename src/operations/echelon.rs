use crate::matrix::Matrix;

fn find_pivot_row(
    data: &[f64], 
    col_count: usize, 
    from_row: usize, 
    row_count: usize, 
    col: usize) -> Option<usize>{

    for row in from_row..row_count{
        if data[row*col_count + col] != 0.0{
            return Some(row);
        }
    }
    None
}

fn swap_rows(data: &mut [f64], 
    col_count: usize, 
    row_a: usize, 
    row_b: usize){

    for col in 0..col_count{
        let index_a = row_a*col_count + col;
        let index_b = row_b*col_count + col;
        data.swap(index_a, index_b);
    }
}

fn normalize_row(data: &mut [f64], 
    col_count: usize, 
    row: usize, 
    from_col: usize){

  let pivot = data[row * col_count + from_col];
  for col in from_col..col_count {
    data[row * col_count + col] /= pivot;
  }
}

fn eliminate_below(data: &mut [f64], 
    col_count: usize, 
    pivot_row: usize, 
    row_count: usize, 
    pivot_col: usize){

  for row in (pivot_row + 1)..row_count {
    let factor = data[row * col_count + pivot_col];
    for col in pivot_col..col_count {
      data[row * col_count + col] -= factor * data[pivot_row * col_count + col];
    }
  }
}

fn eliminate_above(data: &mut [f64],
    col_count: usize,
    pivot_row: usize,
    pivot_col: usize){

  for row in 0..pivot_row {
    let factor = data[row * col_count + pivot_col];
    for col in pivot_col..col_count {
      data[row * col_count + col] -= factor * data[pivot_row * col_count + col];
    }
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
    eliminate_above(data, col_count, pivot_row, pivot_col);
    pivot_col += 1;
  }
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

    let Some(max_row) = find_pivot_row(data, 
        col_count, 
        pivot_row, 
        row_count, 
        pivot_col)
    else{
        continue;
    };

    if max_row != pivot_row {
      swap_rows(data, 
          col_count, 
          pivot_row, 
          max_row);
    }

    normalize_row(data, 
        col_count, 
        pivot_row, 
        pivot_col);

    eliminate_below(data, 
        col_count, 
        pivot_row, 
        row_count, 
        pivot_col);

    pivot_row += 1;
  }
}
