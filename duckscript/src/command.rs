//! # command
//!
//! The command trait and access module.
//!

#[cfg(test)]
#[path = "./command_test.rs"]
mod command_test;

use crate::context::Context;
use crate::instruction::Instruction;
use std::collections::HashMap;

pub trait Command {
    /// Runs the given instruction
    fn run(&self, context: &mut Context, instruction: &Instruction);
}

/// Holds and enables access to the runtime commands implementations
pub struct Commands {
    /// mapping between command names to implementations
    commands: HashMap<String, Box<dyn Command>>,
    /// mapping between aliases to command names
    aliases: HashMap<String, String>,
}

impl Commands {
    /// Creates and returns a new instance.
    pub fn new() -> Commands {
        Commands {
            commands: HashMap::new(),
            aliases: HashMap::new(),
        }
    }

    /// Return the command based on the given command name/alias
    pub fn get(&self, name: &str) -> Option<&Box<dyn Command>> {
        let command_name = match self.aliases.get(name) {
            Some(value) => value,
            None => name,
        };

        self.commands.get(command_name)
    }
}
