use crate::utils::pckg;
use duckscript::types::command::{Command, CommandArgs, CommandResult, Commands};
use duckscript::types::env::Env;
use duckscript::types::instruction::{Instruction, InstructionType};
use duckscript::types::runtime::{Context, StateValue};
use duckscript::{parser, runner};
use std::collections::HashMap;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

fn create_test_script(file: &str, test_name: &str) -> String {
    format!(
        r#"
!include_files {}
{}
"#,
        file, test_name
    )
    .to_string()
}

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "TestFile")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["test_file".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        if arguments.args.is_empty() {
            CommandResult::Crash("File name not provided.".to_string())
        } else {
            let file = arguments.args[0].clone();
            let requested_test_name = if arguments.args.len() > 1 {
                arguments.args[1].clone()
            } else {
                "".to_string()
            };

            match parser::parse_file(&arguments.args[0]) {
                Ok(instructions) => match arguments.commands.get("function") {
                    Some(function_command) => {
                        let mut command_names = function_command.aliases();
                        command_names.push(function_command.name());

                        let mut test_names = vec![];
                        for instruction in instructions {
                            match instruction.instruction_type {
                                InstructionType::Script(ref script_instruction) => {
                                    match (
                                        script_instruction.command.as_ref(),
                                        script_instruction.arguments.as_ref(),
                                    ) {
                                        (Some(current_command), Some(current_arguments)) => {
                                            if !current_arguments.args.is_empty()
                                                && command_names.contains(&current_command)
                                            {
                                                if current_arguments.args[0].starts_with("test_") {
                                                    test_names
                                                        .push(current_arguments.args[0].clone());
                                                }
                                            }
                                        }
                                        _ => (),
                                    };
                                }
                                _ => (),
                            };
                        }

                        let file_included = file.contains(&requested_test_name);

                        for test_name in test_names {
                            if file_included || test_name.contains(&requested_test_name) {
                                let script = create_test_script(&file, &test_name);

                                let mut context = Context::new();
                                context.commands = commands.clone();

                                match runner::run_script(&script, context, None) {
                                    Err(error) => {
                                        writeln!(
                                            arguments.env.out,
                                            "test: [{}][{}] ... failed",
                                            &file, &test_name
                                        )
                                        .unwrap();

                                        return CommandResult::Crash(
                                            format!(
                                                "Error while running test: {}\n{}",
                                                &test_name,
                                                &error.to_string()
                                            )
                                            .to_string(),
                                        );
                                    }
                                    _ => writeln!(
                                        arguments.env.out,
                                        "test: [{}][{}] ... ok",
                                        &file, &test_name
                                    )
                                    .unwrap(),
                                }
                            }
                        }

                        CommandResult::Continue(Some("true".to_string()))
                    }
                    None => {
                        CommandResult::Crash("Unable to find the function command.".to_string())
                    }
                },
                Err(error) => CommandResult::Crash(
                    format!(
                        "Error while parsing file: {}\n{}",
                        &arguments.args[0],
                        &error.to_string()
                    )
                    .to_string(),
                ),
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
