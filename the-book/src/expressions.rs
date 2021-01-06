fn main() {
  let mut a = 0;
  
  println!("block_scope: {}", block_scope());
  println!("if expression: {}", ternary());
  println!("loop: {}", loop {
    a += 1;
    if a < 10 { 
      continue;
    }
    break a
  })
}

fn block_scope() -> i32 {
  let a = 8;
  {
    let b = 16;
    a + b
  }
}

fn ternary() -> i32 {
  // very much like a ternary operator in other languages. But, 'else'
  // must be present, otherwise it is a unit type not an i32.
  if true { 5 } else { 6 }
}