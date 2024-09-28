use crate::sdk::std::flowcontrol::{end, forin, ifelse, while_mod};
use crate::types::scope::get_line_context_name;
use crate::utils::state::{get_core_sub_state_for_command, get_list, get_sub_state};
use crate::utils::{annotation, instruction_query, pckg, scope};
use duckscript::types::command::{Command, CommandResult, Commands, GoToValue};
use duckscript::types::env::Env;
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
    pub(crate) scoped: bool,
}

#[derive(Debug)]
struct CallInfo {
    pub(crate) call_line: usize,
    pub(crate) start_line: usize,
    pub(crate) end_line: usize,
    pub(crate) line_context_name: String,
    pub(crate) output_variable: Option<String>,
    pub(crate) scoped: bool,
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
    fn_info_state.insert("scoped".to_string(), StateValue::Boolean(meta_info.scoped));

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

        let scoped = match fn_info_state.get("scoped") {
            Some(state_value) => match state_value {
                StateValue::Boolean(value) => *value,
                _ => false,
            },
            _ => false,
        };

        Some(FunctionMetaInfo {
            name: name.to_string(),
            start,
            end,
            scoped,
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
    sub_state.insert(
        "line_context_name".to_string(),
        StateValue::String(call_info.line_context_name.clone()),
    );
    if let Some(output_variable) = &call_info.output_variable {
        sub_state.insert(
            "output_variable".to_string(),
            StateValue::String(output_variable.to_string()),
        );
    }
    sub_state.insert("scoped".to_string(), StateValue::Boolean(call_info.scoped));

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

                let line_context_name = match sub_state.get("line_context_name") {
                    Some(value) => match value {
                        StateValue::String(line_context_name) => line_context_name.clone(),
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

                let scoped = match sub_state.get("scoped") {
                    Some(value) => match value {
                        StateValue::Boolean(scoped) => *scoped,
                        _ => false,
                    },
                    None => false,
                };

                Some(CallInfo {
                    call_line,
                    start_line,
                    end_line,
                    line_context_name,
                    output_variable,
                    scoped,
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
            if fn_info.scoped {
                match scope::push(variables, state, &vec![]) {
                    Err(error) => return CommandResult::Error(error),
                    _ => (),
                }
            }

            // define function arguments
            let mut index = 0;
            for argument in arguments {
                index = index + 1;

                variables.insert(index.to_string(), argument);
            }

            let line_context_name = get_line_context_name(state);

            // store to call stack
            let call_info = CallInfo {
                call_line: line,
                start_line: fn_info.start,
                end_line: fn_info.end,
                line_context_name,
                output_variable,
                scoped: fn_info.scoped,
            };
            push_to_call_stack(state, &call_info);

            CommandResult::GoTo(None, GoToValue::Line(fn_info.start + 1))
        }
        None => {
            CommandResult::Error(format!("Function: {} not found.", &function_name).to_string())
        }
    }
}

#[derive(Clone)]
pub(crate) struct FunctionCommand {
    package: String,
}

impl FunctionCommand {
    /// Creates and returns a new instance.
    pub(crate) fn new(package: &str) -> FunctionCommand {
        FunctionCommand {
            package: package.to_string(),
        }
    }
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
        _variables: &mut HashMap<String, String>,
        _output_variable: Option<String>,
        instructions: &Vec<Instruction>,
        commands: &mut Commands,
        line: usize,
        _env: &mut Env,
    ) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Error("Missing function name.".to_string())
        } else {
            let (function_name, scoped) = if arguments.len() == 1 {
                (arguments[0].clone(), false)
            } else {
                match annotation::parse(&arguments[0]) {
                    Some(annotations) => (
                        arguments[1].clone(),
                        annotations.contains(&"scope".to_string()),
                    ),
                    None => (arguments[0].clone(), false),
                }
            };

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
                    end_names.push(end::END_COMMAND_NAME.to_string());

                    let if_command = ifelse::IfCommand::new(&self.package);
                    let mut start_blocks = if_command.aliases();
                    start_blocks.push(if_command.name());
                    let forin_command = forin::ForInCommand::new(&self.package);
                    start_blocks.append(&mut forin_command.aliases());
                    start_blocks.push(forin_command.name());
                    let while_command = while_mod::WhileCommand::new(&self.package);
                    start_blocks.append(&mut while_command.aliases());
                    start_blocks.push(while_command.name());

                    let end_if_command = ifelse::EndIfCommand::new(&self.package);
                    let mut end_blocks = end_if_command.aliases();
                    end_blocks.push(end_if_command.name());
                    let end_forin_command = forin::EndForInCommand::new(&self.package);
                    end_blocks.append(&mut end_forin_command.aliases());
                    end_blocks.push(end_forin_command.name());
                    let end_while_command = while_mod::EndWhileCommand::new(&self.package);
                    end_blocks.append(&mut end_while_command.aliases());
                    end_blocks.push(end_while_command.name());
                    end_blocks.push(end::END_COMMAND_NAME.to_string());

                    match instruction_query::find_commands(
                        instructions,
                        &start_names,
                        &vec![],
                        &end_names,
                        Some(line + 1),
                        None,
                        false,
                        &start_blocks,
                        &end_blocks,
                    ) {
                        Ok(positions_options) => match positions_options {
                            Some(positions) => {
                                let fn_end_line = positions.end;

                                let fn_info = FunctionMetaInfo {
                                    name: function_name.clone(),
                                    start: line,
                                    end: fn_end_line,
                                    scoped,
                                };

                                end::set_command(fn_end_line, state, end_command.name());

                                match store_fn_info_in_state(state, &fn_info) {
                                    Ok(_) => {
                                        #[derive(Clone)]
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
                                                output_variable: Option<String>,
                                                _instructions: &Vec<Instruction>,
                                                _commands: &mut Commands,
                                                line: usize,
                                                _env: &mut Env,
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
                            None => CommandResult::Crash(
                                format!("Function: {} end not found.", &function_name).to_string(),
                            ),
                        },
                        Err(error) => CommandResult::Crash(error),
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
pub(crate) struct EndFunctionCommand {
    package: String,
}

impl EndFunctionCommand {
    /// Creates and returns a new instance.
    pub(crate) fn new(package: &str) -> EndFunctionCommand {
        EndFunctionCommand {
            package: package.to_string(),
        }
    }
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
        variables: &mut HashMap<String, String>,
        _output_variable: Option<String>,
        _instructions: &Vec<Instruction>,
        _commands: &mut Commands,
        line: usize,
        _env: &mut Env,
    ) -> CommandResult {
        let line_context_name = get_line_context_name(state);

        match pop_from_call_stack(state) {
            Some(call_info) => {
                if call_info.end_line == line && call_info.line_context_name == line_context_name {
                    let next_line = call_info.call_line + 1;

                    if call_info.scoped {
                        match scope::pop(variables, state, &vec![]) {
                            Err(error) => return CommandResult::Error(error),
                            _ => (),
                        }
                    }

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

#[derive(Clone)]
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
        _instructions: &Vec<Instruction>,
        _commands: &mut Commands,
        line: usize,
        _env: &mut Env,
    ) -> CommandResult {
        let line_context_name = get_line_context_name(state);

        match pop_from_call_stack(state) {
            Some(call_info) => {
                if call_info.start_line < line
                    && call_info.end_line > line
                    && call_info.line_context_name == line_context_name
                {
                    match call_info.output_variable {
                        Some(ref name) => {
                            if arguments.is_empty() {
                                variables.remove(name);
                            } else {
                                variables.insert(name.to_string(), arguments[0].clone());
                            }
                        }
                        None => (),
                    };

                    let output = if arguments.is_empty() {
                        None
                    } else {
                        Some(arguments[0].clone())
                    };

                    if call_info.scoped {
                        let copy = match call_info.output_variable {
                            Some(ref name) => vec![name.to_string()],
                            None => vec![],
                        };

                        match scope::pop(variables, state, &copy) {
                            Err(error) => return CommandResult::Error(error),
                            _ => (),
                        }
                    }

                    let next_line = call_info.call_line + 1;
                    CommandResult::GoTo(output, GoToValue::Line(next_line))
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
