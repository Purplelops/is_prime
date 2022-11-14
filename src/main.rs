use std::io;
use std::process;

fn main() {
    println!("Prime number checker.");
    println!("Please input the number you want to check.");

    // Create a new empty string
    let mut number = String::new();

    // Take user input and store it in the string
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
    
    // Convert string into a number
    let number: u128 = number.trim().parse()
        .expect("Something went wrong");

    // Check if number is divisible by 2
    if number % 2 == 0 {
        println!("Not prime! It is divisible by 2");
        process::exit(0);
    }

    let mut divisor: u128 = 3;

    // Loop over all numbers less than half the number the user picked
    loop {
        // If the remainder is zero, then it's not prime
        if number % divisor == 0 {
            println!("Not prime! It is divisible by {divisor}");
            break;
        }
        else {
            // Check if number is more than half of the number the user picked
            if divisor >= number / 2 {
                println!("{number} is prime!");
                break;
            }
            else {
                // Add two to the divisor, because a divisor of a prime can't be even
                divisor += 2;
            }
        }
    }
}
