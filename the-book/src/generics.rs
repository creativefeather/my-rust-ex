#![allow(dead_code)]
struct Point<T> {
  x: T,
  y: T
}

impl<T> Point<T> {
  fn x(&self) -> &T {
    &self.x
  }
}

impl Point<f32> {
  fn distance_from_origin(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}

pub trait Summary {
  fn summarize(&self) -> String;
}

fn main() {
  do_largest();
  do_largest_w_generics();
}

fn do_largest() {
  // Note: here an array, later a vector.
  let number_list = [50, 25, 100, 65];

  let result = largest(&number_list);
  println!("The largest number is {}", result);

  let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

  let result = largest(&number_list[3..]);
  println!("The largest number is {}", result);
}

// !!! 'list' represents any concrete slice of i32
// !!! This isn't generics being used, but it is sort of an abstract
// !!! list.
fn largest(list: &[i32]) -> &i32 {
  let mut largest = &list[0];

  for item in list {
    if item > largest {
      largest = item;
    }
  }
  
  largest
}

fn do_largest_w_generics() {
  let number_list = vec![34, 50, 25, 100, 65];

  let result = largest_w_generics(&number_list);
  println!("The largest number is {}", result);
  
  let char_list = vec!['y', 'm', 'a', 'q'];

  let result = largest_w_generics(&char_list);
  println!("The largest char is {}", result);
}
fn largest_w_generics<T: PartialOrd>(list: &[T]) -> &T {
  let mut largest = &list[0];

  for item in list {
    if item > largest {
      largest = item;
    }
  }
  
  largest
}
// Add function that will add integers or floats