use rand;
use std::io;

fn main() {
    println!("Welcome to the guess the number game ! You have to guess a number between 1 and 10");

    let secret_number: u8 = rand::random_range(1..11);
    let mut guess_amount: u8 = 1;

    loop {
        let mut guess = String::new();

        println!("Guess number {guess_amount}");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to reade line");

        let parsed_guess = guess
            .trim()
            .parse::<u8>()
            .expect("Player guess should be a valid number");

        if parsed_guess == secret_number {
            println!(
                "Congratz ! You found the secret number after {guess_amount} tries: {secret_number}"
            );
            break;
        } else if parsed_guess > secret_number {
            println!("You number seems a bit high...");
        } else {
            println!("Your number seems a bit low...");
        }

        guess_amount += 1;
    }

    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to read line");
}
