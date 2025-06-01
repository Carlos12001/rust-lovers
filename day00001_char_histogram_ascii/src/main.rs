use std::env;
use std::io::{self, Write};

const MAX_LEN: usize = 256;

fn char_histogram(input: &str, freq: &mut [u32; 26]) {
    for byte in input.bytes() {
        let index = if (b'A'..=b'Z').contains(&byte) {
            (byte - b'A') as usize
        } else if (b'a'..=b'z').contains(&byte) {
            (byte - b'a') as usize
        } else {
            continue;
        };
        freq[index] += 1;
    }
}

fn print_char_histogram(freq: &[u32; 26]) {
    for (i, &count) in freq.iter().enumerate() {
        if count == 0 {
            continue;
        }
        print!("{}: ", (b'a' + i as u8) as char);
        for _ in 0..count {
            print!("*");
        }
        println!();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input = String::new();
    println!("Welcome to char_histogram_ascii!!");

    if args.len() == 1 {
        println!("Enter input text: \n");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input = input.trim().to_string(); // remove trailing newline
    } else if args.len() == 2 {
        println!("Analyzing input from command-line argument...\n");

        if args[1].len() >= MAX_LEN {
            eprintln!(
                "Error: the input value exceeded {} characters",
                MAX_LEN - 1
            );
            std::process::exit(1);
        }

        input = args[1].clone();
    } else {
        eprintln!("Error: the program only accepts one optional argument");
        std::process::exit(1);
    }

    let mut freq = [0u32; 26];
    char_histogram(&input, &mut freq);
    print_char_histogram(&freq);
}
