//! # parser
//!
//! The duck script parser.
//!

#[cfg(test)]
#[path = "./parser_test.rs"]
mod parser_test;

use crate::io;
use crate::preprocessor;
use crate::types::error::{ErrorInfo, ScriptError};
use crate::types::instruction::{
    Instruction, InstructionMetaInfo, InstructionType, PreProcessInstruction, ScriptInstruction,
};

static COMMENT_PREFIX_STR: &str = "#";
static PRE_PROCESS_PREFIX: char = '!';
static LABEL_PREFIX: char = ':';

/// parses the file and returns a vector of instructions
pub fn parse_file(file: &str) -> Result<Vec<Instruction>, ScriptError> {
    let mut meta_info = InstructionMetaInfo::new();
    meta_info.source = Some(file.to_string());

    match io::read_text_file(file) {
        Ok(text) => parse_lines(&text, meta_info),
        Err(error) => Err(error),
    }
}

/// parses the provided script text and returns a vector of instructions
pub fn parse_text(text: &str) -> Result<Vec<Instruction>, ScriptError> {
    parse_lines(&text, InstructionMetaInfo::new())
}

fn parse_lines(
    lines: &str,
    meta_info: InstructionMetaInfo,
) -> Result<Vec<Instruction>, ScriptError> {
    let mut instructions = vec![];

    let mut line_number = 1;
    for line in lines.lines() {
        let mut line_meta_info = meta_info.clone();
        line_meta_info.line = Some(line_number);
        line_number = line_number + 1;

        match parse_line(&line, line_meta_info) {
            Ok(instruction) => {
                instructions.push(instruction.clone());

                match instruction.instruction_type {
                    InstructionType::PreProcess(_) => match preprocessor::run(&instruction) {
                        Ok(mut added_instructions) => instructions.append(&mut added_instructions),
                        Err(error) => return Err(error),
                    },
                    _ => (),
                }
            }
            Err(error) => return Err(error),
        };
    }

    Ok(instructions)
}

fn parse_line(line_text: &str, meta_info: InstructionMetaInfo) -> Result<Instruction, ScriptError> {
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
        Err(ScriptError {
            info: ErrorInfo::PreProcessNoCommandFound(meta_info),
        })
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
            Err(ScriptError {
                info: ErrorInfo::PreProcessNoCommandFound(meta_info),
            })
        } else {
            match parse_arguments(&meta_info, &line_text, index) {
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
        match find_label(&meta_info, &line_text, index) {
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
        index = match find_output_and_command(&meta_info, &line_text, index, &mut instruction) {
            Ok(next_index) => next_index,
            Err(error) => return Err(error),
        };

        match parse_arguments(&meta_info, &line_text, index) {
            Ok(arguments) => {
                instruction.arguments = arguments;

                let instruction_type = if instruction.label.is_none()
                    && instruction.output.is_none()
                    && instruction.command.is_none()
                {
                    InstructionType::Empty
                } else {
                    InstructionType::Script(instruction)
                };

                Ok(Instruction {
                    meta_info,
                    instruction_type,
                })
            }
            Err(error) => Err(error),
        }
    }
}

fn parse_arguments(
    meta_info: &InstructionMetaInfo,
    line_text: &Vec<char>,
    start_index: usize,
) -> Result<Option<Vec<String>>, ScriptError> {
    let mut arguments = vec![];

    let mut index = start_index;
    loop {
        match parse_next_argument(&meta_info, &line_text, index) {
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
    meta_info: &InstructionMetaInfo,
    line_text: &Vec<char>,
    start_index: usize,
) -> Result<(usize, Option<String>), ScriptError> {
    parse_next_value(&meta_info, &line_text, start_index, true, true, false)
}

fn parse_next_value(
    meta_info: &InstructionMetaInfo,
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
        let mut found_variable_prefix = false;
        for _i in index..end_index {
            let character = line_text[index];
            index = index + 1;

            if in_argument {
                if in_control {
                    if found_variable_prefix {
                        if character == '{' {
                            argument.push_str("\\${");
                            in_control = false;
                            found_variable_prefix = false;
                        } else {
                            return Err(ScriptError {
                                info: ErrorInfo::ControlWithoutValidValue(meta_info.clone()),
                            });
                        }
                    } else if character == '\\' || character == '"' {
                        argument.push(character);
                        in_control = false;
                    } else if character == 'n' {
                        argument.push('\n');
                        in_control = false;
                    } else if character == 'r' {
                        argument.push('\r');
                        in_control = false;
                    } else if character == 't' {
                        argument.push('\t');
                        in_control = false;
                    } else if character == '$' {
                        found_variable_prefix = true;
                    } else {
                        return Err(ScriptError {
                            info: ErrorInfo::ControlWithoutValidValue(meta_info.clone()),
                        });
                    }
                } else if character == '\\' {
                    if allow_control {
                        in_control = true;
                        found_variable_prefix = false;
                    } else {
                        return Err(ScriptError {
                            info: ErrorInfo::InvalidControlLocation(meta_info.clone()),
                        });
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
                        return Err(ScriptError {
                            info: ErrorInfo::InvalidQuotesLocation(meta_info.clone()),
                        });
                    }
                } else if character == '\\' {
                    if allow_control {
                        in_control = true;
                    } else {
                        return Err(ScriptError {
                            info: ErrorInfo::InvalidControlLocation(meta_info.clone()),
                        });
                    }
                } else {
                    argument.push(character);
                }
            }
        }

        if in_argument && !found_end && (in_control || using_quotes) {
            if in_control {
                Err(ScriptError {
                    info: ErrorInfo::ControlWithoutValidValue(meta_info.clone()),
                })
            } else {
                Err(ScriptError {
                    info: ErrorInfo::MissingEndQuotes(meta_info.clone()),
                })
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
    meta_info: &InstructionMetaInfo,
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
                match parse_next_value(&meta_info, &line_text, index, false, false, false) {
                    Ok(output) => {
                        let (next_index, value) = output;
                        index = next_index;

                        match value {
                            Some(label_value) => {
                                if label_value.is_empty() {
                                    return Err(ScriptError {
                                        info: ErrorInfo::EmptyLabel(meta_info.clone()),
                                    });
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
    meta_info: &InstructionMetaInfo,
    line_text: &Vec<char>,
    start_index: usize,
    instruction: &mut ScriptInstruction,
) -> Result<usize, ScriptError> {
    match parse_next_value(&meta_info, &line_text, start_index, false, false, true) {
        Ok(output) => {
            let (next_index, value) = output;

            if value.is_none() {
                Ok(next_index)
            } else {
                let mut index = next_index;
                let end_index = line_text.len();
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
                    match parse_next_value(&meta_info, &line_text, index, false, false, false) {
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
