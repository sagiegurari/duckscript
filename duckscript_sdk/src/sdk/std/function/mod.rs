use crate::utils::instruction_query;
use crate::utils::pckg;
use crate::utils::state::get_sub_state;
use duckscript::types::command::{Command, CommandResult, Commands, GoToValue};
use duckscript::types::error::ScriptError;
use duckscript::types::instruction::{Instruction, InstructionMetaInfo};
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

static FUNCTION_META_INFO_STATE_KEY: &str = "meta_info";

struct FunctionMetaInfo {
    pub(crate) name: String,
    pub(crate) start: usize,
    pub(crate) end: usize,
}

struct _CallInfo {
    pub(crate) call_line: usize,
    pub(crate) arguments: Vec<String>,
    pub(crate) output_variable: Option<String>,
    pub(crate) function_info: FunctionMetaInfo,
}

fn store_fn_info_in_state(
    fn_state: &mut HashMap<String, StateValue>,
    meta_info: &FunctionMetaInfo,
) -> Result<(), String> {
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

    let meta_info_state = get_sub_state(FUNCTION_META_INFO_STATE_KEY.to_string(), fn_state);
    let fn_info_state = get_sub_state(meta_info.name.clone(), meta_info_state);

    fn_info_state.insert(
        "start".to_string(),
        StateValue::UnsignedNumber(meta_info.start),
    );
    fn_info_state.insert("end".to_string(), StateValue::UnsignedNumber(meta_info.end));

    Ok(())
}

fn get_fn_info_from_state(
    fn_state: &mut HashMap<String, StateValue>,
    name: &str,
) -> Option<FunctionMetaInfo> {
    let mut meta_info_state = get_sub_state(FUNCTION_META_INFO_STATE_KEY.to_string(), fn_state);
    let fn_info_state = get_sub_state(name.to_string(), &mut meta_info_state);

    if fn_info_state.contains_key("start") {
        let start = match fn_info_state.get("start") {
            Some(state_value) => match state_value {
                StateValue::UnsignedNumber(value) => *value,
                _ => return None, //ignore corrupted data
            },
            _ => return None,
        };
        let end = match fn_info_state.get("start") {
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
        state: &mut HashMap<String, StateValue>,
        instructions: &Vec<Instruction>,
        commands: &mut Commands,
        arguments: Vec<String>,
        meta_info: InstructionMetaInfo,
    ) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Error("Missing function name.".to_string())
        } else {
            match meta_info.line {
                Some(fn_start_line) => {
                    let function_name = arguments[0].clone();
                    match get_fn_info_from_state(state, &function_name) {
                        Some(fn_info) => {
                            if fn_info.start != fn_start_line {
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
                                Some(fn_start_line + 1),
                                None,
                                &start_names,
                            ) {
                                Ok(fn_end_line_option) => match fn_end_line_option {
                                    Some(fn_end_line) => {
                                        let fn_info = FunctionMetaInfo {
                                            name: function_name.clone(),
                                            start: fn_start_line,
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
                                                        state: &mut HashMap<String, StateValue>,
                                                        _instructions: &Vec<Instruction>,
                                                        _commands: &mut Commands,
                                                        arguments: Vec<String>,
                                                        _meta_info: InstructionMetaInfo,
                                                    ) -> CommandResult
                                                    {
                                                        //TODO set vars!!!
                                                        for argument in &arguments {
                                                            print!("{} ", argument);
                                                        }

                                                        //TODO create call stack!!!

                                                        match get_fn_info_from_state(
                                                            state,
                                                            &self.name(),
                                                        ) {
                                                            Some(fn_info) => CommandResult::GoTo(
                                                                None,
                                                                GoToValue::Line(fn_info.end + 1),
                                                            ),
                                                            None => CommandResult::Error(
                                                                format!(
                                                                    "Function: {} not found.",
                                                                    &self.name
                                                                )
                                                                .to_string(),
                                                            ),
                                                        }
                                                    }
                                                }

                                                match commands.set(Box::new(CallFunctionCommand {
                                                    name: function_name.clone(),
                                                })) {
                                                    Ok(_) => CommandResult::GoTo(
                                                        None,
                                                        GoToValue::Line(fn_end_line + 1),
                                                    ),
                                                    Err(error) => {
                                                        CommandResult::Error(error.to_string())
                                                    }
                                                }
                                            }
                                            Err(error) => CommandResult::Error(error),
                                        }
                                    }
                                    None => CommandResult::Error(
                                        format!("Function: {} end not found.", &function_name)
                                            .to_string(),
                                    ),
                                },
                                Err(error) => CommandResult::Error(error),
                            }
                        }
                    }
                }
                None => CommandResult::Error(
                    "Missing function line number, meta info is partial.".to_string(),
                ),
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
        _state: &mut HashMap<String, StateValue>,
        _instructions: &Vec<Instruction>,
        _commands: &mut Commands,
        _arguments: Vec<String>,
        _meta_info: InstructionMetaInfo,
    ) -> CommandResult {
        CommandResult::Continue(None)
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
    ]
}

pub(crate) fn load(commands: &mut Commands, package: &str) -> Result<(), ScriptError> {
    let multi_commands = create(package);

    for command in multi_commands {
        commands.set(command)?;
    }

    Ok(())
}
