fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
  if a.len() > b.len() {
    a
  } else {
    b
  }
}

pub fn lifetimes_showcase() {
  println!("=== Lifetimes ===");
  let string1 = "long";
  let string2 = "large";

  let result = longest(string1, string2);
  println!("The longest string is: {result}");
}