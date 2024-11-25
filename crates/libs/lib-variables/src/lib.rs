fn mutability_example() {
    let mut text: String = String::from("first text");
    println!("Mutability first value: {}", text);

    text = String::from("second text");
    println!("Mutability second value: {}", text);
}


fn shadowing_example() {
    let exe: String = String::from("abc");

    let exe: usize = exe.len();

    println!("Shadow value: {}", exe);
}


pub fn variables_facade() {
    println!("=== Mutability/Immutability ===");
    mutability_example();

    println!("=== Shadowing ===");
    shadowing_example();
}