// Generic Trait Bound
fn method_with_generic<T: IntoIterator + std::fmt::Debug>(list: T) {
 println!("{list:?}");
}

pub fn generics_showcase() {
  println!("=== Generics ===");

  method_with_generic(vec![1, 2, 3]);
  method_with_generic(vec!["a", "b", "c"]);
}