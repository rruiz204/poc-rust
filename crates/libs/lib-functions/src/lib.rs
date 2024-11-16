fn function_with_parameters(a: i32, b: i32) -> i32 {
    a * b
}


pub fn functions_facade() {
    println!("The result is: {}", function_with_parameters(3, 6));
}