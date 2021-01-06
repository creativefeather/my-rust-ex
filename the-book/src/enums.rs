#![allow(unused, dead_code)]

// Each variant of an enum has a "kind", e.g. V4, and can have
// associated data--making it effectively an Algebraic Data Type.
enum IpAddr {
  V4(u8, u8, u8, u8),
  V6(String),
}

// Notice we have structs and tuple structs here
enum Message {
  Quit,
  Move { x: i32, y:i32 },
  Write(String),
  ChangeColor(i32, i32, i32)
}

fn main() {
  
}


// ???Enums as value range validators???
enum PreEven {
  Even(i32),
  Odd
} // The structure of the enum here is actually the same as Option<T>

impl PreEven {
  pub fn validate(n: i32) -> PreEven {
    if n % 2 == 0 {
      return PreEven::Even(n);
    } else {
      return PreEven::Odd;
    } 
  }
}

// Note: If enums as primitive type subsets, I guess this is how
// I'd do it ...
struct Even(i32);
impl Even {
  pub fn validate(n: i32) -> Option<Even> {
    if n % 2 == 0 { return Option::Some(Even(n)); }
    else { return Option::None}
  }
}