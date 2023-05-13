/**
 * Scalar Types
 * Signed Integers: i8, i16, i32, i64, i128 and isize (pointer size)
 * Unsigned Integers: u8, u16, u32, u64, u128 and usize (pointer size)
 * Floating Point: f32, f64
 * char Unicode Scalar Values like 'a', 'α' and '∞' (4 bytes each)
 * bool either true or false
 * and the unit type (), whose only possible value is an empty tuple: ()
 */

/**
 * Compound Types
 * Arrays like [1, 2, 3]
 * Tuples like (1, true)
 */

fn main() {
  let logical: bool = true;

  let a_float: f64 = 1.0;
  let an_integer = 5i32;

  let mut inferred_type = 12;
  inferred_type = 4294967296i64;

  // Mutable variables can be changed
  let mut mutable = 12;
  mutable = 21;

  // Error! The type of a variable can't be changed
  // mutable = true;

  // Variables can be overwritten with shadowing
  let mut mutable = true;
}