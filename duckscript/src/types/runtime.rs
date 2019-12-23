//! # runtime
//!
//! The runtime context structures.
//!

#[cfg(test)]
#[path = "./runtime_test.rs"]
mod runtime_test;

use crate::types::command::Commands;
use crate::types::instruction::Instruction;
use std::collections::HashMap;

/// The context structure
pub struct Context {
    /// The runtime variables
    pub variables: HashMap<String, String>,
    /// The runtime state
    pub state: HashMap<String, String>,
    /// The command implementations
    pub commands: Commands,
}

impl Context {
    /// Creates and returns a new instance.
    pub fn new() -> Context {
        Context {
            variables: HashMap::new(),
            state: HashMap::new(),
            commands: Commands::new(),
        }
    }
}

/// The runtime structure
pub struct Runtime {
    /// The script instructions
    pub instructions: Option<Vec<Instruction>>,
    /// The runtime context
    pub context: Context,
}

impl Runtime {
    /// Creates and returns a new instance.
    pub fn new(context: Context) -> Runtime {
        Runtime {
            instructions: None,
            context,
        }
    }
}
