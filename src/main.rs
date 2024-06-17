use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    guessing_game()
}

fn guessing_game() {
    println!("Guess the number");
    println!("----------------------------");

    let secret_number = rand::thread_rng().gen_range(1..=50);


    loop {
        let mut guess = String::new();

        println!("Please input your guess.");

        io::stdin().read_line(&mut guess)
            .expect("An error occurred");

        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please type a valid number");
                continue
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too high"),

            Ordering::Equal => {
                println!("Congratulations !!!");
                break;
            }
        }

        println!("      ")
    }
}
