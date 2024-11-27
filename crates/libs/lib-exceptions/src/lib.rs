fn divide(a: i32, b: i32) -> Result<i32, String> {
    if a == 0 || b == 0 {
        Err(String::from("No divide zero"))
    } else {
        Ok(a / b)
    }
}

fn process_divide() -> Result<i32, String> {
    let result = divide(10, 0)?;
    Ok(result)
}

pub fn exceptions_facade() {
    println!("=== Error Handling ===");

    match process_divide() {
        Ok(value) => println!("Result: {value}"),
        Err(e) => println!("Error: {e}")
    }
}