use std::{collections::HashMap, vec};

pub fn show_variable_types() {
    let integer: i32 = 100;
    println!("The value of 'integer' is: {}", integer);

    let floatter: f64 = 3.4324;
    println!("The value of 'floatter' is: {}", floatter);

    let maybe: bool = true;
    println!("The value of 'maybe' is: {}", maybe);

    let stringer: String = String::from("idk man :b");
    println!("The value of 'stringer' is: {}", stringer);
}


pub fn show_compound_types() {
    let tuple: (i32, bool) = (100, false);
    println!("The value of the tuple is: {:?}", tuple);

    let array: [&str; 5] = ["a", "b", "c", "d", "e"];
    println!("The value of the array is: {:?}", array);

    let mut vector: Vec<i32> = vec![1, 2, 3, 4];
    vector.push(100);
    println!("The value of the vector is: {:?}", vector);

    let hashmap: HashMap<&str, i32> = HashMap::from([
        ("first", 12), ("second", 24)
    ]);
    println!("The value of the hashmap is: {:?}", hashmap);
}


pub fn mutability_example() {
    let mut text: String = String::from("first text");
    println!("Mutability first value: {}", text);

    text = String::from("second text");
    println!("Mutability second value: {}", text);
}


pub fn shadowing_example() {
    let exe: String = String::from("abc");

    let exe: usize = exe.len();

    println!("Shadow value: {}", exe);
}


pub fn variables_facade() {
    show_variable_types();
    show_compound_types();

    mutability_example();
    shadowing_example();
}