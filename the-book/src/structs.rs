#![allow(unused, dead_code)]

struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool
}

// Tuple Structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-Like Struct
struct UnitLike();

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32
}

impl Rectangle {
  // An associated constructor function
  fn square(size: u32) -> Rectangle {
    Rectangle {
      width: size,
      height: size
    }
  }
  // Having to specify self for a method seems dumb at first, but this
  // enables you to specify immutibility or mutability--other langaguages
  // it's not possible to enforce methods that don't allow mutability. Also,
  // passing ownership is possible. It's just not a concept in other languages.
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

fn main() {
  let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1
  };

  let mut user2 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1
  };
  
  user2.email = String::from("anotheremail@examplecom");
  
  // Struct Update Syntax ...
  let user3 = User {
    email: String:: from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user2
  };
  
  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);
  
  let rect = Rectangle {
    width: 30,
    height: 50
  };

  println!(
    "The area of {:?} is {} square pixels",
    rect,
    rect.area());
}

fn build_user(email: String, username: String) -> User {
  // Field Init Shorthand ...
  User {
    email,
    username,
    active: true,
    sign_in_count: 1
  }
}
