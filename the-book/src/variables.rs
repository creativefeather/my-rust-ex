fn main() {

  // shadowing
  let x = b'A';
  println!("The value of x is: {}", x);
  let x = 6.0f32; // shadowing...different than having a mutable variable
  println!("The value of x is: {}\n", x);

  // Move
  let x = 5;
  let y = x;

  println!("{}", "let x = 5;");
  println!("{}", "let y = x;");
  println!("x: {}\n", x);

  let s1 = String::from("hello");
  let s2 = s1;

  println!("{}", "let s1 = String::from(\"hello\");");
  println!("{}", "let s2 = s1;");
  println!("{}", s1);
}