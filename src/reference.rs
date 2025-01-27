fn main() {
  let mut s1 = String::from("priyansh");
  do_something(&mut s1);
  println!("name is {}", s1);
}

fn do_something(s2: &mut String) {
  s2.push_str(" patel");
  println!("{}", s2);
}
