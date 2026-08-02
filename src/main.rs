#[allow(unused_imports)]
use std::io::{self, Write, Read};

fn main() {
    loop {
        // Read
        let mut input = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim();

        // Eval
        // here...

        // Print
        println!("{}: command not found", input);


        // Repeat
    }
    // let _ = std::io::stdin().read(&mut [0u8]).unwrap();
}
