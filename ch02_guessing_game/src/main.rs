use std::io; // this is a module in rust std library for input/output operations

fn main() {
    println!("Guess my number!");

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

    println!("You guessed: {}", number_guessed);
}
