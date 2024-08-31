//! # runner
//!
//! The main entry point which enables running scripts.
//!

#[cfg(test)]
#[path = "./runner_test.rs"]
mod runner_test;

use crate::expansion::{self, ExpandedValue};
use crate::parser;
use crate::types::command::{CommandResult, Commands, GoToValue};
use crate::types::env::Env;
use crate::types::error::ScriptError;
use crate::types::instruction::{
    Instruction, InstructionMetaInfo, InstructionType, ScriptInstruction,
};
use crate::types::runtime::{Context, Runtime, StateValue};
use std::collections::HashMap;
use std::io::stdin;

#[derive(Debug)]
enum EndReason {
    ExitCalled,
    ReachedEnd,
    Crash(ScriptError),
}

/// Executes the provided script with the given context
pub fn run_script(text: &str, context: Context, env: Option<Env>) -> Result<Context, ScriptError> {
    match parser::parse_text(text) {
        Ok(instructions) => run(instructions, context, env),
        Err(error) => Err(error),
    }
}

/// Executes the provided script file with the given context
pub fn run_script_file(
    file: &str,
    context: Context,
    env: Option<Env>,
) -> Result<Context, ScriptError> {
    match parser::parse_file(file) {
        Ok(instructions) => run(instructions, context, env),
        Err(error) => Err(error),
    }
}

/// Provides the REPL entry point
pub fn repl(mut context: Context) -> Result<Context, ScriptError> {
    let mut text = String::new();
    let mut instructions = vec![];

    loop {
        text.clear();

        match stdin().read_line(&mut text) {
            Ok(_) => {
                match parser::parse_text(&text) {
                    Ok(mut new_instructions) => {
                        // get start line
                        let start = instructions.len();

                        // add new instructions
                        instructions.append(&mut new_instructions);
                        let runtime = create_runtime(instructions.clone(), context, None);

                        let (updated_context, end_reason) = run_instructions(runtime, start, true)?;

                        context = updated_context;

                        match end_reason {
                            EndReason::ExitCalled => return Ok(context),
                            EndReason::Crash(error) => println!("{}", &error.to_string()),
                            _ => (),
                        };
                    }
                    Err(error) => return Err(error),
                }
            }
            Err(error) => {
                return Err(ScriptError::Runtime(
                    error.to_string(),
                    Some(InstructionMetaInfo::new()),
                ));
            }
        };
    }
}

fn run(
    instructions: Vec<Instruction>,
    context: Context,
    env: Option<Env>,
) -> Result<Context, ScriptError> {
    let runtime = create_runtime(instructions, context, env);

    match run_instructions(runtime, 0, false) {
        Ok((context, _)) => Ok(context),
        Err(error) => Err(error),
    }
}

fn create_runtime(instructions: Vec<Instruction>, context: Context, env: Option<Env>) -> Runtime {
    let mut runtime = Runtime::new(context, env);

    let mut line = 0;
    for instruction in &instructions {
        if let InstructionType::Script(ref value) = &instruction.instruction_type {
            if let Some(ref label) = value.label {
                runtime.label_to_line.insert(label.to_string(), line);
            };
        };

        line += 1;
    }

    runtime.instructions = Some(instructions);

    runtime
}

fn run_instructions(
    mut runtime: Runtime,
    start_at: usize,
    repl_mode: bool,
) -> Result<(Context, EndReason), ScriptError> {
    let mut line = start_at;
    let mut state = runtime.context.state.clone();

    let instructions = match runtime.instructions {
        Some(ref instructions) => instructions,
        None => return Ok((runtime.context, EndReason::ReachedEnd)),
    };

    let mut end_reason = EndReason::ReachedEnd;
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
            instructions,
            instruction,
            line,
            &mut runtime.env,
        );

        match command_result {
            CommandResult::Exit(output) => {
                update_output(
                    &mut runtime.context.variables,
                    output_variable,
                    output.clone(),
                );
                end_reason = EndReason::ExitCalled;

                if repl_mode {
                    return Ok((runtime.context, end_reason));
                }

                if let Some(exit_code_str) = output {
                    if let Ok(exit_code) = exit_code_str.parse::<i32>() {
                        if exit_code != 0 {
                            return Err(ScriptError::Runtime(
                                format!("Exit with error code: {}", exit_code).to_string(),
                                Some(meta_info.clone()),
                            ));
                        }
                    }
                }

                break;
            }
            CommandResult::Error(error) => {
                update_output(
                    &mut runtime.context.variables,
                    output_variable,
                    Some("false".to_string()),
                );

                let post_error_line = line + 1;

                if let Err(error) = run_on_error_instruction(
                    &mut runtime.context.commands,
                    &mut runtime.context.variables,
                    &mut state,
                    instructions,
                    error,
                    meta_info.clone(),
                    &mut runtime.env,
                ) {
                    return Err(ScriptError::Runtime(error, Some(meta_info.clone())));
                };

                line = post_error_line;
            }
            CommandResult::Crash(error) => {
                let script_error = ScriptError::Runtime(error, Some(meta_info));

                if repl_mode {
                    return Ok((runtime.context, EndReason::Crash(script_error)));
                }

                return Err(script_error);
            }
            CommandResult::Continue(output) => {
                update_output(&mut runtime.context.variables, output_variable, output);

                line += 1;
            }
            CommandResult::GoTo(output, goto_value) => {
                update_output(&mut runtime.context.variables, output_variable, output);

                match goto_value {
                    GoToValue::Label(label) => match runtime.label_to_line.get(&label) {
                        Some(value) => line = *value,
                        None => {
                            return Err(ScriptError::Runtime(
                                format!("Label: {} not found.", label),
                                Some(meta_info),
                            ));
                        }
                    },
                    GoToValue::Line(line_number) => line = line_number,
                }
            }
        };
    }

    runtime.context.state = state;

    Ok((runtime.context, end_reason))
}

fn update_output(
    variables: &mut HashMap<String, String>,
    output_variable: Option<String>,
    output: Option<String>,
) {
    if output_variable.is_some() {
        match output {
            Some(value) => variables.insert(output_variable.unwrap(), value),
            None => variables.remove(&output_variable.unwrap()),
        };
    }
}

fn run_on_error_instruction(
    commands: &mut Commands,
    variables: &mut HashMap<String, String>,
    state: &mut HashMap<String, StateValue>,
    instructions: &Vec<Instruction>,
    error: String,
    meta_info: InstructionMetaInfo,
    env: &mut Env,
) -> Result<(), String> {
    if commands.exists("on_error") {
        let mut script_instruction = ScriptInstruction::new();
        script_instruction.command = Some("on_error".to_string());
        script_instruction.arguments = Some(vec![
            error,
            meta_info.line.unwrap_or(0).to_string(),
            meta_info.source.unwrap_or("".to_string()),
        ]);
        let instruction = Instruction {
            meta_info: InstructionMetaInfo::new(),
            instruction_type: InstructionType::Script(script_instruction),
        };

        let (command_result, output_variable) = run_instruction(
            commands,
            variables,
            state,
            instructions,
            instruction,
            0,
            env,
        );

        match command_result {
            CommandResult::Exit(output) => {
                update_output(variables, output_variable, output);

                Err("Exiting Script.".to_string())
            }
            CommandResult::Crash(error) => Err(error),
            _ => Ok(()),
        }
    } else {
        Ok(())
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
    env: &mut Env,
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
                        let command_arguments = bind_command_arguments(
                            variables,
                            script_instruction,
                            &instruction.meta_info,
                        );

                        if command_instance.requires_context() {
                            command_instance.run_with_context(
                                command_arguments,
                                state,
                                variables,
                                output_variable.clone(),
                                instructions,
                                commands,
                                line,
                                env,
                            )
                        } else {
                            command_instance.run(command_arguments)
                        }
                    }
                    None => CommandResult::Crash(format!("Command: {} not found.", &command)),
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
    meta_info: &InstructionMetaInfo,
) -> Vec<String> {
    let mut arguments = vec![];

    match instruction.arguments {
        Some(ref arguments_ref) => {
            for argument in arguments_ref {
                match expansion::expand_by_wrapper(&argument, meta_info, variables) {
                    ExpandedValue::Single(value) => arguments.push(value),
                    ExpandedValue::Multi(values) => {
                        for value in values {
                            arguments.push(value)
                        }
                    }
                    ExpandedValue::None => arguments.push("".to_string()),
                }
            }
        }
        None => (),
    };

    arguments
}
