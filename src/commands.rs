use crate::engine::{Engine, Registry};
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