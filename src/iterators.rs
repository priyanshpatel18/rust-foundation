fn main() {
  let mut nums = vec![1, 2, 3, 4, 5];

  // Iterating using for_loop
  for num in &nums {
      print!("{} ", num);
  }
  println!();

  // Iterating after creating the iterator
  let itr = nums.iter();
  for value in itr {
      print!("{} ", value);
  }
  println!();

  // Iterating using .next()
  // .next() gives Option<T>
  let mut itr = nums.iter();
  while let Some(value) = itr.next() {
      print!("{} ", value);
  }
  println!();

  // Mutable iterator
  let itr = nums.iter_mut();
  for value in itr {
      *value *= 10;
      print!("{} ", value);
  }
  println!();

  // into_iter()
  // Takes ownership of the vector
  // It is same as the first for_loop
  let itr = nums.into_iter();
  for value in itr {
      print!("{} ", value);
  }
  println!();

  // Iterator adapters
  // Methods that don't consume the iterator, but return a new iterator by changing some of its properties

  let nums = vec![1, 2, 3, 4, 5];

  // Map
  let map = nums.iter().map(|x| x * 10);
  for value in map {
      print!("{} ", value);
  }
  println!();

  // Filter
  let filter = nums.iter().filter(|x| *x % 2 == 0);
  for value in filter {
      print!("{} ", value);
  }
  println!();
}
