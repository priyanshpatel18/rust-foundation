enum Shape {
  Rectangle { width: f32, height: f32 },
  Circle { radius: f32 },
}

fn main() {
  let rect = Shape::Rectangle {
      width: 5.0,
      height: 10.0,
  };
  println!("Area of rectangle: {}", calculate_area(rect));
  let circle = Shape::Circle { radius: 7.0 };
  println!("Area of circle: {}", calculate_area(circle));
}

fn calculate_area(shape: Shape) -> f32 {
  match shape {
      Shape::Rectangle { width, height } => width * height,
      Shape::Circle { radius } => 3.14 * radius * radius,
  }
}
