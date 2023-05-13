use std::fmt::{self, Formatter, Display, Result};

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
  }
}

fn transpose(matrix: Matrix) -> Matrix {
  let Matrix(a, b, c, d) = matrix;

  return Matrix(a, c, b, d)
}

fn main() {
  let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
  println!("{}", matrix);
  println!("{}", transpose(matrix));
}