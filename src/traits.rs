pub trait Summary {
  fn summarize(&self) -> String;
}

struct User {
  name: String,
  age: u32,
}

impl Summary for User {
  fn summarize(&self) -> String {
      format!("{} is {} years old", self.name, self.age)
  }
}

// Trait as parameter
pub fn notify(structure: impl Summary) {
  println!("{}", structure.summarize());
}

fn main() {
  let user = User {
      name: String::from("Priynansh"),
      age: 20,
  };

  notify(user);
}
