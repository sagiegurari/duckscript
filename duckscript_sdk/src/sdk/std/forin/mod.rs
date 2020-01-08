use crate::utils::instruction_query;
use crate::utils::pckg;
use crate::utils::state::{get_core_sub_state_for_command, get_handle, get_list, get_sub_state};
use duckscript::types::command::{Command, CommandResult, Commands, GoToValue};
use duckscript::types::error::ScriptError;
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

static FORIN_STATE_KEY: &str = "forin";
static META_INFO_STATE_KEY: &str = "meta_info";
static CALL_STACK_STATE_KEY: &str = "call_stack";

#[derive(Debug, Clone)]
struct ForInMetaInfo {
    pub(crate) start: usize,
    pub(crate) end: usize,
}

#[derive(Debug, Clone)]
struct CallInfo {
    pub(crate) iteration: usize,
    pub(crate) meta_info: ForInMetaInfo,
}

fn serialize_forin_meta_info(
    meta_info: &ForInMetaInfo,
    sub_state: &mut HashMap<String, StateValue>,
) {
    sub_state.insert(
        "start".to_string(),
        StateValue::UnsignedNumber(meta_info.start),
    );
    sub_state.insert("end".to_string(), StateValue::UnsignedNumber(meta_info.end));
}

fn deserialize_forin_meta_info(
    sub_state: &mut HashMap<String, StateValue>,
) -> Option<ForInMetaInfo> {
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

    Some(ForInMetaInfo { start, end })
}

fn serialize_call_info(call_info: &CallInfo, sub_state: &mut HashMap<String, StateValue>) {
    sub_state.insert(
        "iteration".to_string(),
        StateValue::UnsignedNumber(call_info.iteration),
    );

    let mut meta_info_state = HashMap::new();
    serialize_forin_meta_info(&call_info.meta_info, &mut meta_info_state);
    sub_state.insert(
        "meta_info".to_string(),
        StateValue::SubState(meta_info_state),
    );
}

fn deserialize_call_info(sub_state: &mut HashMap<String, StateValue>) -> Option<CallInfo> {
    let iteration = match sub_state.get("iteration") {
        Some(state_value) => match state_value {
            StateValue::UnsignedNumber(value) => *value,
            _ => return None,
        },
        None => return None,
    };
    let meta_info = match sub_state.get("meta_info") {
        Some(state_value) => match state_value.clone() {
            StateValue::SubState(mut value) => match deserialize_forin_meta_info(&mut value) {
                Some(meta_info) => meta_info,
                None => return None,
            },
            _ => return None,
        },
        None => return None,
    };

    Some(CallInfo {
        iteration,
        meta_info,
    })
}

fn create_forin_meta_info_for_line(
    line: usize,
    instructions: &Vec<Instruction>,
    package: String,
) -> Result<ForInMetaInfo, String> {
    // start names
    let forin_command = ForInCommand {
        package: package.clone(),
    };
    let mut start_names = forin_command.aliases();
    start_names.push(forin_command.name());

    // end names
    let end_forin_command = EndForInCommand {
        package: package.clone(),
    };
    let mut end_names = end_forin_command.aliases();
    end_names.push(end_forin_command.name());

    let positions_options = instruction_query::find_commands(
        instructions,
        &start_names,
        &vec![],
        &end_names,
        Some(line + 1),
        None,
        true,
    )?;

    match positions_options {
        Some(positions) => Ok(ForInMetaInfo {
            start: line,
            end: positions.end,
        }),
        None => Err("End of for in block not found.".to_string()),
    }
}

fn get_or_create_forin_meta_info_for_line(
    line: usize,
    state: &mut HashMap<String, StateValue>,
    instructions: &Vec<Instruction>,
    package: String,
) -> Result<ForInMetaInfo, String> {
    let forin_state = get_core_sub_state_for_command(state, FORIN_STATE_KEY.to_string());
    let forin_meta_info_state = get_sub_state(META_INFO_STATE_KEY.to_string(), forin_state);

    let key = line.to_string();
    let mut forin_state_for_line = get_sub_state(key.clone(), forin_meta_info_state);

    match deserialize_forin_meta_info(&mut forin_state_for_line) {
        Some(if_else_info) => Ok(if_else_info),
        None => match create_forin_meta_info_for_line(line, instructions, package) {
            Ok(if_else_info) => {
                serialize_forin_meta_info(&if_else_info, forin_state_for_line);
                Ok(if_else_info)
            }
            Err(error) => Err(error),
        },
    }
}

fn pop_call_info_for_line(
    line: usize,
    state: &mut HashMap<String, StateValue>,
    recursive: bool,
) -> Option<CallInfo> {
    let forin_state = get_core_sub_state_for_command(state, FORIN_STATE_KEY.to_string());
    let call_info_stack = get_list(CALL_STACK_STATE_KEY.to_string(), forin_state);

    match call_info_stack.pop() {
        Some(state_value) => match state_value {
            StateValue::SubState(mut call_info_state) => {
                match deserialize_call_info(&mut call_info_state) {
                    Some(call_info) => {
                        if call_info.meta_info.start == line || call_info.meta_info.end == line {
                            Some(call_info)
                        } else if recursive {
                            pop_call_info_for_line(line, state, recursive)
                        } else {
                            store_call_info(&call_info, state);
                            None
                        }
                    }
                    None => None,
                }
            }
            _ => pop_call_info_for_line(line, state, recursive),
        },
        None => None,
    }
}

fn store_call_info(call_info: &CallInfo, state: &mut HashMap<String, StateValue>) {
    let forin_state = get_core_sub_state_for_command(state, FORIN_STATE_KEY.to_string());
    let call_info_stack = get_list(CALL_STACK_STATE_KEY.to_string(), forin_state);

    let mut call_info_state = HashMap::new();
    serialize_call_info(call_info, &mut call_info_state);
    call_info_stack.push(StateValue::SubState(call_info_state));
}

fn get_next_iteration(
    iteration: usize,
    handle: String,
    state: &mut HashMap<String, StateValue>,
) -> Option<String> {
    match get_handle(state, handle) {
        Some(state_value) => match state_value {
            StateValue::List(list) => {
                if list.len() > iteration {
                    match list[iteration] {
                        StateValue::Boolean(ref value) => Some(value.to_string()),
                        StateValue::Number(ref value) => Some(value.to_string()),
                        StateValue::UnsignedNumber(ref value) => Some(value.to_string()),
                        StateValue::Number32Bit(ref value) => Some(value.to_string()),
                        StateValue::UnsignedNumber32Bit(ref value) => Some(value.to_string()),
                        StateValue::Number64Bit(ref value) => Some(value.to_string()),
                        StateValue::UnsignedNumber64Bit(ref value) => Some(value.to_string()),
                        StateValue::String(ref value) => Some(value.to_string()),
                        StateValue::List(_) => None,
                        StateValue::SubState(_) => None,
                    }
                } else {
                    None
                }
            }
            _ => None,
        },
        None => None,
    }
}

struct ForInCommand {
    package: String,
}

impl Command for ForInCommand {
    fn name(&self) -> String {
        pckg::concat(&self.package, "ForIn")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["for".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
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
        _commands: &mut Commands,
        line: usize,
    ) -> CommandResult {
        if arguments.len() != 3 || arguments[1] != "in" {
            CommandResult::Error("Invalid for/in statement".to_string())
        } else {
            let call_info = match pop_call_info_for_line(line, state, false) {
                Some(call_info) => call_info,
                None => {
                    let forin_meta_info_result = get_or_create_forin_meta_info_for_line(
                        line,
                        state,
                        instructions,
                        self.package.clone(),
                    );

                    match forin_meta_info_result {
                        Ok(forin_meta_info) => CallInfo {
                            iteration: 0,
                            meta_info: forin_meta_info,
                        },
                        Err(error) => return CommandResult::Crash(error.to_string()),
                    }
                }
            };

            let iteration = call_info.iteration;
            let forin_meta_info = call_info.meta_info;

            match variables.get(&arguments[2]) {
                Some(handle) => match get_next_iteration(iteration, handle.to_string(), state) {
                    Some(next_value) => {
                        store_call_info(
                            &CallInfo {
                                iteration: iteration + 1,
                                meta_info: forin_meta_info,
                            },
                            state,
                        );

                        variables.insert(arguments[0].clone(), next_value);
                        CommandResult::Continue(None)
                    }
                    None => {
                        let next_line = forin_meta_info.end + 1;
                        CommandResult::GoTo(None, GoToValue::Line(next_line))
                    }
                },
                None => {
                    let next_line = forin_meta_info.end + 1;
                    CommandResult::GoTo(None, GoToValue::Line(next_line))
                }
            }
        }
    }
}

struct EndForInCommand {
    package: String,
}

impl Command for EndForInCommand {
    fn name(&self) -> String {
        pckg::concat(&self.package, "EndForIn")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["end_for".to_string()]
    }

    fn help(&self) -> String {
        "".to_string()
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
    ) -> CommandResult {
        match pop_call_info_for_line(line, state, true) {
            Some(call_info) => {
                let next_line = call_info.meta_info.start;
                store_call_info(&call_info, state);
                CommandResult::GoTo(None, GoToValue::Line(next_line))
            }
            None => CommandResult::Error(
                "Found an end for/in command but not currently running part of a for/in invocation flow."
                    .to_string(),
            ),
        }
    }
}

pub(crate) fn create(package: &str) -> Vec<Box<dyn Command>> {
    vec![
        Box::new(ForInCommand {
            package: package.to_string(),
        }),
        Box::new(EndForInCommand {
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
