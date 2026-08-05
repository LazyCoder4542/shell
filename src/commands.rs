use crate::engine::{Engine, Registry};
use std::env;
use std::path::{self};

pub trait Command {
    fn exec(&self, _args: &[&str], _engine: &mut Engine) -> Result<Option<String>, String> {
        Ok(None)
    }
}

pub struct Exit {}

impl Command for Exit {
    fn exec(&self, _args: &[&str], _engine: &mut Engine) -> Result<Option<String>, String> {
        if _args.len() > 1 {

            return Err(String::from("too many arguments"));
        }
        _engine.exit();
        Ok(None)
    }
}

pub struct Echo {}

impl Command for Echo {
    fn exec(&self, _args: &[&str], _engine: &mut Engine) -> Result<Option<String>, String> {
        Ok(Some(_args.join(" ")))
    }
}

pub struct Type {}

impl Command for Type {
    fn exec(&self, _args: &[&str], _engine: &mut Engine) -> Result<Option<String>, String> {
        if _args.is_empty() {
            return Ok(None)
        }

        let mut out: Vec<String> = Vec::with_capacity(_args.len());
        for arg in _args.iter() {
            if _engine.registry.command_exists(arg) {
                out.push(format!("{} is a shell builtin", arg));
            }
            else if let Some(p) = Registry::get_exec(arg) {
                out.push(format!("{} is {}", arg, &p.to_string_lossy()));
            }
            else {
                out.push(format!("{}: not found", arg));
            }
        }
        Ok(Some(out.join("\n")))
    }
}

pub struct Pwd {}

impl Command for Pwd {
    fn exec(&self, _args: &[&str], _engine: &mut Engine) -> Result<Option<String>, String> {
        if !_args.is_empty() {
            return Err(String::from("too many arguments"));
        }
        Ok(Some(env::current_dir().unwrap().to_string_lossy().to_string()))
    }
}

pub struct Cd {}

impl Command for Cd {
    fn exec(&self, _args: &[&str], _engine: &mut Engine) -> Result<Option<String>, String> {
        if _args.len() > 1 {
            return Err(String::from("too many arguments"));
        }
        let home_dir = env::home_dir().unwrap().to_string_lossy().to_string();
        let sep_char = path::MAIN_SEPARATOR_STR;
        let new_path: &str = match _args.get(0) {
            Some(val) => {val},
            None => &home_dir
        };
        let parts: Option<(&str, &str)> = new_path.split_once(sep_char);
        let new_path: String = match parts {
            Some(x) => {
                format!("{}{}{}", if x.0 == "~" {&home_dir} else {x.0}, sep_char, x.1)
            },
            None => {
                if new_path == "~" {String::from(home_dir)} else {new_path.to_string()}
            }
        };
        let p = path::Path::new(&new_path);
        match env::set_current_dir(p) {
            Ok(()) => {Ok(None)}
            Err(_e) => {Err(format!("{}: No such file or directory", p.to_string_lossy()))}
        }
    }
    }