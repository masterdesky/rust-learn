fn main() {
    let x = 5;

    let x = x + 1;

    // This is a new scope and the previous `x` is shadowed only within this block
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}