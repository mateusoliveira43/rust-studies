use rand::Rng;
use std::cmp::Ordering;
use std::io;

const INVALID_GUESS: &str = "Invalid guess. Guess must be an integer between 1 and 100.";

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                if (num > 100) | (num == 0) {
                    println!("{INVALID_GUESS}");
                    continue;
                }
                num
            }
            Err(_) => {
                println!("{INVALID_GUESS}");
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

// TODO add tests
// 0 and negatives are invalid
// 101 and highers are invalid
// strings are invalid
// floats are invalid
// fix rand seed and do an iteration until win
// iteration until win, without fixed rand seed
