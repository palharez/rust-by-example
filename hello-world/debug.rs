#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
  let name = "Eduardo";
  let age = 25;

  let eduardo = Person { name, age };

  // Pretty print
  println!("{:#?}", eduardo);
}