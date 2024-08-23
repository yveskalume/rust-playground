use std::cmp::Ordering;
use std::io;

use rand::Rng;

#[derive(Debug)]
struct Player {
    name: String,
    tries: i32,
}

impl Player {
    fn create_new(name: String) -> Player {
        Player {
            name,
            tries: 0,
        }
    }

    fn new_try(&mut self) {
        self.tries += 1;
    }
}


fn main() {
    let program_names = ["Guessing Game", "Fibonacci sequence"];
    let program_functions = [guessing_game, fibonacci];

    let mut choice = String::new();

    println!("Welcome to my Rust playground");
    println!("Which program do you want to run");
    println!("---------------------------------");

    let mut program_index = 1;
    for program_name in program_names {
        println!("{}. {}", program_index, program_name);
        program_index += 1;
    }

    io::stdin().read_line(&mut choice)
        .expect("An error occured");

    let choice: i32 = choice.trim().parse().expect("Please enter a valid choice !");

    if choice - 1 >= 0 && choice - 1 < program_functions.len() as u32 as i32 {
        let choice_index: usize = (choice - 1) as usize;
        program_functions[choice_index]()
    } else {
        println!("Please enter a valid choice !");
    }
}

fn guessing_game() {
    println!("Guess the number");
    println!("----------------------------");

    println!("Please enter your name : ");

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("An error occurred");

    let mut player = Player::create_new(String::from(name.trim()));


    let secret_number = rand::thread_rng().gen_range(1..=50);

    println!("WELCOME {}", player.name);

    loop {
        player.new_try();

        let mut guess = String::new();

        println!("Please input your guess.");

        io::stdin().read_line(&mut guess)
            .expect("An error occurred");

        if guess.trim() == "quit" {
            println!("Quitting....");
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please type a valid number");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low (Try {})",player.tries),
            Ordering::Greater => println!("Too high (Try {})",player.tries),

            Ordering::Equal => {
                println!("Congratulations !!! {:?}", player);
                break;
            }
        }

        println!("      ")
    }
}

fn fibonacci() {
    println!("Fibonacci sequence");
    println!("----------------------------");
    let mut number = String::new();

    println!("Please how many number of the sequence do you want to see ?");

    io::stdin().read_line(&mut number)
        .expect("An error occurred");

    let number: i128 = number.trim().parse().expect("Invalid number");

    let mut first_number = 0;
    let mut second_number = 1;
    if number > 2 {
        print!("{},{},", first_number, second_number);
        for _i in 0..number - 2 {
            let actual = first_number + second_number;
            print!("{},", actual);
            first_number = second_number;
            second_number = actual;
        }
    } else if number == 2 {
        println!("{},{},", first_number, second_number);
    } else if number == 1 {
        println!("{}", first_number);
    }
}
