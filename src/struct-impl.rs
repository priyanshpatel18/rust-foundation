struct Triamgle {
  base: f64,
  height: f64,
}

impl Triamgle {
  fn area(&self) -> f64 {
      self.base * self.height / 2.0
  }

  fn perimeter(&self, demo: i32) -> f64 {
      self.base + self.height + demo as f64
  }

  fn debug() {
      println!("Debug");
  }
}

fn main() {
  let triangle = Triamgle {
      base: 10.0,
      height: 5.0,
  };

  println!("Area: {}", triangle.area());
  println!("Perimeter: {}", triangle.perimeter(1));
  Triamgle::debug();
}
