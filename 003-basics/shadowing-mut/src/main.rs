fn main() {
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Number of spaces: {}", spaces);

    // However, the following would cause a compile-time error due to type mismatch
    // let mut spaces = "   ";
    // spaces = spaces.len();
}
