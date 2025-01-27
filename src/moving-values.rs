fn main() {
  let mut s1 = String::from("priyansh");

  s1 = do_something(s1);
  println!("name is {}", s1);
}

fn do_something(s2: String) -> String {
  println!("{}", s2);
  return s2;
}
