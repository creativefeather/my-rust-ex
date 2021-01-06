use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    
    // inclusive on the lower bound, exclusive on the upper bound
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess = String::new();

        println!("Please input your guess.");
        
        // '&mut' indicates a mutable reference
        // read_line returns an io::result
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        // parse returns a 'Result' ... pattern match Result enum or Result.expect()
        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue    
        };
        
        println!("You guessed: {}", guess); 
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    println!("The secret number is: {}", secret_number);
}