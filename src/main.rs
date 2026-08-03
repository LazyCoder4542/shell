#[allow(unused_imports)]
use std::io::{self, Write, Read};
use codecrafters_shell::{Echo, Engine, Exit, Registry, Type};

fn main() {
    // Setup Registry
    let mut reg = Registry::new();
    reg.register_command("exit", Box::new(Exit {}));
    reg.register_command("echo", Box::new(Echo {}));
    reg.register_command("type", Box::new(Type {}));

    // Start Engine
    let mut eng = Engine::new(&reg);

    eng.run();
}