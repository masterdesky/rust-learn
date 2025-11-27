fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The guessed number is: {}", guess);

    // However, this will cause a compile-time error because the type cannot be inferred
    // let guess = "42".parse().expect("Not a number!");

    
}
