use rand::Rng;
use std::cmp::Ordering; // this is a module in rust std library for comparing values
use std::io; // this is a module in rust std library for input/output operations // this is a module in rand crate for random number generation

fn main() {
    println!("Guess my number!");

    // generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        
        println!("Input your guess.");

        // q: what is let in rust?
        // a: let is a variable binding operator, so the variable can be used later
        // q: what is mut in rust?
        // a: mut is a mutable operator, so the variable can be changed
        // q: rust variables are immutable by default, how to make them mutable?
        // a: use mut keyword before the variable name to make it mutable
        // q: rust variables naming convention?
        // a: snake_case is the convention for variable naming in rust
        // q: what is :: in rust?
        // a: :: is an associated function, so it is called on the type directly
        let mut number_guessed = String::new();

        // q: what is & in rust?
        // a: & is a reference operator, so it is a reference to the variable
        io::stdin()
            .read_line(&mut number_guessed)
            .expect("Failed to read line");

        // q: what is "shadowing" in rust?
        // a: shadowing is a feature in rust that allows you to
        // reuse the same variable name for different types
        let number_guessed: u32 = match number_guessed
            .trim() // trim the "\n" from the input: when you hit enter
            // parse the input to a number
            .parse()
        {
            // Switching from an expect call to a match expression is how you
            // generally move from crashing on an error to handling the error.
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", number_guessed);

        match number_guessed.cmp(&secret_number) {
            // q: what is "=>" in rust?
            // a: => is a match operator, so it is used to match the value with the pattern
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
