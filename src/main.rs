use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    loop {
        let mut guess = String::new();
        let secret_number = 12;

        // Read the input and mutate the newly created string
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Parse the string
        let guess: u32 = match guess.trim().parse() {
            Ok(result) => result,
            Err(_) => continue,
        };

        // Compare the guess to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("That's it! You guessed it!");
                break;
            },
        }
    }
}
