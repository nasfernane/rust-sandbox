use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the guess the number game ! You have to guess a number between 1 and 10");

    let secret_number: u8 = rand::random_range(1..=10);
    let mut guess_amount: u8 = 1;

    loop {
        let mut guess = String::new();

        println!("Guess number {guess_amount}");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to reade line");

        let guess = match guess.trim().parse::<u8>() {
            Ok(num) => num,
            Err(_) => {
                println!("Your number must be a valid number");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small !"),
            Ordering::Greater => println!("Too big !"),
            Ordering::Equal => {
                println!(
                    "Congratz ! You found the secret number after {guess_amount} tries: {secret_number}"
                );
                break;
            }
        }

        guess_amount += 1;
    }
}
