fn main() {
  let name = String::from("Priyansh");
  let length = get_str_length(name.clone());
  println!("Length of {} is {}!", name, length);
}

fn get_str_length(str: String) -> usize {
  str.chars().count()
}
