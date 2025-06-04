use std::env;
use std::io::{self, Write};

fn count_vowels(s: &str) -> usize {
    s.chars()
        .filter(|c: &char| "aeiouAEIOU".contains(*c))
        .count()
}

fn main() {
    println!("============================================");
    println!("Welcome to the Count Vowels Program!");
    println!("============================================\n\n");

    let args: Vec<String> = env::args().collect();

    let mut s = String::new();
    if args.len() == 1 {
        println!("Please enter a setence:\n");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut s).expect("Failed to read input");
        s = s.trim().to_string()
    } else {
        s = args[1].clone();
    }
    println!("Numbers of vowels:\n{}", count_vowels(&s));
    println!("============================================");
}
