use std::env;
use std::io::{self, Write};

fn sum_even_digits(mut n: u32) -> u32 {
    let mut sum = 0;
    while n > 0 {
        let digit = n % 10;
        sum += if digit % 2 == 0 { digit } else { 0 };
        n /= 10;
    }
    sum
}

fn main() {
    println!("============================================");
    println!("Welcome to the Sum of Even Digits Calculator!");
    println!("============================================\n\n");

    let args: Vec<String> = env::args().collect();
    let mut input = String::new();

    if args.len() > 1 {
        input = args[1].clone();
    } else {
        println!("This program calculates the sum of even digits in a given input string.\n");
        println!("Please enter a string of digits (e.g., '1234567890'):\n");
        io::stdout().flush().unwrap(); // Clear the output buffer

        io::stdin()
            .read_line(&mut input)
            .expect("Falied to read input");
        input = input.trim().to_string(); // Remove any trailing newline or spaces
    }
    match input.parse::<u32>() {
        Ok(n) => {
            let sum = sum_even_digits(n);
            println!("\nThe sum of even digits in '{}' is:\n{}", input, sum);
        }
        Err(_) => {
            eprintln!(
                "Error: Invalid input. Please enter a valid positive integer."
            );
        }
    }
    println!("\n\n============================================");
}
