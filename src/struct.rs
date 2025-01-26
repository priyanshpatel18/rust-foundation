struct User {
  name: String,
  age: i32,
}

fn main() {
  let user = User {
      name: String::from("Priyansh"),
      age: 20,
  };

  println!("{} is {} years old", user.name, user.age);
}
