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
}