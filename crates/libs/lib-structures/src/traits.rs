trait Shape {
  fn area(&self) -> f64;
}

struct Rectangle {
  width: f64,
  height: f64,
}

impl Shape for Rectangle {
  fn area(&self) -> f64 {
    self.width * self.height
  }
}

struct Triangle {
  base: f64,
  height: f64,
}

impl Shape for Triangle {
  fn area(&self) -> f64 {
    0.5 * self.base * self.height
  }
}

fn show_area<T: Shape>(shape: &T) /* where T: Shape */ {
  println!("Area: {:.2}", shape.area());
}

pub fn traits_showcase() {
  println!("=== Traits ===");

  let rectangle = Rectangle { width: 4.0, height: 7.0 };
  let triangle = Triangle { base: 3.0, height: 6.0 };

  show_area(&rectangle);
  show_area(&triangle);
}