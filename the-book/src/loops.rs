fn main() {
  let mut counter = 0;

  let result = loop {
    counter += 1;

    if counter == 10 {
      break counter * 7;
    }
  };  // <-- don't forget the semi-colon

  println!("The result is {}", result);
}