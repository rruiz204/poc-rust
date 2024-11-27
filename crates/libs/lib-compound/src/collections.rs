fn vectors_example() {
  println!("=== Vectors ===");
  let mut numbers: Vec<i32> = vec![1, 2, 3];
  numbers.push(4);

  for number in &mut numbers {
    *number += 10;
  }

  let second_item: &i32 = &numbers[1];
  println!("The second element is {second_item}");
}

pub fn collections_showcase() {
  vectors_example();
}