//! # context
//!
//! The runtime context structures.
//!

#[cfg(test)]
#[path = "./context_test.rs"]
mod context_test;

use crate::command::Commands;
use crate::instruction::Instruction;
use std::collections::HashMap;

/// The context structure
pub struct Context {
    /// The script instructions
    pub instructions: Option<Vec<Instruction>>,
    /// The runtime variables
    pub variables: HashMap<String, String>,
    /// The command implementations
    pub commands: Commands,
}

impl Context {
    /// Creates and returns a new instance.
    pub fn new() -> Context {
        Context {
            instructions: None,
            variables: HashMap::new(),
            commands: Commands::new(),
        }
    }
}
