pub struct Matrix{
    rows: usize,
    cols: usize,
    matrix: Vec<f64>
}

impl Matrix{
    pub fn new(r: usize, c: usize, input: Vec<f64>) -> Matrix{
        Matrix {
            rows: r,
            cols: c,
            matrix: input,
        }
    }

    pub fn print_matrix(&self){
        for row in 0..self.rows{
            for col in 0..self.cols{
                print!("{} ", self.matrix[row * self.cols + col]);
            }
            println!();
        }
    }

    pub fn read_matrix(&self) -> &Vec<f64> {
        return &self.matrix;
    }

    pub fn mut_matrix(&mut self) -> &mut Vec<f64> {
        return &mut self.matrix;
    }

    pub fn get_rows(&self) -> usize {
        return self.rows;
    }

    pub fn get_cols(&self) -> usize {
        return self.cols;
    }
}
