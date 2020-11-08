use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn start_game() {
    println!("Guess the number!");

    // Generating a random number using rand module

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is: {}", secret_number);

    // Limit flag
    let mut limit = 0;
    // Infinite loop
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // Taking input from the user
        io::stdin()
            .read_line(&mut guess)
            .expect("Error to read line");

        // checking if user input is character or number if error so print and take another input
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("characters not valid");
                continue;
            }
        };
        println!("You guessed: {}", guess);

        // conditions to check both numbers
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                println!(
                    "you guessed the number in {} times & remaining changes are {}",
                    limit,
                    10 - limit
                );
                break;
            }
        }
        // exit game automatically if user guessing limit is over
        if limit == 10 {
            println!();
            println!("game over");
            println!("The secret number is: {}", secret_number);
            break;
        }
        limit += 1;
    }
}
fn main() {
    start_game()
}
