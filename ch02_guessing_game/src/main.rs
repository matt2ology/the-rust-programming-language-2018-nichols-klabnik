use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess my number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Input your guess.");

        let mut number_guessed = String::new();

        io::stdin()
            .read_line(&mut number_guessed)
            .expect("Failed to read line");

        let number_guessed: u32 = match number_guessed.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", number_guessed);

        match number_guessed.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
