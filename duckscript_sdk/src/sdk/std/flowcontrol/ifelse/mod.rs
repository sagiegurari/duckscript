use crate::sdk::std::flowcontrol::{end, forin, function, get_line_key, while_mod};
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

static IFELSE_STATE_KEY: &str = "ifelse";
static META_INFO_STATE_KEY: &str = "meta_info";
static CALL_STACK_STATE_KEY: &str = "call_stack";

#[derive(Debug, Clone)]
struct IfElseMetaInfo {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) else_lines: Vec<usize>,
}

#[derive(Debug)]
struct CallInfo {
    pub(crate) current: usize,
    pub(crate) passed: bool,
    pub(crate) else_line_index: usize,
    pub(crate) meta_info: IfElseMetaInfo,
    pub(crate) line_context_name: String,
}

fn serialize_ifelse_meta_info(
    meta_info: &IfElseMetaInfo,
    sub_state: &mut HashMap<String, StateValue>,
) {
    sub_state.insert(
        "start".to_string(),
        StateValue::UnsignedNumber(meta_info.start),
    );
    sub_state.insert("end".to_string(), StateValue::UnsignedNumber(meta_info.end));

    let mut list = vec![];
    for line in &meta_info.else_lines {
        list.push(StateValue::UnsignedNumber(*line));
    }
    sub_state.insert("else_lines".to_string(), StateValue::List(list));
}

fn deserialize_ifelse_meta_info(
    sub_state: &mut HashMap<String, StateValue>,
) -> Option<IfElseMetaInfo> {
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

    let mut else_lines = vec![];
    match sub_state.get("else_lines") {
        Some(state_values_list) => match state_values_list {
            StateValue::List(list) => {
                for state_value_item in list {
                    match state_value_item {
                        StateValue::UnsignedNumber(value) => else_lines.push(*value),
                        _ => return None,
                    }
                }
            }
            _ => return None,
        },
        None => return None,
    };

    Some(IfElseMetaInfo {
        start,
        end,
        else_lines,
    })
}

fn serialize_call_info(call_info: &CallInfo, sub_state: &mut HashMap<String, StateValue>) {
    sub_state.insert(
        "current".to_string(),
        StateValue::UnsignedNumber(call_info.current),
    );
    sub_state.insert("passed".to_string(), StateValue::Boolean(call_info.passed));
    sub_state.insert(
        "else_line_index".to_string(),
        StateValue::UnsignedNumber(call_info.else_line_index),
    );

    let mut meta_info_state = HashMap::new();
    serialize_ifelse_meta_info(&call_info.meta_info, &mut meta_info_state);
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
    let current = match sub_state.get("current") {
        Some(state_value) => match state_value {
            StateValue::UnsignedNumber(value) => *value,
            _ => return None,
        },
        None => return None,
    };
    let passed = match sub_state.get("passed") {
        Some(state_value) => match state_value {
            StateValue::Boolean(value) => *value,
            _ => return None,
        },
        None => return None,
    };
    let else_line_index = match sub_state.get("else_line_index") {
        Some(state_value) => match state_value {
            StateValue::UnsignedNumber(value) => *value,
            _ => return None,
        },
        None => return None,
    };
    let meta_info = match sub_state.get("meta_info") {
        Some(state_value) => match state_value.clone() {
            StateValue::SubState(mut value) => match deserialize_ifelse_meta_info(&mut value) {
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
        current,
        passed,
        else_line_index,
        meta_info,
        line_context_name,
    })
}

fn create_if_meta_info_for_line(
    line: usize,
    instructions: &Vec<Instruction>,
    package: String,
) -> Result<IfElseMetaInfo, String> {
    // start names
    let if_command = IfCommand {
        package: package.clone(),
    };
    let mut start_names = if_command.aliases();
    start_names.push(if_command.name());

    // middle names
    let else_if_command = ElseIfCommand {
        package: package.clone(),
    };
    let mut middle_names = else_if_command.aliases();
    start_names.push(else_if_command.name());
    let else_command = ElseCommand {
        package: package.clone(),
    };
    middle_names.append(&mut else_command.aliases());
    start_names.push(else_command.name());

    // end names
    let end_if_command = EndIfCommand {
        package: package.clone(),
    };
    let mut end_names = end_if_command.aliases();
    end_names.push(end_if_command.name());
    end_names.push(end::END_COMMAND_NAME.to_string());

    let function_command = function::FunctionCommand::new(&package);
    let mut start_blocks = function_command.aliases();
    start_blocks.push(function_command.name());
    let forin_command = forin::ForInCommand::new(&package);
    start_blocks.append(&mut forin_command.aliases());
    start_blocks.push(forin_command.name());
    let while_command = while_mod::WhileCommand::new(&package);
    start_blocks.append(&mut while_command.aliases());
    start_blocks.push(while_command.name());

    let end_forin_command = forin::EndForInCommand::new(&package);
    let mut end_blocks = end_forin_command.aliases();
    end_blocks.push(end_forin_command.name());
    let end_function_command = function::EndFunctionCommand::new(&package);
    end_blocks.append(&mut end_function_command.aliases());
    end_blocks.push(end_function_command.name());
    let end_while_command = while_mod::EndWhileCommand::new(&package);
    end_blocks.append(&mut end_while_command.aliases());
    end_blocks.push(end_while_command.name());
    end_blocks.push(end::END_COMMAND_NAME.to_string());

    let positions_options = instruction_query::find_commands(
        instructions,
        &start_names,
        &middle_names,
        &end_names,
        Some(line + 1),
        None,
        true,
        &start_blocks,
        &end_blocks,
    )?;

    match positions_options {
        Some(positions) => Ok(IfElseMetaInfo {
            start: line,
            end: positions.end,
            else_lines: positions.middle,
        }),
        None => Err("End of if/else block not found.".to_string()),
    }
}

fn get_or_create_if_meta_info_for_line(
    line: usize,
    state: &mut HashMap<String, StateValue>,
    instructions: &Vec<Instruction>,
    package: String,
) -> Result<IfElseMetaInfo, String> {
    let key = get_line_key(line, state);
    let if_state = get_core_sub_state_for_command(state, IFELSE_STATE_KEY.to_string());
    let if_meta_info_state = get_sub_state(META_INFO_STATE_KEY.to_string(), if_state);

    let mut if_state_for_line = get_sub_state(key.clone(), if_meta_info_state);

    let result = match deserialize_ifelse_meta_info(&mut if_state_for_line) {
        Some(if_else_info) => Ok(if_else_info),
        None => match create_if_meta_info_for_line(line, instructions, package.clone()) {
            Ok(if_else_info) => {
                serialize_ifelse_meta_info(&if_else_info, if_state_for_line);
                Ok(if_else_info)
            }
            Err(error) => Err(error),
        },
    };

    match result {
        Ok(ref info) => {
            let end_if_command = EndIfCommand {
                package: package.clone(),
            };
            end::set_command(info.end, state, end_if_command.name());
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
    let if_state = get_core_sub_state_for_command(state, IFELSE_STATE_KEY.to_string());
    let call_info_stack = get_list(CALL_STACK_STATE_KEY.to_string(), if_state);

    match call_info_stack.pop() {
        Some(state_value) => match state_value {
            StateValue::SubState(mut call_info_state) => {
                match deserialize_call_info(&mut call_info_state) {
                    Some(call_info) => {
                        if call_info.current == line
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
    let if_state = get_core_sub_state_for_command(state, IFELSE_STATE_KEY.to_string());
    let call_info_stack = get_list(CALL_STACK_STATE_KEY.to_string(), if_state);

    let mut call_info_state = HashMap::new();
    serialize_call_info(call_info, &mut call_info_state);
    call_info_stack.push(StateValue::SubState(call_info_state));
}

#[derive(Clone)]
pub(crate) struct IfCommand {
    package: String,
}

impl IfCommand {
    /// Creates and returns a new instance.
    pub(crate) fn new(package: &str) -> IfCommand {
        IfCommand {
            package: package.to_string(),
        }
    }
}

impl Command for IfCommand {
    fn name(&self) -> String {
        pckg::concat(&self.package, "If")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["if".to_string()]
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
            match get_or_create_if_meta_info_for_line(
                line,
                state,
                instructions,
                self.package.clone(),
            ) {
                Ok(if_else_info) => {
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
                                let next_line = if if_else_info.else_lines.is_empty() {
                                    if_else_info.end
                                } else {
                                    if_else_info.else_lines[0]
                                };

                                let line_context_name = get_line_context_name(state);

                                let call_info = CallInfo {
                                    current: next_line,
                                    passed,
                                    else_line_index: 0,
                                    meta_info: if_else_info.clone(),
                                    line_context_name,
                                };

                                store_call_info(&call_info, state);

                                CommandResult::Continue(None)
                            } else if if_else_info.else_lines.is_empty() {
                                let next_line = if_else_info.end + 1;
                                CommandResult::GoTo(None, GoToValue::Line(next_line))
                            } else {
                                let next_line = if_else_info.else_lines[0];

                                let line_context_name = get_line_context_name(state);

                                let call_info = CallInfo {
                                    current: next_line,
                                    passed: false,
                                    else_line_index: 0,
                                    meta_info: if_else_info.clone(),
                                    line_context_name,
                                };

                                store_call_info(&call_info, state);

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
struct ElseIfCommand {
    package: String,
}

impl Command for ElseIfCommand {
    fn name(&self) -> String {
        pckg::concat(&self.package, "ElseIf")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["elif".to_string(), "elseif".to_string()]
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
            match pop_call_info_for_line(line, state) {
            Some(call_info) => {
                if call_info.passed {
                    let next_line = call_info.meta_info.end + 1;
                    CommandResult::GoTo(None, GoToValue::Line(next_line))
                } else {
                    let if_else_info = call_info.meta_info.clone();
                    let line_context_name = get_line_context_name(state);
                    match condition::eval_condition(arguments, instructions, state, variables, commands, env) {
                        Ok(passed) => {
                            if passed {
                                let next_line = if call_info.else_line_index + 1 < if_else_info.else_lines.len() {
                                    if_else_info.else_lines[call_info.else_line_index + 1]
                                } else {
                                    if_else_info.else_lines[0]
                                };

                                let else_call_info = CallInfo {
                                    current: next_line,
                                    passed,
                                    else_line_index: call_info.else_line_index,
                                    meta_info: if_else_info,
                                    line_context_name
                                };

                                store_call_info(&else_call_info, state);

                                CommandResult::Continue(None)
                            } else if call_info.else_line_index + 1 < if_else_info.else_lines.len() {
                                let next_index = call_info.else_line_index + 1;
                                let next_line = if_else_info.else_lines[next_index];

                                let call_info = CallInfo {
                                    current: next_line,
                                    passed: false,
                                    else_line_index: next_index,
                                    meta_info: if_else_info,
                                    line_context_name
                                };

                                store_call_info(&call_info, state);

                                CommandResult::GoTo(None, GoToValue::Line(next_line))
                            } else {
                                let next_line = if_else_info.end + 1;

                                CommandResult::GoTo(None, GoToValue::Line(next_line))
                            }
                        }
                        Err(error) => CommandResult::Error(error.to_string()),
                    }
                }
            },
            None => CommandResult::Error("Found an else-if block but not currently running part of an if/else invocation flow.".to_string())
        }
        }
    }
}

#[derive(Clone)]
struct ElseCommand {
    package: String,
}

impl Command for ElseCommand {
    fn name(&self) -> String {
        pckg::concat(&self.package, "Else")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["else".to_string()]
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
                if call_info.passed {
                    let next_line = call_info.meta_info.end + 1;
                    CommandResult::GoTo(None, GoToValue::Line(next_line))
                } else {
                    CommandResult::Continue(None)
                }
            }
            None => CommandResult::Error(
                "Found an else block but not currently running part of an if/else invocation flow."
                    .to_string(),
            ),
        }
    }
}

#[derive(Clone)]
pub(crate) struct EndIfCommand {
    package: String,
}

impl EndIfCommand {
    /// Creates and returns a new instance.
    pub(crate) fn new(package: &str) -> EndIfCommand {
        EndIfCommand {
            package: package.to_string(),
        }
    }
}

impl Command for EndIfCommand {
    fn name(&self) -> String {
        pckg::concat(&self.package, "EndIf")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["end_if".to_string(), "endif".to_string(), "fi".to_string()]
    }

    fn help(&self) -> String {
        "".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, _arguments: Vec<String>) -> CommandResult {
        CommandResult::Continue(None)
    }
}

pub(crate) fn create(package: &str) -> Vec<Box<dyn Command>> {
    vec![
        Box::new(IfCommand {
            package: package.to_string(),
        }),
        Box::new(ElseIfCommand {
            package: package.to_string(),
        }),
        Box::new(ElseCommand {
            package: package.to_string(),
        }),
        Box::new(EndIfCommand {
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
