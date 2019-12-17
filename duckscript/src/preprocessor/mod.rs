//! # preprocessor
//!
//! The pre processor is invoked on parse time by the script parser.
//! Currently it provide only limited amount of commands and it is not meant to be extendable.
//!

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

mod include_files_preprocessor;
mod print_preprocessor;

use crate::error::{ErrorInfo, ScriptError};
use crate::instruction::{Instruction, InstructionType};

pub(crate) fn run(instruction: &Instruction) -> Result<Vec<Instruction>, ScriptError> {
    match instruction.instruction_type {
        InstructionType::PreProcess(ref instruction_type) => match &instruction_type.command {
            Some(command) => match command.as_ref() {
                "print" => {
                    print_preprocessor::run(&instruction_type.arguments);
                    Ok(vec![])
                }
                "include_files" => include_files_preprocessor::run(
                    &instruction_type.arguments,
                    &instruction.meta_info,
                ),
                _ => Err(ScriptError {
                    info: ErrorInfo::UnknownPreProcessorCommand(instruction.meta_info.clone()),
                }),
            },
            None => Err(ScriptError {
                info: ErrorInfo::PreProcessNoCommandFound(instruction.meta_info.clone()),
            }),
        },
        _ => Ok(vec![]),
    }
}
