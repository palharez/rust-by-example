static LANGUAGE: &str = "Rust";
static THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
    let n = 3;

    println!("{}", LANGUAGE);
    println!("{}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // THRESHOLD = 5; // error: cannot assign to this expression
}
