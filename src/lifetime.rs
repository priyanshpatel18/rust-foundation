fn main() {
  let ans;
  let first_name = String::from("Priyansh");
  {
      let last_name = String::from("Patel");
      ans = longest(&first_name, &last_name);
  }

  println!("Longest is {}", ans);
}

fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
  if str1.len() > str2.len() {
      str1
  } else {
      str2
  }
}
