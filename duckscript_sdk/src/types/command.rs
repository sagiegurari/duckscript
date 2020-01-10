use duckscript::types::command::{Command, CommandResult, Commands};
use duckscript::types::instruction::{Instruction, InstructionType};
use duckscript::types::runtime::StateValue;
use duckscript::{parser, runner};
use std::collections::HashMap;

#[cfg(test)]
#[path = "./command_test.rs"]
mod command_test;

pub(crate) struct AliasCommand {
    name: String,
    aliases: Vec<String>,
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
Alias for:

```sh
{}
```
"#,
            &self.raw_command
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
            // define function arguments
            if !arguments.is_empty() {
                let mut index = 0;
                for argument in arguments {
                    index = index + 1;
                    let mut key = "argument".to_string();
                    key.push_str(&index.to_string());

                    variables.insert(key, argument);
                }
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
    script: String,
    arguments_amount: usize,
) -> AliasCommand {
    let raw_command = script.trim().to_string();

    AliasCommand {
        name,
        aliases,
        raw_command,
        arguments_amount,
    }
}
