mod matrix;
mod operations;

use std::io::{self};
use matrix::Matrix;
use operations::{row_ops, echelon, arithmetic};

fn main() {
    let mut m1 = Matrix::new(vec![
        vec![1.0, 2.0, 3.0], 
        vec![2.0, 4.0, 6.0],
        vec![1.0, 1.0, 1.0]]);

    let mut m2 = Matrix::new(vec![
        vec![1.0, 2.0],
        vec![3.0, 4.0]]);

    print_section("Printing M1");
    m1.print_matrix();

    print_section("Printing M2");
    m2.print_matrix();

    print_section("Trying Addition");
    arithmetic::add(&mut m1, &mut m2);
}

fn print_section(title: &str) {
  println!("\n==================== {} ====================\n", title);
}
