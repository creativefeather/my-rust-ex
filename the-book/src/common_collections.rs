#![allow(unused, dead_code)]
use std::collections::HashMap;

[derive(Eq)]
enum Keys {
  Blue,
  Red,
  Yellow
}

impl Keys {
  fn to_string(&self) -> &str {
    match self {
      Keys::Blue => "Blue",
      Keys::Red => "Red",
      Keys::Yellow => "Yellow"
    }
  }
}
fn main() {
  // it would be kind of pointless to create a new empty vector that
  // was immutable.
  let mut v: Vec<i32> = Vec::new();  
  let v2 = vec![1, 2, 3, 4, 5];

  // iterate_mut_vector();
  // read_vector(&v);
  
  hash_insert();
}

fn read_vector(v: &Vec<i32>) {
  if let Some(third) = v.get(2) {
    println!("The third element is {}", third);
  } else {
    println!("There is no third element.");
  }

  // will panic if outside bounds of vector
  let third: &i32 = &v[2];
  println!("The third element is {}", third);
}

fn iterate_mut_vector() {
  let mut v = vec![100, 32, 57];

  println!("FOR LOOP:");
  for i in &mut v {
    *i += 50;
    println!("{}", i);
  }
}

fn push_str() {
  let mut s1 = String::from("foo");
  let s2 = "bar";

  // s2 is a &str, ownership is not being transfered
  s1.push_str(s2);
  s1.push('s');
  println!("s2 is {}", s2);
}

fn tic_tac_toe() {
  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");
  
  // Doesn't actually take ownership...
  let s = format!("{}-{}-{}", s1, s2, s3);
}

fn hash_insert() {
  let mut scores = HashMap::new();
  
  scores.insert(Keys::Blue, 10);
  scores.insert(Keys::Yellow, 50);
  
  if let Some(x) = scores.get(Keys::Blue) {
    println!("score: {}", x);
  }
  
  for (key, value) in &scores {
    println!("{}: {}", key.to_string(), value);
  }
}