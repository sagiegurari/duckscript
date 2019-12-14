//! # parser
//!
//! The duck script parser.
//!

#[cfg(test)]
#[path = "./parser_test.rs"]
mod parser_test;

use crate::instruction::{
    Instruction, InstructionMetaInfo, InstructionType, PreProcessInstruction,
};
use crate::types::DuckScriptError;

static COMMENT_PREFIX_STR: &str = "#";
static PRE_PROCESS_PREFIX: char = '!';
static LABEL_PREFIX: char = ':';

pub fn parse_line(
    line_text: &str,
    meta_info: InstructionMetaInfo,
) -> Result<Instruction, DuckScriptError> {
    let trimmed_text = line_text.trim();

    if trimmed_text.is_empty() || trimmed_text.starts_with(&COMMENT_PREFIX_STR) {
        Ok(Instruction {
            meta_info,
            instruction_type: InstructionType::Empty,
        })
    } else {
        let chars: Vec<char> = trimmed_text.chars().collect();

        if chars[0] == PRE_PROCESS_PREFIX {
            parse_pre_process_line(&chars, meta_info, 1)
        } else {
            parse_command_line(&chars, meta_info, 0)
        }
    }
}

fn parse_pre_process_line(
    line_text: &Vec<char>,
    meta_info: InstructionMetaInfo,
    start_index: usize,
) -> Result<Instruction, DuckScriptError> {
    if line_text.is_empty() {
        Err(DuckScriptError::PreProcessNoCommandFound)
    } else {
        let mut command = String::new();
        let mut index = start_index;
        let end_index = line_text.len();
        for i in index..end_index {
            let character = line_text[index];
            index = index + 1;

            if character == ' ' {
                if !command.is_empty() {
                    break;
                }
            } else {
                command.push(character);
            }
        }

        if command.is_empty() {
            Err(DuckScriptError::PreProcessNoCommandFound)
        } else {
            match parse_arguments(&line_text, index) {
                Ok(arguments) => {
                    let mut instruction = PreProcessInstruction::new();
                    instruction.command = Some(command);
                    instruction.arguments = arguments;

                    Ok(Instruction {
                        meta_info,
                        instruction_type: InstructionType::PreProcess(instruction),
                    })
                }
                Err(error) => Err(error),
            }
        }
    }
}

fn parse_command_line(
    line_text: &Vec<char>,
    meta_info: InstructionMetaInfo,
    start_index: usize,
) -> Result<Instruction, DuckScriptError> {
        let end_index = line_text.len();

    if line_text.is_empty() || start_index >= end_index {
        Ok(Instruction {
            meta_info,
            instruction_type: InstructionType::Empty,
        })
    } else {
        // search for label
        let mut index = start_index;
        if line_text[index] == LABEL_PREFIX {
            index=index+1;
            for i in index..end_index {
                let character = line_text[index];
            }
        }

        //TODO IMPL
        Ok(Instruction {
            meta_info,
            instruction_type: InstructionType::Empty,
        })
    }
}

fn parse_arguments(
    line_text: &Vec<char>,
    start_index: usize,
) -> Result<Option<Vec<String>>, DuckScriptError> {
    let mut arguments = vec![];

    let mut index = start_index;
    loop {
        match parse_next_argument(&line_text, index) {
            Ok(output) => {
                let (next_index, argument) = output;

                if argument.is_none() {
                    break;
                }

                arguments.push(argument.unwrap());
                index = next_index;
            }
            Err(error) => return Err(error),
        }
    }

    if arguments.is_empty() {
        Ok(None)
    } else {
        Ok(Some(arguments))
    }
}

fn parse_next_argument(
    line_text: &Vec<char>,
    start_index: usize,
) -> Result<(usize, Option<String>), DuckScriptError> {
    let end_index = line_text.len();

    if start_index >= end_index {
        Ok((start_index, None))
    } else {
        let mut argument = String::new();
        let mut index = start_index;
        let mut in_argument = false;
        let mut using_quotes = false;
        let mut in_control = false;
        let mut found_end = false;
        for i in index..end_index {
            let character = line_text[index];
            index = index + 1;

            if in_argument {
                if in_control {
                    if character == '\\' || character == '"' {
                        argument.push(character);
                        in_control = false;
                    } else {
                        return Err(DuckScriptError::ControlWithoutValidValue);
                    }
                } else if character == '\\' {
                    in_control = true;
                } else if using_quotes && character == '"' {
                    found_end = true;
                    break;
                } else if !using_quotes && character == ' ' {
                    index = index - 1;
                    found_end = true;
                    break;
                } else {
                    argument.push(character);
                }
            } else if character != ' ' {
                in_argument = true;

                if character == '"' {
                    using_quotes = true;
                } else if character == '\\' {
                    in_control = true;
                } else {
                    argument.push(character);
                }
            }
        }

        if in_argument && !found_end && (in_control || using_quotes) {
            if in_control {
                Err(DuckScriptError::ControlWithoutValidValue)
            } else {
                Err(DuckScriptError::MissingEndQuotes)
            }
        } else if argument.is_empty() {
            if using_quotes {
                Ok((index, Some(argument)))
            } else {
                Ok((index, None))
            }
        } else {
            Ok((index, Some(argument)))
        }
    }
}
