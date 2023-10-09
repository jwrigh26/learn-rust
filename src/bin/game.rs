use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret_number);

    println!("Guess the number betweem 1 and 100!");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Attempt to parse the guess to u32 first
        let temp_guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // How to guard against values greater than u8::MAX?

        let guess: u8 = match guard_value(temp_guess) {
            Ok(value) => value,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn guard_value(value: u32) -> Result<u8, String> {
    if value > u8::MAX as u32 {
        return Err(String::from("Guess is way too big! Try again and guess a number between 1 and 100."));
    }
    Ok(value as u8)
}
