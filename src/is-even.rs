fn main() {
  let number = 18;
  let ans = if is_even(number) { "even" } else { "odd" };
  println!("{} is {}", number, ans);
}

fn is_even(n: i32) -> bool {
  n % 2 == 0
}
