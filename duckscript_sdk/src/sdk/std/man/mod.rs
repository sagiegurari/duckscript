use crate::utils::pckg;
use duckscript::types::command::{Command, CommandResult, Commands};
use duckscript::types::env::Env;
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

fn print_help(env: &mut Env, help_doc: String, name: &str) -> CommandResult {
    if help_doc.is_empty() {
        writeln!(env.out, "No documentation found for command: {}", name).unwrap();
        CommandResult::Continue(None)
    } else {
        writeln!(env.out, "{}", &help_doc).unwrap();
        CommandResult::Continue(Some(help_doc))
    }
}

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "ShowCommandDocumentation")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["man".to_string()]
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
        _state: &mut HashMap<String, StateValue>,
        _variables: &mut HashMap<String, String>,
        _output_variable: Option<String>,
        _instructions: &Vec<Instruction>,
        commands: &mut Commands,
        _line: usize,
        env: &mut Env,
    ) -> CommandResult {
        if arguments.is_empty() {
            print_help(env, self.help(), &self.name())
        } else {
            let name = &arguments[0];

            match commands.get(name) {
                Some(command) => {
                    let help_doc = command.help();
                    print_help(env, help_doc, name)
                }
                None => {
                    if name == &self.name() || self.aliases().contains(name) {
                        print_help(env, self.help(), &self.name())
                    } else {
                        writeln!(env.out, "Command: {} not found.", name).unwrap();
                        CommandResult::Continue(None)
                    }
                }
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
