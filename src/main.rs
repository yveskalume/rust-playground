use std::io;

fn main() {
    guessing_game()
}

fn guessing_game() {
    println!("Guess the number");
    println!("----------------------------");
    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("An error occurred");

    println!("You guessed: {}", guess);
}
