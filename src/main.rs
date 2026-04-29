mod matrix;
mod operations;

use matrix::Matrix;
use operations::echelon::{row_echelon, reduced_row_echelon};
use operations::arithmetic::{add, subtract, multiply};
use operations::determinant::determinant;
use operations::inverse::inverse;

fn main() {
    let m1 = Matrix::new(3, 3, vec![ 1.0, 2.0, 3.0, 2.0, 4.0, 6.0, 1.0, 1.0, 1.0]);
    let mut m2 = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);

    print_section("Printing M1");
    m1.print_matrix();

    print_section("Printing M2");
    m2.print_matrix();

    print_section("Printing Number of Rows in M1");
    println!("{}", m1.get_rows());

    print_section("Printing Number of Columns in M1");
    println!("{}", m1.get_cols());

    print_section("Performing Row Echelon Reduction");
    print_section("BEFORE");
    m2.print_matrix();

    print_section("AFTER");
    row_echelon(&mut m2);
    m2.print_matrix();

    print_section("Performing Reduced Row Echelon Reduction");
    print_section("BEFORE");
    m2.print_matrix();

    print_section("AFTER");
    reduced_row_echelon(&mut m2);
    m2.print_matrix();

    // --- Arithmetic showcases ---
    let a = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let b = Matrix::new(2, 2, vec![5.0, 6.0, 7.0, 8.0]);

    print_section("Matrix Addition (A + B)");
    println!("A:");
    a.print_matrix();
    println!("B:");
    b.print_matrix();
    println!("Result:");
    add(&a, &b).unwrap().print_matrix();

    print_section("Matrix Subtraction (A - B)");
    println!("A:");
    a.print_matrix();
    println!("B:");
    b.print_matrix();
    println!("Result:");
    subtract(&a, &b).unwrap().print_matrix();

    print_section("Matrix Multiplication (A * B)");
    println!("A:");
    a.print_matrix();
    println!("B:");
    b.print_matrix();
    println!("Result:");
    multiply(&a, &b).unwrap().print_matrix();

    // non-square multiplication: 2x3 * 3x2
    let c = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let d = Matrix::new(3, 2, vec![7.0, 8.0, 9.0, 10.0, 11.0, 12.0]);

    print_section("Non-Square Multiplication (2x3 * 3x2 = 2x2)");
    println!("C (2x3):");
    c.print_matrix();
    println!("D (3x2):");
    d.print_matrix();
    println!("Result (2x2):");
    multiply(&c, &d).unwrap().print_matrix();

    let e = Matrix::new(3, 3, vec![1.0, 2.0, 3.0, 0.0, 1.0, 4.0, 5.0, 6.0, 0.0]);

    print_section("Determinant of E");
    println!("E:");
    e.print_matrix();
    println!("det(E) = {}", determinant(&e).unwrap());

    print_section("Inverse of E");
    println!("E:");
    e.print_matrix();
    println!("E^-1:");
    inverse(&e).unwrap().print_matrix();
}

fn print_section(title: &str) {
  println!("\n==================== {} ====================\n", title);
}
