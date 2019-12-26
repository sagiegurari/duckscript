use duckscript::types::instruction::{Instruction, InstructionType};

#[cfg(test)]
#[path = "./instruction_query_test.rs"]
mod instruction_query_test;

pub(crate) fn find_command(
    instructions: &Vec<Instruction>,
    name_or_alias: &Vec<String>,
    start: Option<usize>,
    end: Option<usize>,
    error_on_command: &Vec<String>,
) -> Result<Option<usize>, String> {
    let start_index = match start {
        Some(value) => value,
        None => 0,
    };
    let end_index = match end {
        Some(value) => {
            if value > instructions.len() {
                instructions.len()
            } else {
                value
            }
        }
        None => instructions.len(),
    };

    for line in start_index..end_index {
        let instruction = &instructions[line];

        match instruction.instruction_type {
            InstructionType::Script(ref script_instruction) => match script_instruction.command {
                Some(ref command) => {
                    if name_or_alias.contains(command) {
                        return Ok(Some(line));
                    } else if error_on_command.contains(command) {
                        return Err(command.to_string());
                    }

                    ()
                }
                None => (),
            },
            _ => (),
        }
    }

    Ok(None)
}
