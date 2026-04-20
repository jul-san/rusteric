mod matrix;
mod operations;

use matrix::Matrix;
use operations::echelon::{row_echelon, reduced_row_echelon};

fn main() {
    let mut m1 = Matrix::new(3, 3, vec![ 1.0, 2.0, 3.0, 2.0, 4.0, 6.0, 1.0, 1.0, 1.0]);

    let mut m2 = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);

    print_section("Printing M1");
    m1.print_matrix();

    print_section("Printing M2");
    m2.print_matrix();

    print_section("Printing Number of Rows in M1");
    println!("{}", m1.get_rows());

    print_section("Printing Number of Columns in M1");
    println!("{}", m1.get_cols());

    print_section("Performaing Row Echelon Reduction");
    print_section("BEFORE");
    m2.print_matrix();

    print_section("AFTER");
    row_echelon(&mut m2);
    m2.print_matrix();

    print_section("Performaing Reduced Row Echelon Reduction");
    print_section("BEFORE");
    m2.print_matrix();

    print_section("AFTER");
    reduced_row_echelon(&mut m2);
    m2.print_matrix();


}

fn print_section(title: &str) {
  println!("\n==================== {} ====================\n", title);
}
