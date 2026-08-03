use std::io::{self, Write};
use crate::{commands::Command, parser::{Parser, Token}};
use std::collections::HashMap;

pub struct Registry {
    commands: HashMap<String, Box<dyn Command>>
}

impl Registry {
    pub fn new() -> Self {
        Registry {commands: HashMap::new()}
    }
    pub fn register_command(&mut self, name: &str, command: Box<dyn Command>) {
        self.commands.insert(name.to_string(), command);
    }
    pub fn get_command(&self, name: &str) -> Option<&Box<dyn Command>> {
        self.commands.get(name)
    }
    pub fn command_exists(&self, name: &str) -> bool {
        self.commands.contains_key(name)
    }
}

enum State {
    Init,
    Running,
    Exiting
}

pub struct Engine<'a > {
    state: State,
    pub registry: &'a Registry
}


impl<'a > Engine<'a > {
    pub fn new(reg: &'a Registry) -> Self {
        Engine { state: State::Init, registry: reg }
    }
    pub fn run(&mut self) {
        self.state = State::Running;
        loop {
            if let State::Exiting = self.state {
                break;
            }

            let input: String = Engine::read_input();
            let tokens = Parser::parse(&input);

            if !tokens.is_empty() {
                let Token::Word(s) = &tokens[0];
                let args: Vec<&str> = tokens[1..].iter().map(|x| match x {
                    Token::Word(s) => s.as_str()
                }).collect();
                let res = self.exec(&s, &args);
                match res {
                    Ok(r) => {
                        if let Some(out) = r {
                            println!("{}", &out);
                        }
                    },
                    Err(e) => println!("{}: {}", &s, &e)
                }
            }

        }
    }
    fn read_input() -> String {
        let mut input = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        input.trim().to_string()
    }
    fn exec(&mut self, cmd_name: &str, args: &[&str]) -> Result<Option<String>, String> {
        if let Some(cmd) = self.registry.get_command(cmd_name) {
            cmd.exec(args, self)
        }
        else {
            Err(String::from("command not found"))
        }
    }
    pub fn exit(&mut self) {
        self.state = State::Exiting
    }
}