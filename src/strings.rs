fn main() {
  let mut name = String::from("Priyansh");
  name.push_str(" Patel");
  println!("Name is {}", name);
  name.replace_range(8..name.len(), "");
  println!("Name is {}", name);
}
