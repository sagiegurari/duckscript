use crate::sdk::std::flowcontrol::{end, forin, function, get_line_key, ifelse};
use crate::types::scope::get_line_context_name;
use crate::utils::state::{get_core_sub_state_for_command, get_list, get_sub_state};
use crate::utils::{condition, instruction_query, pckg};
use duckscript::types::command::{Command, CommandResult, Commands, GoToValue};
use duckscript::types::env::Env;
use duckscript::types::error::ScriptError;
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

static WHILE_STATE_KEY: &str = "while";
static META_INFO_STATE_KEY: &str = "meta_info";
static CALL_STACK_STATE_KEY: &str = "call_stack";

#[derive(Debug, Clone)]
struct WhileMetaInfo {
    pub(crate) start: usize,
    pub(crate) end: usize,
}

#[derive(Debug)]
struct CallInfo {
    pub(crate) meta_info: WhileMetaInfo,
    pub(crate) line_context_name: String,
}

fn serialize_while_meta_info(
    meta_info: &WhileMetaInfo,
    sub_state: &mut HashMap<String, StateValue>,
) {
    sub_state.insert(
        "start".to_string(),
        StateValue::UnsignedNumber(meta_info.start),
    );
    sub_state.insert("end".to_string(), StateValue::UnsignedNumber(meta_info.end));
}

fn deserialize_while_meta_info(
    sub_state: &mut HashMap<String, StateValue>,
) -> Option<WhileMetaInfo> {
    let start = match sub_state.get("start") {
        Some(state_value) => match state_value {
            StateValue::UnsignedNumber(value) => *value,
            _ => return None,
        },
        None => return None,
    };
    let end = match sub_state.get("end") {
        Some(state_value) => match state_value {
            StateValue::UnsignedNumber(value) => *value,
            _ => return None,
        },
        None => return None,
    };

    Some(WhileMetaInfo { start, end })
}

fn serialize_call_info(call_info: &CallInfo, sub_state: &mut HashMap<String, StateValue>) {
    let mut meta_info_state = HashMap::new();
    serialize_while_meta_info(&call_info.meta_info, &mut meta_info_state);
    sub_state.insert(
        "meta_info".to_string(),
        StateValue::SubState(meta_info_state),
    );
    sub_state.insert(
        "line_context_name".to_string(),
        StateValue::String(call_info.line_context_name.clone()),
    );
}

fn deserialize_call_info(sub_state: &mut HashMap<String, StateValue>) -> Option<CallInfo> {
    let meta_info = match sub_state.get("meta_info") {
        Some(state_value) => match state_value.clone() {
            StateValue::SubState(mut value) => match deserialize_while_meta_info(&mut value) {
                Some(meta_info) => meta_info,
                None => return None,
            },
            _ => return None,
        },
        None => return None,
    };
    let line_context_name = match sub_state.get("line_context_name") {
        Some(value) => match value {
            StateValue::String(line_context_name) => line_context_name.clone(),
            _ => return None,
        },
        None => return None,
    };

    Some(CallInfo {
        meta_info,
        line_context_name,
    })
}

fn create_while_meta_info_for_line(
    line: usize,
    instructions: &Vec<Instruction>,
    package: String,
) -> Result<WhileMetaInfo, String> {
    // start names
    let if_command = WhileCommand {
        package: package.clone(),
    };
    let mut start_names = if_command.aliases();
    start_names.push(if_command.name());

    // end names
    let end_while_command = EndWhileCommand {
        package: package.clone(),
    };
    let mut end_names = end_while_command.aliases();
    end_names.push(end_while_command.name());
    end_names.push(end::END_COMMAND_NAME.to_string());

    let function_command = function::FunctionCommand::new(&package);
    let mut start_blocks = function_command.aliases();
    start_blocks.push(function_command.name());
    let forin_command = forin::ForInCommand::new(&package);
    start_blocks.append(&mut forin_command.aliases());
    start_blocks.push(forin_command.name());
    let if_command = ifelse::IfCommand::new(&package);
    start_blocks.append(&mut if_command.aliases());
    start_blocks.push(if_command.name());

    let end_forin_command = forin::EndForInCommand::new(&package);
    let mut end_blocks = end_forin_command.aliases();
    end_blocks.push(end_forin_command.name());
    let end_function_command = function::EndFunctionCommand::new(&package);
    end_blocks.append(&mut end_function_command.aliases());
    end_blocks.push(end_function_command.name());
    let end_if_command = ifelse::EndIfCommand::new(&package);
    end_blocks.append(&mut end_if_command.aliases());
    end_blocks.push(end_if_command.name());
    end_blocks.push(end::END_COMMAND_NAME.to_string());

    let positions_options = instruction_query::find_commands(
        instructions,
        &start_names,
        &vec![],
        &end_names,
        Some(line + 1),
        None,
        true,
        &start_blocks,
        &end_blocks,
    )?;

    match positions_options {
        Some(positions) => Ok(WhileMetaInfo {
            start: line,
            end: positions.end,
        }),
        None => Err("End of while block not found.".to_string()),
    }
}

fn get_or_create_while_meta_info_for_line(
    line: usize,
    state: &mut HashMap<String, StateValue>,
    instructions: &Vec<Instruction>,
    package: String,
) -> Result<WhileMetaInfo, String> {
    let key = get_line_key(line, state);
    let while_state = get_core_sub_state_for_command(state, WHILE_STATE_KEY.to_string());
    let while_meta_info_state = get_sub_state(META_INFO_STATE_KEY.to_string(), while_state);

    let mut while_state_for_line = get_sub_state(key.clone(), while_meta_info_state);

    let result = match deserialize_while_meta_info(&mut while_state_for_line) {
        Some(while_info) => Ok(while_info),
        None => match create_while_meta_info_for_line(line, instructions, package.clone()) {
            Ok(while_info) => {
                serialize_while_meta_info(&while_info, while_state_for_line);
                Ok(while_info)
            }
            Err(error) => Err(error),
        },
    };

    match result {
        Ok(ref info) => {
            let end_while_command = EndWhileCommand {
                package: package.clone(),
            };
            end::set_command(info.end, state, end_while_command.name());
        }
        _ => (),
    };

    result
}

fn pop_call_info_for_line(
    line: usize,
    state: &mut HashMap<String, StateValue>,
) -> Option<CallInfo> {
    let line_context_name = get_line_context_name(state);
    let while_state = get_core_sub_state_for_command(state, WHILE_STATE_KEY.to_string());
    let call_info_stack = get_list(CALL_STACK_STATE_KEY.to_string(), while_state);

    match call_info_stack.pop() {
        Some(state_value) => match state_value {
            StateValue::SubState(mut call_info_state) => {
                match deserialize_call_info(&mut call_info_state) {
                    Some(call_info) => {
                        if call_info.meta_info.end == line
                            && call_info.line_context_name == line_context_name
                        {
                            Some(call_info)
                        } else {
                            pop_call_info_for_line(line, state)
                        }
                    }
                    None => None,
                }
            }
            _ => pop_call_info_for_line(line, state),
        },
        None => None,
    }
}

fn store_call_info(call_info: &CallInfo, state: &mut HashMap<String, StateValue>) {
    let while_state = get_core_sub_state_for_command(state, WHILE_STATE_KEY.to_string());
    let call_info_stack = get_list(CALL_STACK_STATE_KEY.to_string(), while_state);

    let mut call_info_state = HashMap::new();
    serialize_call_info(call_info, &mut call_info_state);
    call_info_stack.push(StateValue::SubState(call_info_state));
}

#[derive(Clone)]
pub(crate) struct WhileCommand {
    package: String,
}

impl WhileCommand {
    /// Creates and returns a new instance.
    pub(crate) fn new(package: &str) -> WhileCommand {
        WhileCommand {
            package: package.to_string(),
        }
    }
}

impl Command for WhileCommand {
    fn name(&self) -> String {
        pckg::concat(&self.package, "While")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["while".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn requires_context(&self) -> bool {
        true
    }

    fn run_with_context(
        &self,
        arguments: Vec<String>,
        state: &mut HashMap<String, StateValue>,
        variables: &mut HashMap<String, String>,
        _output_variable: Option<String>,
        instructions: &Vec<Instruction>,
        commands: &mut Commands,
        line: usize,
        env: &mut Env,
    ) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Error("Missing condition".to_string())
        } else {
            match get_or_create_while_meta_info_for_line(
                line,
                state,
                instructions,
                self.package.clone(),
            ) {
                Ok(while_info) => {
                    match condition::eval_condition(
                        arguments,
                        instructions,
                        state,
                        variables,
                        commands,
                        env,
                    ) {
                        Ok(passed) => {
                            if passed {
                                let line_context_name = get_line_context_name(state);

                                let call_info = CallInfo {
                                    meta_info: while_info.clone(),
                                    line_context_name,
                                };

                                store_call_info(&call_info, state);

                                CommandResult::Continue(None)
                            } else {
                                let next_line = while_info.end + 1;
                                CommandResult::GoTo(None, GoToValue::Line(next_line))
                            }
                        }
                        Err(error) => CommandResult::Error(error.to_string()),
                    }
                }
                Err(error) => CommandResult::Crash(error.to_string()),
            }
        }
    }
}

#[derive(Clone)]
pub(crate) struct EndWhileCommand {
    package: String,
}

impl EndWhileCommand {
    /// Creates and returns a new instance.
    pub(crate) fn new(package: &str) -> EndWhileCommand {
        EndWhileCommand {
            package: package.to_string(),
        }
    }
}

impl Command for EndWhileCommand {
    fn name(&self) -> String {
        pckg::concat(&self.package, "EndWhile")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["end_while".to_string(), "endwhile".to_string()]
    }

    fn help(&self) -> String {
        "".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn requires_context(&self) -> bool {
        true
    }

    fn run_with_context(
        &self,
        _arguments: Vec<String>,
        state: &mut HashMap<String, StateValue>,
        _variables: &mut HashMap<String, String>,
        _output_variable: Option<String>,
        _instructions: &Vec<Instruction>,
        _commands: &mut Commands,
        line: usize,
        _env: &mut Env,
    ) -> CommandResult {
        match pop_call_info_for_line(line, state) {
            Some(call_info) => {
                let next_line = call_info.meta_info.start;
                store_call_info(&call_info, state);
                CommandResult::GoTo(None, GoToValue::Line(next_line))
            }
            None => CommandResult::Error(
                "Found an end while command but not currently running part of a while invocation flow."
                    .to_string(),
            ),
        }
    }
}

pub(crate) fn create(package: &str) -> Vec<Box<dyn Command>> {
    vec![
        Box::new(WhileCommand {
            package: package.to_string(),
        }),
        Box::new(EndWhileCommand {
            package: package.to_string(),
        }),
    ]
}

pub(crate) fn load(commands: &mut Commands, package: &str) -> Result<(), ScriptError> {
    let multi_commands = create(package);

    for command in multi_commands {
        commands.set(command)?;
    }

    Ok(())
}
