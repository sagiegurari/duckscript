use duckscript::types::instruction::{Instruction, InstructionType};
use std::cmp::min;

#[cfg(test)]
#[path = "./instruction_query_test.rs"]
mod instruction_query_test;

#[derive(Debug, Clone)]
pub(crate) struct Positions {
    pub(crate) middle: Vec<usize>,
    pub(crate) end: usize,
}

fn get_start(start: Option<usize>) -> usize {
    match start {
        Some(value) => value,
        None => 0,
    }
}

fn get_end(end: Option<usize>, instructions: &Vec<Instruction>) -> usize {
    match end {
        Some(value) => min(instructions.len(), value),
        None => instructions.len(),
    }
}

pub(crate) fn find_command(
    instructions: &Vec<Instruction>,
    name_or_alias: &Vec<String>,
    start: Option<usize>,
    end: Option<usize>,
    error_on_command: &Vec<String>,
) -> Result<Option<usize>, String> {
    if name_or_alias.is_empty() {
        Err("No command names/aliases provided for search.".to_string())
    } else {
        let start_index = get_start(start);
        let end_index = get_end(end, instructions);

        for line in start_index..end_index {
            let instruction = &instructions[line];

            match instruction.instruction_type {
                InstructionType::Script(ref script_instruction) => match script_instruction.command
                {
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
}

pub(crate) fn find_commands(
    instructions: &Vec<Instruction>,
    start_names: &Vec<String>,
    middle_names: &Vec<String>,
    end_names: &Vec<String>,
    start: Option<usize>,
    end: Option<usize>,
    allow_recursive: bool,
) -> Result<Option<Positions>, String> {
    if start_names.is_empty() || end_names.is_empty() {
        Err("No command names/aliases provided for search.".to_string())
    } else {
        let start_index = get_start(start);
        let end_index = get_end(end, instructions);

        let mut positions = Positions {
            middle: vec![],
            end: 0,
        };
        let mut skip_to = start_index;
        for line in start_index..end_index {
            if line >= skip_to {
                let instruction = &instructions[line];

                match instruction.instruction_type {
                    InstructionType::Script(ref script_instruction) => {
                        match script_instruction.command {
                            Some(ref command) => {
                                if middle_names.contains(command) {
                                    positions.middle.push(line);
                                } else if end_names.contains(command) {
                                    positions.end = line;
                                    return Ok(Some(positions));
                                } else if start_names.contains(command) {
                                    if allow_recursive {
                                        match find_commands(
                                            instructions,
                                            start_names,
                                            middle_names,
                                            end_names,
                                            Some(line + 1),
                                            Some(end_index),
                                            allow_recursive,
                                        ) {
                                            Ok(positions_options) => match positions_options {
                                                Some(sub_positions) => {
                                                    skip_to = sub_positions.end + 1;
                                                    ()
                                                }
                                                None => return Err(format!("Unsupported nested structure: {} found but end of structure not found.",command).to_string()),
                                            },
                                            Err(error) => return Err(error),
                                        };
                                    } else {
                                        return Err(format!(
                                            "Unsupported nested structure: {} found.",
                                            command
                                        )
                                        .to_string());
                                    }
                                }

                                ()
                            }
                            None => (),
                        }
                    }
                    _ => (),
                }
            }
        }

        Err("Missing end of structure.".to_string())
    }
}
