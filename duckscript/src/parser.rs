//! # parser
//!
//! The duck script parser.
//!

#[cfg(test)]
#[path = "./parser_test.rs"]
mod parser_test;

use crate::instruction::{
    Instruction, InstructionMetaInfo, InstructionType, PreProcessInstruction, ScriptInstruction,
};
use crate::types::ScriptError;

static COMMENT_PREFIX_STR: &str = "#";
static PRE_PROCESS_PREFIX: char = '!';
static LABEL_PREFIX: char = ':';

pub fn parse_line(
    line_text: &str,
    meta_info: InstructionMetaInfo,
) -> Result<Instruction, ScriptError> {
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
) -> Result<Instruction, ScriptError> {
    if line_text.is_empty() {
        Err(ScriptError::PreProcessNoCommandFound)
    } else {
        let mut command = String::new();
        let mut index = start_index;
        let end_index = line_text.len();
        for _i in index..end_index {
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
            Err(ScriptError::PreProcessNoCommandFound)
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
) -> Result<Instruction, ScriptError> {
    let end_index = line_text.len();

    if line_text.is_empty() || start_index >= end_index {
        Ok(Instruction {
            meta_info,
            instruction_type: InstructionType::Empty,
        })
    } else {
        // search for label
        let mut index = start_index;
        let mut instruction = ScriptInstruction::new();
        match find_label(&line_text, index) {
            Ok(output) => {
                let (next_index, value) = output;
                index = next_index;

                if value.is_some() {
                    instruction.label = value.clone();
                }
            }
            Err(error) => return Err(error),
        };

        // find output variable and command
        index = match find_output_and_command(&line_text, index, &mut instruction) {
            Ok(next_index) => next_index,
            Err(error) => return Err(error),
        };

        match parse_arguments(&line_text, index) {
            Ok(arguments) => {
                instruction.arguments = arguments;

                Ok(Instruction {
                    meta_info,
                    instruction_type: InstructionType::Script(instruction),
                })
            }
            Err(error) => Err(error),
        }
    }
}

fn parse_arguments(
    line_text: &Vec<char>,
    start_index: usize,
) -> Result<Option<Vec<String>>, ScriptError> {
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
) -> Result<(usize, Option<String>), ScriptError> {
    parse_next_value(&line_text, start_index, true, true, false)
}

fn parse_next_value(
    line_text: &Vec<char>,
    start_index: usize,
    allow_quotes: bool,
    allow_control: bool,
    stop_on_equals: bool,
) -> Result<(usize, Option<String>), ScriptError> {
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
        for _i in index..end_index {
            let character = line_text[index];
            index = index + 1;

            if in_argument {
                if in_control {
                    if character == '\\' || character == '"' {
                        argument.push(character);
                        in_control = false;
                    } else {
                        return Err(ScriptError::ControlWithoutValidValue);
                    }
                } else if character == '\\' {
                    if allow_control {
                        in_control = true;
                    } else {
                        return Err(ScriptError::InvalidControlLocation);
                    }
                } else if using_quotes && character == '"' {
                    found_end = true;
                    break;
                } else if !using_quotes
                    && (character == ' '
                        || character == '#'
                        || (stop_on_equals && character == '='))
                {
                    if character == ' ' || character == '=' {
                        index = index - 1;
                    } else if character == '#' {
                        index = end_index;
                    }
                    found_end = true;
                    break;
                } else {
                    argument.push(character);
                }
            } else if character == '#' {
                index = end_index;
                break;
            } else if character != ' ' {
                in_argument = true;

                if character == '"' {
                    if allow_quotes {
                        using_quotes = true;
                    } else {
                        return Err(ScriptError::InvalidQuotesLocation);
                    }
                } else if character == '\\' {
                    if allow_control {
                        in_control = true;
                    } else {
                        return Err(ScriptError::InvalidControlLocation);
                    }
                } else {
                    argument.push(character);
                }
            }
        }

        if in_argument && !found_end && (in_control || using_quotes) {
            if in_control {
                Err(ScriptError::ControlWithoutValidValue)
            } else {
                Err(ScriptError::MissingEndQuotes)
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

fn find_label(
    line_text: &Vec<char>,
    start_index: usize,
) -> Result<(usize, Option<String>), ScriptError> {
    let end_index = line_text.len();

    if start_index >= end_index {
        Ok((start_index, None))
    } else {
        let mut label = None;
        let mut index = start_index;
        for _i in index..end_index {
            let character = line_text[index];
            index = index + 1;

            if character == LABEL_PREFIX {
                match parse_next_value(&line_text, index, false, false, false) {
                    Ok(output) => {
                        let (next_index, value) = output;
                        index = next_index;

                        match value {
                            Some(label_value) => {
                                if label_value.is_empty() {
                                    return Err(ScriptError::EmptyLabel);
                                }

                                let mut text = String::new();
                                text.push(LABEL_PREFIX);
                                text.push_str(&label_value);

                                label = Some(text);
                            }
                            None => (),
                        };

                        break;
                    }
                    Err(error) => return Err(error),
                };
            } else if character != ' ' {
                index = index - 1;
                break;
            }
        }

        Ok((index, label))
    }
}

fn find_output_and_command(
    line_text: &Vec<char>,
    start_index: usize,
    instruction: &mut ScriptInstruction,
) -> Result<usize, ScriptError> {
    let end_index = line_text.len();

    match parse_next_value(&line_text, start_index, false, false, true) {
        Ok(output) => {
            let (next_index, value) = output;

            if value.is_none() {
                Ok(next_index)
            } else {
                let mut index = next_index;
                for _i in index..end_index {
                    let character = line_text[index];
                    index = index + 1;

                    if character != ' ' {
                        if character == '=' {
                            instruction.output = value.clone();
                        }

                        break;
                    }
                }

                if instruction.output.is_some() {
                    match parse_next_value(&line_text, index, false, false, false) {
                        Ok(output) => {
                            let (next_index, value) = output;

                            if value.is_none() {
                                Ok(index)
                            } else {
                                instruction.command = value.clone();
                                Ok(next_index)
                            }
                        }
                        Err(error) => Err(error),
                    }
                } else {
                    instruction.command = value.clone();

                    Ok(next_index)
                }
            }
        }
        Err(error) => Err(error),
    }
}
