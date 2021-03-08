use duckscript::parser;
use duckscript::types::error::ScriptError;
use duckscript::types::instruction::{Instruction, InstructionType, ScriptInstruction};

pub(crate) fn lint_file(file: &str) -> Result<(), ScriptError> {
    match parser::parse_file(file) {
        Ok(instructions) => {
            println!("File: {} parsed correctly.", file);

            match lint_instructions(instructions) {
                Ok(_) => {
                    println!("No lint errors found in file: {}", file);
                    Ok(())
                }
                Err(error) => Err(error),
            }
        }
        Err(error) => Err(error),
    }
}

fn lint_instructions(instructions: Vec<Instruction>) -> Result<(), ScriptError> {
    for instruction in &instructions {
        let result = match &instruction.instruction_type {
            InstructionType::Script(ref script_instruction) => lint_instruction(script_instruction),
            _ => Ok(()),
        };

        match result {
            Err(error) => {
                return Err(ScriptError::Runtime(
                    error,
                    Some(instruction.meta_info.clone()),
                ))
            }
            _ => (),
        }
    }

    Ok(())
}

fn lint_instruction(script_instruction: &ScriptInstruction) -> Result<(), String> {
    if !is_lower_case(script_instruction.label.clone()) {
        Err("Labels should be all lowercase.".to_string())
    } else if !is_lower_case(script_instruction.command.clone()) {
        Err("Commands should be all lowercase.".to_string())
    } else if !is_lower_case(script_instruction.output.clone()) {
        Err("Output variable should be all lowercase.".to_string())
    } else {
        Ok(())
    }
}

fn is_lower_case(value: Option<String>) -> bool {
    match value {
        Some(text) => {
            let lower_case_text = text.to_lowercase();

            lower_case_text == text
        }
        None => true,
    }
}
