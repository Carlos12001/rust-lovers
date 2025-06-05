use std::env;
use std::io::{self, Write};
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut name = String::new();

    if args.len() == 1 {
        println!("Enter your name:\n");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut name)
            .expect("Error: to read line.");
        name = name.trim().to_string();
    } else {
        name = args[1].clone();
    }
    println!("Your name is: {}\n", name);
}
