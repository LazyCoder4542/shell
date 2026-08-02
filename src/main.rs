#[allow(unused_imports)]
use std::io::{self, Write, Read};

fn exit(args: &[&str]) -> Result<String, String> {
    if args.len() > 1 {
        Err(String::from("too many arguments"))
    }
    else {
        Ok(String::new())
    }
}

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
        let args: Vec<&str> = input.split(' ').collect();

        // Eval
        if args[0] == "exit" {
            match exit(&args[1..]) {
                Ok(out) => {
                    println!("{}", out);
                    break;
                },
                Err(e) => println!("exit: {e}")
            }
        }
        else if args[0] == "echo" {
            println!("{}", args[1..].join(" "));
        }
        else {
            println!("{}: command not found", args[0]);

        }

        // Repeat
    }
    // let _ = std::io::stdin().read(&mut [0u8]).unwrap();
}
