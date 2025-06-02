use std::env;
use std::io::{self, Write};

fn reverse_words(sentence: &str) -> String {
    sentence
        .split_whitespace()
        .rev()
        .collect::<Vec<&str>>()
        .join(" ")
}

fn main() {
    println!("============================================");
    println!("Welcome to the Reverse Words in Sentence Program!");
    println!("============================================\n\n");

    let args: Vec<String> = env::args().collect();

    let mut input = String::new();

    if args.len() > 1 {
        input = args[1].clone();
    } else {
        println!("Please enter a setence:\n");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        input = input.trim().to_string();
    }
    let result = reverse_words(&input);
    println!("Result:\n{}", result);
}
