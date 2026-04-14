use crate::matrix::Matrix;

pub fn add(m1: &mut Matrix, m2: &mut Matrix){
    //check if dimensions match 
    if m1.read_matrix().len() != m2.read_matrix().len() {
        println!("Dimensions do not match!");
    }
}
