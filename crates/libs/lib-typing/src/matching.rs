fn rolling(number: &i32) {
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        _ => println!("Other")
    }
}

pub fn matching_showcase() {
  println!("=== Matching ===");
  let number: i32 = 2;
  rolling(&number);
}
