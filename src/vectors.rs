fn main() {
  let mut vect = Vec::new();
  vect.push(1);
  vect.push(2);
  vect.push(3);

  let new_vec = even_filter(&vect);
  println!("Original: {:?}", vect);

  println!("Even Filter: {:?}", new_vec);
  print!("Odd Filter: ");
  odd_filter(&mut vect);
  println!("{:?}", vect);

  let mut another_vec: Vec<i32> = vec![1, 2, 3];
  another_vec.push(4);
  println!("{:?}", another_vec);
}

fn even_filter(vec: &Vec<i32>) -> Vec<i32> {
  let mut new_vec = Vec::new();

  for i in vec {
      if i % 2 == 0 {
          new_vec.push(*i);
      }
  }

  return new_vec;
}

fn odd_filter(vec: &mut Vec<i32>) {
  for i in 0..vec.len() - 1 {
      if vec[i] % 2 != 1 {
          vec.remove(i);
      }
  }
}
