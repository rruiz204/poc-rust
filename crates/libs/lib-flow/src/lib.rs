fn simple_conditions(number: i32) {
    if number == 0 {return;}

    if number % 2 == 0 {
        println!("is an even number");
    } else {
        println!("is an odd number");
    }
}


fn conditions_operators(number: i32) {
    if number >= 10 && number <= 20 {
      println!("it is a valid number");
    }
    return;
}


fn assing_conditions(caterogy: &str) -> i32 {
    let number: i32 = if caterogy == "zetha" {10} else {4};
    return number;
}


fn loops_with_ranges() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    for number in 1..numbers.len() {
        println!("{number}");
    }
}


fn loops_with_collections() {
    let vector: Vec<i32> = vec![1, 2, 3];

    for number in vector {
        println!("{number}")
    }
}

pub fn flow_facade() {
  simple_conditions(100);
  conditions_operators(16);

  let result: i32 = assing_conditions("zetha");
  println!("The result is: {}", result);

  loops_with_ranges();
  loops_with_collections();
}
