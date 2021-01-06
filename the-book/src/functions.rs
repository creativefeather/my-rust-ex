fn main() {
  let num = (7, 8);
  another_function(num.0, num.1);
}

fn another_function(x: i32, y: i32) -> i32 {
  println!("The value of x is: {}", x);
  y
}