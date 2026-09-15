use std::io;

fn main() {
    println!("Guess the number!");
    println!("Generating number...");


    println!("Done!");
    println!("Please type in your guess: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    println!("You guessed: {guess}");

}
