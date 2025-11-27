const GLOBAL_CONSTANT: &str = "This is a global constant";

fn test_scope() {
    const LOCAL_CONSTANT: &str = "Local constant in function";
    println!("Local constant: {}", LOCAL_CONSTANT);
    println!("Accessing global constant from function: {}", GLOBAL_CONSTANT);

    // This line would cause an error because MAIN_CONSTANT is not in scope here 
    // println!("Accessing main constant from function: {}", MAIN_CONSTANT);

}

fn main() {
    println!("Testing constants in Rust!");
    
    println!("Global constant: {}", GLOBAL_CONSTANT);
    
    const MAIN_CONSTANT: &str = "Constant defined in main scope";
    println!("Main constant: {}", MAIN_CONSTANT);
    
    test_scope();
}