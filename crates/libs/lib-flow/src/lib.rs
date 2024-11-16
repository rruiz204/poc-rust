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

pub fn flow_facade() {
  simple_conditions(100);
  conditions_operators(16);

  let result: i32 = assing_conditions("zetha");
  println!("The result is: {}", result);
}
