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

/// enum defining what values can be stored in the state map
#[derive(Debug, Clone)]
pub enum StateValue {
    /// boolean value
    Boolean(bool),
    /// signed number
    Number(isize),
    /// unsigned number
    UnsignedNumber(usize),
    /// signed number
    Number32Bit(i32),
    /// unsigned number
    UnsignedNumber32Bit(u32),
    /// signed number
    Number64Bit(i64),
    /// unsigned number
    UnsignedNumber64Bit(u64),
    /// textual value
    String(String),
    /// byte (u8) array
    ByteArray(Vec<u8>),
    /// list
    List(Vec<StateValue>),
    /// sub state value
    SubState(HashMap<String, StateValue>),
}

/// The context structure
pub struct Context {
    /// The runtime variables
    pub variables: HashMap<String, String>,
    /// The runtime state
    pub state: HashMap<String, StateValue>,
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
    /// Label to line number mapping
    pub label_to_line: HashMap<String, usize>,
    /// The runtime context
    pub context: Context,
}

impl Runtime {
    /// Creates and returns a new instance.
    pub fn new(context: Context) -> Runtime {
        Runtime {
            instructions: None,
            label_to_line: HashMap::new(),
            context,
        }
    }
}
