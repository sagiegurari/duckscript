use crate::utils::pckg;
use duckscript::runner;
use duckscript::types::command::{Command, CommandResult, Commands};
use duckscript::types::env::Env;
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::{Context, StateValue};
use std::collections::HashMap;
use walkdir::DirEntry;
use walkdir::WalkDir;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

fn is_test_file(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|name| name.ends_with("_test.ds"))
        .unwrap_or(false)
        && entry.file_type().is_file()
}

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "TestDirectory")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["test_directory".to_string()]
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
        _env: &mut Env,
    ) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Crash("Directory name not provided.".to_string())
        } else {
            let mut script = String::new();

            let test_name = if arguments.len() > 1 {
                arguments[1].clone()
            } else {
                "".to_string()
            };

            let walker = WalkDir::new(&arguments[0])
                .sort_by(|entry1, entry2| entry1.file_name().cmp(entry2.file_name()))
                .into_iter();
            for entry in walker {
                let entry = entry.unwrap();
                if is_test_file(&entry) {
                    let file = entry.path().display().to_string().replace("\\", "/");

                    let test_script = format!(
                        r#"
result = test_file {} {}
assert result
"#,
                        &file, &test_name
                    );
                    script.push_str(&test_script);
                }
            }

            let mut context = Context::new();
            context.commands = commands.clone();

            match runner::run_script(&script, context, None) {
                Err(error) => CommandResult::Crash(
                    format!("Error while running tests.\n{}", &error.to_string()).to_string(),
                ),
                _ => CommandResult::Continue(Some("true".to_string())),
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
