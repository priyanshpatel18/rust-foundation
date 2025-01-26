fn main() {
  let name = String::from("Priyansh");

  let first_a = find_first_a(name);
  println!("{:?}", first_a);
}

fn find_first_a(str: String) -> Option<i32> {
  for (index, char) in str.chars().enumerate() {
      if char == 'a' {
          return Some(index as i32);
      }
  }

  return None;
}
