#[allow(unused_imports)]
use std::io::{self, Write, Read};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    let mut input = String::new();
    print!("$ ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input = input.trim();

    println!("{}: command not found", input);

    // let _ = std::io::stdin().read(&mut [0u8]).unwrap();
}
