//! # runner
//!
//! The main entry point which enables running scripts.
//!

#[cfg(test)]
#[path = "./runner_test.rs"]
mod runner_test;

use crate::parser;
use crate::types::command::CommandResult;
use crate::types::error::{ErrorInfo, ScriptError};
use crate::types::instruction::{Instruction, InstructionType, ScriptInstruction};
use crate::types::runtime::{Context, Runtime};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn run_script(text: &str, context: Context) -> Result<(), ScriptError> {
    match parser::parse_text(text) {
        Ok(instructions) => run(instructions, context),
        Err(error) => Err(error),
    }
}

pub fn run_script_file(file: &str, context: Context) -> Result<(), ScriptError> {
    match parser::parse_file(file) {
        Ok(instructions) => run(instructions, context),
        Err(error) => Err(error),
    }
}

fn run(instructions: Vec<Instruction>, context: Context) -> Result<(), ScriptError> {
    let mut runtime = Runtime::new(context);
    runtime.instructions = Some(instructions);

    run_instructions(runtime, 0)
}

fn run_instructions(mut runtime: Runtime, start_at: usize) -> Result<(), ScriptError> {
    let mut line = start_at;

    loop {
        let instruction_option = match runtime.instructions {
            Some(ref instructions) => {
                if instructions.len() > line {
                    Some(instructions[line].clone())
                } else {
                    break;
                }
            }
            None => break,
        };

        let (command_result, output_variable) =
            match run_instruction(&mut runtime, instruction_option) {
                Ok(results) => results,
                Err(error) => return Err(error),
            };

        match command_result {
            CommandResult::Exit(output) => {
                update_output(&mut runtime, output_variable, output);

                break;
            }
            CommandResult::Error(error, meta_info) => {
                return Err(ScriptError {
                    info: ErrorInfo::Runtime(error, meta_info),
                });
            }
            CommandResult::Continue(output) => {
                update_output(&mut runtime, output_variable, output);

                line = line + 1;

                ()
            }
            CommandResult::GoTo(output, label) => {
                update_output(&mut runtime, output_variable, output);

                line = line + 1; //TODO USE LABEL
                ()
            }
        };
    }

    Ok(())
}

fn update_output(runtime: &mut Runtime, output_variable: Option<String>, output: Option<String>) {
    if output_variable.is_some() {
        match output {
            Some(value) => runtime
                .context
                .variables
                .insert(output_variable.unwrap(), value),
            None => runtime.context.variables.remove(&output_variable.unwrap()),
        };
    }
}

fn run_instruction(
    runtime: &mut Runtime,
    instruction_option: Option<Instruction>,
) -> Result<(CommandResult, Option<String>), ScriptError> {
    let rc_context = Rc::new(RefCell::new(&runtime.context));

    let mut output_variable = None;
    let command_result = match instruction_option {
        Some(ref instruction) => match instruction.instruction_type {
            InstructionType::Empty => CommandResult::Continue(None),
            InstructionType::PreProcess(_) => CommandResult::Continue(None),
            InstructionType::Script(ref script_instruction) => {
                output_variable = script_instruction.output.clone();

                match script_instruction.command {
                    Some(ref command) => {
                        let mut command_instance_box = None;
                        {
                            println!("command: {}", &command); //todo rmove
                            let context = rc_context.clone();
                            match context.borrow().commands.get(command) {
                                Some(command_box) => {
                                    command_instance_box = Some(command_box.clone())
                                }
                                _ => (),
                            };
                        }

                        match command_instance_box {
                            Some(ref command_instance_box) => {
                                let command_arguments =
                                    bind_command_arguments(rc_context.clone(), &script_instruction);
                                let command_instance = *command_instance_box;
                                command_instance.run(
                                    rc_context.clone(),
                                    command_arguments,
                                    &instruction.meta_info,
                                )
                            }
                            None => CommandResult::Error(
                                format!("Command: {} not found.", &command),
                                instruction.meta_info.clone(),
                            ),
                        }
                    }
                    None => CommandResult::Continue(None),
                }
            }
        },
        None => CommandResult::Continue(None),
    };
    println!("command_result... {:?}", &command_result); //todo rmove

    Ok((command_result, output_variable))
}

fn bind_command_arguments(
    rc_context: Rc<RefCell<&Context>>,
    instruction: &ScriptInstruction,
) -> Vec<String> {
    let context = rc_context.borrow();
    let variables = &context.variables;
    let mut arguments = vec![];

    match instruction.arguments {
        Some(ref arguments_ref) => {
            for argument in arguments_ref {
                let value = expand_by_wrapper(&argument, "${", '}', variables);
                arguments.push(value);
            }
        }
        None => (),
    };

    arguments
}

fn should_break_key(value: char) -> bool {
    value == ' ' || value == '\n' || value == '\t' || value == '\r' || value == '='
}

fn expand_by_wrapper(
    value: &str,
    prefix: &str,
    suffix: char,
    variables: &HashMap<String, String>,
) -> String {
    let mut value_string = String::new();

    let prefix_length = prefix.len();
    let mut prefix_index = 0;
    let prefix_chars: Vec<char> = prefix.chars().collect();

    let mut found_prefix = false;
    let mut key = String::new();
    for next_char in value.chars() {
        if !found_prefix {
            if next_char == prefix_chars[prefix_index] {
                prefix_index = prefix_index + 1;

                if prefix_index == prefix_length {
                    found_prefix = true;
                    prefix_index = 0;
                    key.clear();
                }
            } else {
                if prefix_index > 0 {
                    value_string.push_str(&prefix[..prefix_index]);
                    prefix_index = 0;
                }
                value_string.push(next_char);
            }
        } else if next_char == suffix {
            match variables.get(&key) {
                Some(variable_value) => value_string.push_str(&variable_value),
                _ => (),
            };

            key.clear();
            found_prefix = false;
        } else if should_break_key(next_char) {
            value_string.push_str(&prefix);
            value_string.push_str(&key);
            value_string.push(next_char);

            key.clear();
            found_prefix = false;
        } else {
            key.push(next_char);
        }
    }

    if key.len() > 0 {
        value_string.push_str(&prefix);
        value_string.push_str(&key);
    }

    value_string
}
