extern crate rand;  // we added this just by adding (rand = "0.7.3") under dependencies in Cargo.toml and run "cargo update"
mod fibonacci;
mod guess;
use guess::{get_guess,handle_guess};
use std::collections::HashMap; // std::collections is the standard module to create collections
use std::io; // The "io" module is to input/output data from user and etc.
use rand::random;
// To run application simply run "cargo run" in project root
// Below is for documentation of code! --> to make docs run "cargo doc --open"!
// --open options opens the doc inside browser
/// This is a doc comment! It gets documented.
/// # Example (This is a markdown!)
/// ```rust
/// println!("Hello World!")
/// ```
fn main() {
    // Code 1 : Fibonacci
    println!("Static Fibonacci : ");
    for i in 1..10{
        let fibonacci: u64 = fibonacci::fb(i);
        println!("{} : {}", i, fibonacci);
    }

    // Code 2 : Dynamic Fibonacci
    println!("Dynamic Fibonacci : ");
    let mut map = HashMap::new();
    for i in 1..90{
        let fibonacci: u64 = fibonacci::fb_dynamic(i, &mut map);
        println!("{} : {}", i, fibonacci);
    }

    // Code 3: Get input from user in print it
    let mut data = String::new();
    println!("Enter the data :");
    io::stdin().read_line(&mut data);
    println!("The data is : {}", data);

    // Code 4: Let's have a guess game!!
    println!("Welcome to Guess Game!!");
    let correct = random::<u8>();
    println!("(This is a help :D :D) Correct value is : {}", correct);
    loop{
        let guess = get_guess();
        if handle_guess(guess, correct) {
            break;
        }
    }
}
