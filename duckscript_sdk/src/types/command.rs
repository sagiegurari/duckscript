use crate::types::scope::clear;
use crate::utils::state::{get_handles_sub_state, put_handle};
use duckscript::types::command::{Command, CommandResult, Commands};
use duckscript::types::instruction::{Instruction, InstructionType};
use duckscript::types::runtime::StateValue;
use duckscript::{parser, runner};
use std::collections::HashMap;

pub(crate) struct AliasCommand {
    name: String,
    aliases: Vec<String>,
    help: String,
    scope_name: String,
    raw_command: String,
    arguments_amount: usize,
}

impl Command for AliasCommand {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn aliases(&self) -> Vec<String> {
        self.aliases.clone()
    }

    fn help(&self) -> String {
        format!(
            r#"
{}

#### Source:

```sh
{}
```
"#,
            &self.help, &self.raw_command
        )
        .to_string()
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
        commands: &mut Commands,
        _line: usize,
    ) -> CommandResult {
        if arguments.len() != self.arguments_amount {
            CommandResult::Error("Invalid arguments provided.".to_string())
        } else {
            let mut scope_name = "scope::".to_string();
            scope_name.push_str(&self.scope_name);

            // define script arguments
            let mut handle_option = None;
            if !arguments.is_empty() {
                let mut index = 0;
                let mut array = vec![];
                for argument in arguments {
                    index = index + 1;
                    let mut key = scope_name.clone();
                    key.push_str("::argument::");
                    key.push_str(&index.to_string());

                    variables.insert(key, argument.clone());

                    array.push(StateValue::String(argument.clone()));
                }

                let handle = put_handle(state, StateValue::List(array));

                let mut key = scope_name.clone();
                key.push_str("::arguments");

                variables.insert(key, handle.clone());
                handle_option = Some(handle);
            }

            match parser::parse_text(&self.raw_command) {
                Ok(instructions) => {
                    let mut line = 0;
                    let mut flow_output = None;
                    for instruction in instructions {
                        match instruction.instruction_type {
                            InstructionType::Script(ref script_instruction) => {
                                let (command_result, _) = runner::run_instruction(
                                    commands,
                                    variables,
                                    state,
                                    &vec![],
                                    instruction.clone(),
                                    line,
                                );

                                match command_result {
                                    CommandResult::Exit(output) => {
                                        return CommandResult::Exit(output);
                                    }
                                    CommandResult::Error(error) => {
                                        return CommandResult::Error(error);
                                    }
                                    CommandResult::Crash(error) => {
                                        return CommandResult::Crash(error);
                                    }
                                    CommandResult::GoTo(_, _) => {
                                        return CommandResult::Error(
                                            "goto result not supported in alias command flow."
                                                .to_string(),
                                        );
                                    }
                                    CommandResult::Continue(output) => {
                                        flow_output = output.clone();

                                        match script_instruction.output {
                                            Some(ref output_variable) => {
                                                match output {
                                                    Some(value) => variables
                                                        .insert(output_variable.to_string(), value),
                                                    None => variables.remove(output_variable),
                                                };
                                            }
                                            None => (),
                                        };

                                        line = line + 1;
                                    }
                                };
                            }
                            _ => (),
                        };
                    }

                    match handle_option {
                        Some(handle) => {
                            let handle_state = get_handles_sub_state(state);
                            match handle_state.remove(&handle) {
                                _ => (),
                            }
                        }
                        None => (),
                    }
                    clear(&scope_name, variables);

                    CommandResult::Continue(flow_output)
                }
                Err(error) => CommandResult::Error(error.to_string()),
            }
        }
    }
}

pub(crate) fn create_alias_command(
    name: String,
    aliases: Vec<String>,
    help: String,
    scope_name: String,
    script: String,
    arguments_amount: usize,
) -> AliasCommand {
    let raw_command = script.trim().to_string();

    AliasCommand {
        name,
        aliases,
        help,
        scope_name,
        raw_command,
        arguments_amount,
    }
}
