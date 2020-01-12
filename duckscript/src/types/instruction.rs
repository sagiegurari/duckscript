//! # instruction
//!
//! The instruction type.
//!

#[cfg(test)]
#[path = "./instruction_test.rs"]
mod instruction_test;

/// Defines common instruction capabilities
pub trait InstructionOperations {
    /// Returns true if this instruction has some actionable command to run
    fn is_actionable(&self) -> bool;
}

/// Preprocess instruction
#[derive(Debug, Clone)]
pub struct PreProcessInstruction {
    /// The command name
    pub command: Option<String>,
    /// The command arguments
    pub arguments: Option<Vec<String>>,
}

impl PreProcessInstruction {
    /// Creates and returns a new instance.
    pub fn new() -> PreProcessInstruction {
        PreProcessInstruction {
            command: None,
            arguments: None,
        }
    }
}

impl InstructionOperations for PreProcessInstruction {
    /// Returns true if this instruction has some actionable command to run
    fn is_actionable(&self) -> bool {
        return self.command.is_some();
    }
}

/// Runtime script instruction
#[derive(Debug, Clone)]
pub struct ScriptInstruction {
    /// The label tag
    pub label: Option<String>,
    /// The command output variable name
    pub output: Option<String>,
    /// The command name
    pub command: Option<String>,
    /// The command arguments
    pub arguments: Option<Vec<String>>,
}

impl ScriptInstruction {
    /// Creates and returns a new instance.
    pub fn new() -> ScriptInstruction {
        ScriptInstruction {
            label: None,
            output: None,
            command: None,
            arguments: None,
        }
    }
}

impl InstructionOperations for ScriptInstruction {
    /// Returns true if this instruction has some actionable command to run
    fn is_actionable(&self) -> bool {
        return self.command.is_some();
    }
}

/// Instruction Type - script, preprocess
#[derive(Debug, Clone)]
pub enum InstructionType {
    /// Empty instruction
    Empty,
    /// Preprocess instruction
    PreProcess(PreProcessInstruction),
    /// Runtime script instruction
    Script(ScriptInstruction),
}

/// Meta information for all instruction types
#[derive(Debug, Clone)]
pub struct InstructionMetaInfo {
    /// The line number
    pub line: Option<usize>,
    /// The source file/url/...
    pub source: Option<String>,
}

impl InstructionMetaInfo {
    /// Creates and returns a new instance.
    pub fn new() -> InstructionMetaInfo {
        InstructionMetaInfo {
            line: None,
            source: None,
        }
    }
}

/// Instruction data
#[derive(Debug, Clone)]
pub struct Instruction {
    /// Meta info
    pub meta_info: InstructionMetaInfo,
    /// The instruction
    pub instruction_type: InstructionType,
}

impl InstructionOperations for Instruction {
    /// Returns true if this instruction has some actionable command to run
    fn is_actionable(&self) -> bool {
        match self.instruction_type {
            InstructionType::Empty => false,
            InstructionType::PreProcess(ref value) => value.is_actionable(),
            InstructionType::Script(ref value) => value.is_actionable(),
        }
    }
}
