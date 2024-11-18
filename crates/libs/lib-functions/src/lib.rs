fn with_parameters_example(number: i32) {
    println!("{}", number);
}

fn with_returns(a: i32, b: i32) -> i32 {
    a + b
}

pub fn functions_facade() {
    println!("=== Functions with Parameters ===");
    with_parameters_example(123);

    println!("=== Functions with Returns ===");
    let result: i32 = with_returns(10, 12);
    println!("{}", result);

    println!("=== Clousures Functions ===");
    let multiply = |x: i32, y:i32| println!("{}", x * y);
    multiply(5, 10);
}