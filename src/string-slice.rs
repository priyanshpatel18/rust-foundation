fn main() {
  let str = String::from("Hello World");
  let mut index = 0;

  for i in str.chars() {
      if i == ' ' {
          break;
      }
      index += 1;
  }

  println!("{}", &str[0..index]);

  // Commonly Used Strings
  let s1 = String::from("Hello"); // String Type
  let s2 = &s1; // String Slice - has a 'view' of s1
  let s3 = "Hello"; // String Literal is a string slice but directly points to an address in binary
}
