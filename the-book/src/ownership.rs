#![allow(unused)]

fn main() {
  let x = 5;
  let mut s = String::from("hello");
  s.push_str(", world!");

  first_word(&s);
  
  takes_ownership(s);
  makes_copy(x);
  
}

fn takes_ownership(some_string: String) {
  println!("Takes ownership of a 'String'");
  println!("{}", some_string);
  println!();
}

fn makes_copy(some_integer: i32) {
  println!("Makes copy of scalar value i32");
  println!("{}", some_integer);
  println!();
}

// fn dangle() -> &String {
//   let s = String::from("dangle me");

//   &s
// }

fn no_dangle() -> String {
  let s = String::from("no dangle");

  s
}

fn str_slice() {
  // notice that a string literal is the same type as a string slice
  let a_string = "hello world";
  let s = String::from(a_string);

  let hello = &s[0..5];
  let world = &s[6..11];
}

fn first_word(s: &str) -> &str {
  s
}