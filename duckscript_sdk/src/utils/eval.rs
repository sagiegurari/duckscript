use duckscript::parser;
use duckscript::runner;
use duckscript::types::command::{CommandResult, Commands, GoToValue};
use duckscript::types::env::Env;
use duckscript::types::instruction::{Instruction, InstructionType};
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;

#[cfg(test)]
#[path = "./eval_test.rs"]
mod eval_test;

fn parse(arguments: &Vec<String>) -> Result<Instruction, String> {
    let mut line_buffer = String::new();
    for argument in arguments {
        if argument.is_empty() {
            line_buffer.push_str("\"\"");
        } else if argument.starts_with("\"") && argument.ends_with("\"") {
            line_buffer.push('\\');
            line_buffer.push_str(argument);
            line_buffer.push('\\');
        } else {
            if argument.contains(" ") {
                line_buffer.push('"');
            }
            line_buffer.push_str(argument);
            if argument.contains(" ") {
                line_buffer.push('"');
            }
        }
        line_buffer.push(' ');
    }

    let line_str = line_buffer
        .replace("\r", "")
        .replace("\n", "")
        .replace("\\", "\\\\");

    match parser::parse_text(&line_str) {
        Ok(instructions) => Ok(instructions[0].clone()),
        Err(error) => Err(error.to_string()),
    }
}

pub(crate) fn eval(
    arguments: &Vec<String>,
    state: &mut HashMap<String, StateValue>,
    variables: &mut HashMap<String, String>,
    commands: &mut Commands,
    env: &mut Env,
) -> Result<CommandResult, String> {
    if arguments.is_empty() {
        Ok(CommandResult::Continue(None))
    } else {
        match parse(arguments) {
            Ok(instruction) => {
                let (command_result, _) = runner::run_instruction(
                    commands,
                    variables,
                    state,
                    &vec![],
                    instruction,
                    0,
                    env,
                );

                Ok(command_result)
            }
            Err(error) => Err(error.to_string()),
        }
    }
}

pub(crate) fn eval_with_error(
    arguments: &Vec<String>,
    state: &mut HashMap<String, StateValue>,
    variables: &mut HashMap<String, String>,
    commands: &mut Commands,
    env: &mut Env,
) -> CommandResult {
    match eval(arguments, state, variables, commands, env) {
        Ok(command_result) => match command_result.clone() {
            CommandResult::Crash(error) => CommandResult::Error(error),
            _ => command_result,
        },
        Err(error) => CommandResult::Error(error.to_string()),
    }
}

pub(crate) fn eval_with_instructions(
    arguments: &Vec<String>,
    instructions: &Vec<Instruction>,
    state: &mut HashMap<String, StateValue>,
    variables: &mut HashMap<String, String>,
    commands: &mut Commands,
    env: &mut Env,
) -> CommandResult {
    if arguments.is_empty() {
        CommandResult::Continue(None)
    } else {
        match parse(arguments) {
            Ok(instruction) => {
                let mut all_instructions = instructions.clone();
                all_instructions.push(instruction);
                let (flow_result, flow_output) = eval_instructions(
                    &all_instructions,
                    commands,
                    state,
                    variables,
                    env,
                    all_instructions.len() - 1,
                );

                match flow_result {
                    Some(result) => match result.clone() {
                        CommandResult::Crash(error) => CommandResult::Error(error),
                        _ => result,
                    },
                    None => CommandResult::Continue(flow_output),
                }
            }
            Err(error) => CommandResult::Error(error),
        }
    }
}

pub(crate) fn eval_instructions(
    instructions: &Vec<Instruction>,
    commands: &mut Commands,
    state: &mut HashMap<String, StateValue>,
    variables: &mut HashMap<String, String>,
    env: &mut Env,
    start_line: usize,
) -> (Option<CommandResult>, Option<String>) {
    let mut line = start_line;
    let mut flow_output = None;
    let mut flow_result = None;
    loop {
        let instruction = if instructions.len() > line {
            instructions[line].clone()
        } else {
            break;
        };

        match instruction.instruction_type {
            InstructionType::Script(ref script_instruction) => {
                let (command_result, _) = runner::run_instruction(
                    commands,
                    variables,
                    state,
                    &instructions,
                    instruction.clone(),
                    line,
                    env,
                );

                match command_result {
                    CommandResult::Exit(output) => {
                        flow_result = Some(CommandResult::Exit(output));
                        break;
                    }
                    CommandResult::Error(error) => {
                        flow_result = Some(CommandResult::Error(error));
                        break;
                    }
                    CommandResult::Crash(error) => {
                        flow_result = Some(CommandResult::Crash(error));
                        break;
                    }
                    CommandResult::GoTo(output, goto_value) => {
                        flow_output = output.clone();

                        match goto_value {
                            GoToValue::Label(_) => {
                                flow_result = Some(CommandResult::Error(
                                    "goto label result not supported in alias command flow."
                                        .to_string(),
                                ));
                                break;
                            }
                            GoToValue::Line(line_number) => line = line_number,
                        }
                    }
                    CommandResult::Continue(output) => {
                        flow_output = output.clone();

                        match script_instruction.output {
                            Some(ref output_variable) => {
                                match output {
                                    Some(value) => {
                                        variables.insert(output_variable.to_string(), value)
                                    }
                                    None => variables.remove(output_variable),
                                };
                            }
                            None => (),
                        };

                        line = line + 1;
                    }
                };
            }
            _ => {
                line = line + 1;
            }
        };
    }

    (flow_result, flow_output)
}
