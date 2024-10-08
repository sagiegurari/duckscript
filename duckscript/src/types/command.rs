//! # command
//!
//! The command trait and access module.
//!

#[cfg(test)]
#[path = "./command_test.rs"]
mod command_test;

use crate::types::env::Env;
use crate::types::error::ScriptError;
use crate::types::instruction::Instruction;
use crate::types::runtime::StateValue;
use std::collections::HashMap;

/// Holdes entire context (args, variables, state, etc...)
pub struct CommandInvocationContext<'a> {
    /// The command arguments
    pub arguments: Vec<String>,
    /// Internal state which is only used by commands to store/pull data
    pub state: &'a mut HashMap<String, StateValue>,
    /// All script variables
    pub variables: &'a mut HashMap<String, String>,
    /// The output variable name (if defined)
    pub output_variable: Option<String>,
    /// The entire list of instructions which make up the currently running script
    pub instructions: &'a Vec<Instruction>,
    /// The currently known commands
    pub commands: &'a mut Commands,
    /// The current instruction line number (global line number after including all scripts into one global script)
    pub line: usize,
    /// The current runtime env with access to out/err writers, etc...
    pub env: &'a mut Env,
}

/// GoTo type value
#[derive(Debug, Clone)]
pub enum GoToValue {
    /// label target
    Label(String),
    /// Line number
    Line(usize),
}

/// Command execution result
#[derive(Debug, Clone)]
pub enum CommandResult {
    /// Holds the command output and tells the runner to continue to next instruction
    Continue(Option<String>),
    /// Holds the command output and tells the runner to jump to the provided label
    GoTo(Option<String>, GoToValue),
    /// Holds the error message and the meta info of the instruction that caused it
    Error(String),
    /// Holds the critical error message and the meta info of the instruction that caused it
    Crash(String),
    /// Holds the command output and tells the runner to stop the script execution
    Exit(Option<String>),
}

/// Defines the command capabilities
pub trait Command {
    /// The full command name which can be used to invoke this command.
    fn name(&self) -> String;

    /// A list of aliases that can also be used to invoke this command.
    fn aliases(&self) -> Vec<String> {
        vec![]
    }

    /// Command documentation.
    fn help(&self) -> String {
        format!("No documentation found for command: {}", self.name())
    }

    /// Clones the command and returns a box reference.
    fn clone_and_box(&self) -> Box<dyn Command>;

    /// Run the instruction.
    fn run(&self, mut _context: CommandInvocationContext) -> CommandResult {
        CommandResult::Crash(format!("Not implemented for command: {}", &self.name()).to_string())
    }
}

/// Defines a box reference for a command.
pub type CommandBox = Box<dyn Command>;

impl Clone for Box<dyn Command> {
    fn clone(&self) -> Box<dyn Command> {
        self.clone_and_box()
    }
}

/// Holds and enables access to the runtime commands implementations
#[derive(Clone)]
pub struct Commands {
    /// mapping between command names to implementations
    pub commands: HashMap<String, CommandBox>,
    /// mapping between aliases to command names
    pub aliases: HashMap<String, String>,
}

impl Commands {
    /// Creates and returns a new instance.
    pub fn new() -> Commands {
        Commands {
            commands: HashMap::new(),
            aliases: HashMap::new(),
        }
    }

    /// Adds a new command definition.
    /// It will fail in case another command already defined the same name/aliases
    pub fn set(&mut self, command: CommandBox) -> Result<(), ScriptError> {
        let name = command.name();
        let aliases = command.aliases();

        if self.commands.contains_key(&name) {
            return Err(ScriptError::Initialization(format!(
                "Command: {} already defined.",
                &name
            )));
        }

        for alias in &aliases {
            if self.aliases.contains_key(alias) {
                return Err(ScriptError::Initialization(format!(
                    "Alias: {} for command: {} already defined.",
                    &alias, &name
                )));
            }
        }

        self.commands.insert(name.clone(), command);
        self.aliases.remove(&name);

        for alias in &aliases {
            self.aliases.insert(alias.to_string(), name.clone());
        }

        Ok(())
    }

    /// Return the command based on the given command name/alias
    pub fn get(&self, name: &str) -> Option<&CommandBox> {
        let command_name = match self.aliases.get(name) {
            Some(ref value) => value,
            None => name,
        };

        match self.commands.get(command_name) {
            Some(ref value) => Some(value),
            None => None,
        }
    }

    /// Return true if the command based on the given command name/alias exists
    pub fn exists(&self, name: &str) -> bool {
        let command = self.get(name);

        command.is_some()
    }

    /// Return the command based on the given command name/alias.
    /// It will also remove it in the process.
    pub fn get_for_use(&mut self, name: &str) -> Option<CommandBox> {
        let command_name = match self.aliases.get(name) {
            Some(ref value) => value,
            None => name,
        };

        match self.commands.get(command_name) {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }

    /// Returns all the command names currently registered
    pub fn get_all_command_names(&self) -> Vec<String> {
        let mut names = vec![];

        for key in self.commands.keys() {
            names.push(key.to_string());
        }

        names.sort();

        names
    }

    /// Removes the requested command.
    pub fn remove(&mut self, name: &str) -> bool {
        let command_name = match self.aliases.get(name) {
            Some(ref value) => value,
            None => name,
        };

        match self.commands.remove(command_name) {
            Some(command) => {
                let aliases = command.aliases();
                for alias in &aliases {
                    self.aliases.remove(alias);
                }

                true
            }
            None => false,
        }
    }
}
