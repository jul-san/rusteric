use std::io;

fn main() {

    println!("Enter the dimension (n) of your matrix: ");
    let mut dimension = String::new(); 
    io::stdin()
        .read_line(&mut dimension)
        .expect("Could not read input");

    let matrix = create_matrix(dimension);

    print_matrix(&matrix);
    let matrix = define_values(matrix);
    print_matrix(&matrix);
     
}

fn create_matrix(dimension: String) -> Vec<Vec<i32>> {
    let n: usize = dimension
        .trim()
        .parse()
        .expect("number");

    let matrix = vec![vec![0; n]; n];

    return matrix;
}

fn define_values(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {

    let mut row_counter = 0;
    for row in 0..matrix.len() {
        loop{
            println!("Enter values for the row {row_counter} seperated by spaces:\n");
        
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Invalid values");

            let values: Vec<i32> = input
                .split_whitespace()
                .map(|x| x.parse::<i32>().expect("Enter a valid input"))
                .collect();

            if values.len() != matrix[row].len() {
                let length = matrix[row].len();
                println!("Please enter the correct number of values ({length})\n");
                continue;
            }
        
            matrix[row] = values;
            row_counter += 1;
            break;
        }
    }

    return matrix;
}

fn print_matrix(matrix: &Vec<Vec<i32>>){
    for row in matrix {
        println!("{:?}", row);
    }
}
