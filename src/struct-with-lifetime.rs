struct User<'a> {
  name: &'a str,
}

fn main() {
  let name = String::from("John");

  let user = User {
    name: &name,
  };

  println!("{}", user.name);
}
