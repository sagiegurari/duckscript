use crate::utils::instruction_query;
use crate::utils::pckg;
use crate::utils::state::{get_core_sub_state_for_command, get_list, get_sub_state};
use duckscript::types::command::{Command, CommandResult, Commands, GoToValue};
use duckscript::types::error::ScriptError;
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

static FUNCTION_STATE_KEY: &str = "function";
static META_INFO_STATE_KEY: &str = "meta_info";
static CALL_STACK_STATE_KEY: &str = "call_stack";

#[derive(Debug)]
struct FunctionMetaInfo {
    pub(crate) name: String,
    pub(crate) start: usize,
    pub(crate) end: usize,
}

#[derive(Debug)]
struct CallInfo {
    pub(crate) call_line: usize,
    pub(crate) start_line: usize,
    pub(crate) end_line: usize,
    pub(crate) output_variable: Option<String>,
}

fn store_fn_info_in_state(
    state: &mut HashMap<String, StateValue>,
    meta_info: &FunctionMetaInfo,
) -> Result<(), String> {
    let fn_state = get_core_sub_state_for_command(state, FUNCTION_STATE_KEY.to_string());
    let stored_fn_option = get_fn_info_from_state(fn_state, &meta_info.name);

    if let Some(stored_fn) = stored_fn_option {
        if stored_fn.start != meta_info.start || stored_fn.end != meta_info.end {
            return Err(format!(
                "Function: {} already defined at: {} to: {}",
                &stored_fn.name, stored_fn.start, stored_fn.end
            )
            .to_string());
        }
    }

    let meta_info_state = get_sub_state(META_INFO_STATE_KEY.to_string(), fn_state);
    let fn_info_state = get_sub_state(meta_info.name.clone(), meta_info_state);

    fn_info_state.insert(
        "start".to_string(),
        StateValue::UnsignedNumber(meta_info.start),
    );
    fn_info_state.insert("end".to_string(), StateValue::UnsignedNumber(meta_info.end));

    Ok(())
}

fn get_fn_info_from_state(
    state: &mut HashMap<String, StateValue>,
    name: &str,
) -> Option<FunctionMetaInfo> {
    let fn_state = get_core_sub_state_for_command(state, FUNCTION_STATE_KEY.to_string());
    let mut meta_info_state = get_sub_state(META_INFO_STATE_KEY.to_string(), fn_state);
    let fn_info_state = get_sub_state(name.to_string(), &mut meta_info_state);

    if fn_info_state.contains_key("start") {
        let start = match fn_info_state.get("start") {
            Some(state_value) => match state_value {
                StateValue::UnsignedNumber(value) => *value,
                _ => return None, //ignore corrupted data
            },
            _ => return None,
        };
        let end = match fn_info_state.get("end") {
            Some(state_value) => match state_value {
                StateValue::UnsignedNumber(value) => *value,
                _ => return None, //ignore corrupted data
            },
            _ => return None,
        };

        Some(FunctionMetaInfo {
            name: name.to_string(),
            start,
            end,
        })
    } else {
        None
    }
}

fn push_to_call_stack(state: &mut HashMap<String, StateValue>, call_info: &CallInfo) {
    let fn_state = get_core_sub_state_for_command(state, FUNCTION_STATE_KEY.to_string());
    let call_stack = get_list(CALL_STACK_STATE_KEY.to_string(), fn_state);

    let mut sub_state = HashMap::new();
    sub_state.insert(
        "call_line".to_string(),
        StateValue::UnsignedNumber(call_info.call_line),
    );
    sub_state.insert(
        "start_line".to_string(),
        StateValue::UnsignedNumber(call_info.start_line),
    );
    sub_state.insert(
        "end_line".to_string(),
        StateValue::UnsignedNumber(call_info.end_line),
    );
    if let Some(output_variable) = &call_info.output_variable {
        sub_state.insert(
            "output_variable".to_string(),
            StateValue::String(output_variable.to_string()),
        );
    }

    call_stack.push(StateValue::SubState(sub_state));
}

fn pop_from_call_stack(state: &mut HashMap<String, StateValue>) -> Option<CallInfo> {
    let fn_state = get_core_sub_state_for_command(state, FUNCTION_STATE_KEY.to_string());
    let call_stack = get_list(CALL_STACK_STATE_KEY.to_string(), fn_state);

    match call_stack.pop() {
        Some(sub_state_value) => match sub_state_value {
            StateValue::SubState(sub_state) => {
                let call_line = match sub_state.get("call_line") {
                    Some(value) => match value {
                        StateValue::UnsignedNumber(call_line) => *call_line,
                        _ => return pop_from_call_stack(state),
                    },
                    None => return pop_from_call_stack(state),
                };

                let start_line = match sub_state.get("start_line") {
                    Some(value) => match value {
                        StateValue::UnsignedNumber(start_line) => *start_line,
                        _ => return pop_from_call_stack(state),
                    },
                    None => return pop_from_call_stack(state),
                };

                let end_line = match sub_state.get("end_line") {
                    Some(value) => match value {
                        StateValue::UnsignedNumber(end_line) => *end_line,
                        _ => return pop_from_call_stack(state),
                    },
                    None => return pop_from_call_stack(state),
                };

                let output_variable = match sub_state.get("output_variable") {
                    Some(value) => match value {
                        StateValue::String(output_variable) => Some(output_variable.to_string()),
                        _ => return pop_from_call_stack(state),
                    },
                    None => None,
                };

                Some(CallInfo {
                    call_line,
                    start_line,
                    end_line,
                    output_variable,
                })
            }
            _ => return pop_from_call_stack(state),
        },
        None => None,
    }
}

fn run_call(
    function_name: String,
    arguments: Vec<String>,
    state: &mut HashMap<String, StateValue>,
    variables: &mut HashMap<String, String>,
    output_variable: Option<String>,
    line: usize,
) -> CommandResult {
    match get_fn_info_from_state(state, &function_name) {
        Some(fn_info) => {
            // define function arguments
            let mut index = 0;
            for argument in arguments {
                index = index + 1;

                variables.insert(index.to_string(), argument);
            }

            // store to call stack
            let call_info = CallInfo {
                call_line: line,
                start_line: fn_info.start,
                end_line: fn_info.end,
                output_variable,
            };
            push_to_call_stack(state, &call_info);

            CommandResult::GoTo(None, GoToValue::Line(fn_info.start + 1))
        }
        None => {
            CommandResult::Error(format!("Function: {} not found.", &function_name).to_string())
        }
    }
}

struct FunctionCommand {
    package: String,
}

impl Command for FunctionCommand {
    fn name(&self) -> String {
        pckg::concat(&self.package, "Function")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["function".to_string(), "fn".to_string()]
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
        _variables: &mut HashMap<String, String>,
        _output_variable: Option<String>,
        instructions: &Vec<Instruction>,
        commands: &mut Commands,
        line: usize,
    ) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Error("Missing function name.".to_string())
        } else {
            let function_name = arguments[0].clone();
            match get_fn_info_from_state(state, &function_name) {
                Some(fn_info) => {
                    if fn_info.start != line {
                        CommandResult::Error(
                            format!(
                                "Function: {} already defined at: {} to: {}",
                                &fn_info.name, fn_info.start, fn_info.end
                            )
                            .to_string(),
                        )
                    } else {
                        CommandResult::GoTo(None, GoToValue::Line(fn_info.end + 1))
                    }
                }
                None => {
                    let mut start_names = self.aliases();
                    start_names.push(self.name());
                    let end_command = EndFunctionCommand {
                        package: self.package.clone(),
                    };
                    let mut end_names = end_command.aliases();
                    end_names.push(end_command.name());

                    match instruction_query::find_command(
                        &instructions,
                        &end_names,
                        Some(line + 1),
                        None,
                        &start_names,
                    ) {
                        Ok(fn_end_line_option) => match fn_end_line_option {
                            Some(fn_end_line) => {
                                let fn_info = FunctionMetaInfo {
                                    name: function_name.clone(),
                                    start: line,
                                    end: fn_end_line,
                                };

                                match store_fn_info_in_state(state, &fn_info) {
                                    Ok(_) => {
                                        pub(crate) struct CallFunctionCommand {
                                            name: String,
                                        }

                                        impl Command for CallFunctionCommand {
                                            fn name(&self) -> String {
                                                self.name.clone()
                                            }

                                            fn help(&self) -> String {
                                                "".to_string()
                                            }

                                            fn requires_context(&self) -> bool {
                                                true
                                            }

                                            fn run_with_context(
                                                &self,
                                                arguments: Vec<String>,
                                                state: &mut HashMap<String, StateValue>,
                                                variables: &mut HashMap<String, String>,
                                                output_variable: Option<String>,
                                                _instructions: &Vec<Instruction>,
                                                _commands: &mut Commands,
                                                line: usize,
                                            ) -> CommandResult
                                            {
                                                run_call(
                                                    self.name(),
                                                    arguments,
                                                    state,
                                                    variables,
                                                    output_variable,
                                                    line,
                                                )
                                            }
                                        }

                                        match commands.set(Box::new(CallFunctionCommand {
                                            name: function_name.clone(),
                                        })) {
                                            Ok(_) => CommandResult::GoTo(
                                                None,
                                                GoToValue::Line(fn_end_line + 1),
                                            ),
                                            Err(error) => CommandResult::Error(error.to_string()),
                                        }
                                    }
                                    Err(error) => CommandResult::Error(error),
                                }
                            }
                            None => CommandResult::Error(
                                format!("Function: {} end not found.", &function_name).to_string(),
                            ),
                        },
                        Err(error) => CommandResult::Error(error),
                    }
                }
            }
        }
    }
}

struct EndFunctionCommand {
    package: String,
}

impl Command for EndFunctionCommand {
    fn name(&self) -> String {
        pckg::concat(&self.package, "EndFunction")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["end_function".to_string(), "end_fn".to_string()]
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
        match pop_from_call_stack(state) {
            Some(call_info) => {
                if call_info.end_line == line {
                    let next_line = call_info.call_line + 1;
                    CommandResult::GoTo(None, GoToValue::Line(next_line))
                } else {
                    push_to_call_stack(state, &call_info);
                    CommandResult::Continue(None)
                }
            }
            None => CommandResult::Continue(None),
        }
    }
}

struct ReturnCommand {
    package: String,
}

impl Command for ReturnCommand {
    fn name(&self) -> String {
        pckg::concat(&self.package, "Return")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["return".to_string()]
    }

    fn help(&self) -> String {
        "".to_string()
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
        _instructions: &Vec<Instruction>,
        _commands: &mut Commands,
        line: usize,
    ) -> CommandResult {
        match pop_from_call_stack(state) {
            Some(call_info) => {
                if call_info.start_line < line && call_info.end_line > line {
                    match call_info.output_variable {
                        Some(name) => {
                            if arguments.is_empty() {
                                variables.remove(&name);
                            } else {
                                variables.insert(name, arguments[0].clone());
                            }
                        }
                        None => (),
                    };

                    let next_line = call_info.call_line + 1;
                    CommandResult::GoTo(None, GoToValue::Line(next_line))
                } else {
                    push_to_call_stack(state, &call_info);
                    CommandResult::Continue(None)
                }
            }
            None => CommandResult::Continue(None),
        }
    }
}

pub(crate) fn create(package: &str) -> Vec<Box<dyn Command>> {
    vec![
        Box::new(FunctionCommand {
            package: package.to_string(),
        }),
        Box::new(EndFunctionCommand {
            package: package.to_string(),
        }),
        Box::new(ReturnCommand {
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
