//! # command
//!
//! The command trait and access module.
//!

#[cfg(test)]
#[path = "./command_test.rs"]
mod command_test;

use crate::types::error::{ErrorInfo, ScriptError};
use crate::types::instruction::InstructionMetaInfo;
use crate::types::runtime::Context;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Command execution result
#[derive(Debug, Clone)]
pub enum CommandResult {
    Continue(Option<String>),
    GoTo(Option<String>, String),
    Error(String, InstructionMetaInfo),
    Exit(Option<String>),
}

/// Defines the command capabilities
pub trait Command {
    fn name(&self) -> String;

    fn aliases(&self) -> Vec<String>;

    /// Runs the given instruction
    fn run(
        &self,
        context: Rc<RefCell<&Context>>,
        arguments: Vec<String>,
        meta_info: &InstructionMetaInfo,
    ) -> CommandResult;
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

    pub fn set(&mut self, command: Box<dyn Command>) -> Result<(), ScriptError> {
        let name = command.name();
        let aliases = command.aliases();

        if self.commands.contains_key(&name) {
            return Err(ScriptError {
                info: ErrorInfo::Initialization(format!("Command: {} already defined.", &name)),
            });
        }

        for alias in &aliases {
            if self.aliases.contains_key(alias) {
                return Err(ScriptError {
                    info: ErrorInfo::Initialization(format!(
                        "Alias: {} for command: {} already defined.",
                        &alias, &name
                    )),
                });
            }
        }

        self.commands.insert(name.clone(), command);

        for alias in &aliases {
            self.aliases.insert(alias.to_string(), name.clone());
        }

        Ok(())
    }

    /// Return the command based on the given command name/alias
    pub fn get(&self, name: &str) -> Option<&Box<dyn Command>> {
        let command_name = match self.aliases.get(name) {
            Some(value) => value,
            None => name,
        };

        match self.commands.get(command_name) {
            Some(ref value) => Some(value.clone()),
            None => None,
        }
    }
}
