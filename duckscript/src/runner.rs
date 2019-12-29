//! # runner
//!
//! The main entry point which enables running scripts.
//!

#[cfg(test)]
#[path = "./runner_test.rs"]
mod runner_test;

use crate::expansion;
use crate::parser;
use crate::types::command::{CommandResult, Commands, GoToValue};
use crate::types::error::{ErrorInfo, ScriptError};
use crate::types::instruction::{Instruction, InstructionType, ScriptInstruction};
use crate::types::runtime::{Context, Runtime, StateValue};
use std::collections::HashMap;

/// Executes the provided script with the given context
pub fn run_script(text: &str, context: Context) -> Result<Context, ScriptError> {
    match parser::parse_text(text) {
        Ok(instructions) => run(instructions, context),
        Err(error) => Err(error),
    }
}

/// Executes the provided script file with the given context
pub fn run_script_file(file: &str, context: Context) -> Result<Context, ScriptError> {
    match parser::parse_file(file) {
        Ok(instructions) => run(instructions, context),
        Err(error) => Err(error),
    }
}

fn run(instructions: Vec<Instruction>, context: Context) -> Result<Context, ScriptError> {
    let runtime = create_runtime(instructions, context);

    run_instructions(runtime, 0)
}

fn create_runtime(instructions: Vec<Instruction>, context: Context) -> Runtime {
    let mut runtime = Runtime::new(context);

    let mut line = 0;
    for instruction in &instructions {
        match &instruction.instruction_type {
            InstructionType::Script(ref value) => {
                match value.label {
                    Some(ref label) => {
                        runtime.label_to_line.insert(label.to_string(), line);
                        ()
                    }
                    None => (),
                };
            }
            _ => (),
        };

        line = line + 1;
    }

    runtime.instructions = Some(instructions);

    runtime
}

fn run_instructions(mut runtime: Runtime, start_at: usize) -> Result<Context, ScriptError> {
    let mut line = start_at;
    let mut state = runtime.context.state.clone();

    let instructions = match runtime.instructions {
        Some(ref instructions) => instructions,
        None => return Ok(runtime.context),
    };

    loop {
        let (instruction, meta_info) = if instructions.len() > line {
            let instruction = instructions[line].clone();
            let meta_info = instruction.meta_info.clone();
            (instruction, meta_info)
        } else {
            break;
        };

        let (command_result, output_variable) = run_instruction(
            &mut runtime.context.commands,
            &mut runtime.context.variables,
            &mut state,
            &instructions,
            instruction,
            line,
        );

        match command_result {
            CommandResult::Exit(output) => {
                update_output(&mut runtime.context, output_variable, output);

                break;
            }
            CommandResult::Error(error) => {
                return Err(ScriptError {
                    info: ErrorInfo::Runtime(error, Some(meta_info)),
                });
            }
            CommandResult::Continue(output) => {
                update_output(&mut runtime.context, output_variable, output);

                line = line + 1;

                ()
            }
            CommandResult::GoTo(output, goto_value) => {
                update_output(&mut runtime.context, output_variable, output);

                match goto_value {
                    GoToValue::Label(label) => match runtime.label_to_line.get(&label) {
                        Some(value) => line = *value,
                        None => {
                            return Err(ScriptError {
                                info: ErrorInfo::Runtime(
                                    format!("Label: {} not found.", label),
                                    Some(meta_info),
                                ),
                            });
                        }
                    },
                    GoToValue::Line(line_number) => line = line_number,
                }
            }
        };
    }

    runtime.context.state = state;

    Ok(runtime.context)
}

fn update_output(context: &mut Context, output_variable: Option<String>, output: Option<String>) {
    if output_variable.is_some() {
        match output {
            Some(value) => context.variables.insert(output_variable.unwrap(), value),
            None => context.variables.remove(&output_variable.unwrap()),
        };
    }
}

/// Enables to evaluate a single instruction and return its result.
pub fn run_instruction(
    commands: &mut Commands,
    variables: &mut HashMap<String, String>,
    state: &mut HashMap<String, StateValue>,
    instructions: &Vec<Instruction>,
    instruction: Instruction,
    line: usize,
) -> (CommandResult, Option<String>) {
    let mut output_variable = None;
    let command_result = match instruction.instruction_type {
        InstructionType::Empty => CommandResult::Continue(None),
        InstructionType::PreProcess(_) => CommandResult::Continue(None),
        InstructionType::Script(ref script_instruction) => {
            output_variable = script_instruction.output.clone();

            match script_instruction.command {
                Some(ref command) => match commands.get_for_use(command) {
                    Some(command_instance) => {
                        let command_arguments =
                            bind_command_arguments(&variables, &script_instruction);

                        let command_result = if command_instance.requires_context() {
                            command_instance.run_with_context(
                                command_arguments,
                                state,
                                variables,
                                output_variable.clone(),
                                &instructions,
                                commands,
                                line,
                            )
                        } else {
                            command_instance.run(command_arguments)
                        };

                        commands.return_after_usage(command_instance);

                        command_result
                    }
                    None => CommandResult::Error(format!("Command: {} not found.", &command)),
                },
                None => CommandResult::Continue(None),
            }
        }
    };

    (command_result, output_variable)
}

fn bind_command_arguments(
    variables: &HashMap<String, String>,
    instruction: &ScriptInstruction,
) -> Vec<String> {
    let mut arguments = vec![];

    match instruction.arguments {
        Some(ref arguments_ref) => {
            for argument in arguments_ref {
                let value = expansion::expand_by_wrapper(&argument, "${", '}', variables);

                if !value.is_empty() {
                    arguments.push(value);
                }
            }
        }
        None => (),
    };

    arguments
}
