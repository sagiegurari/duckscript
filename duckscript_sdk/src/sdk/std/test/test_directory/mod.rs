use crate::utils::pckg;
use duckscript::runner;
use duckscript::types::command::{Command, CommandInvocationContext, CommandResult};
use duckscript::types::runtime::Context;
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

    fn run(&self, context: CommandInvocationContext) -> CommandResult {
        if context.arguments.is_empty() {
            CommandResult::Crash("Directory name not provided.".to_string())
        } else {
            let mut script = String::new();

            let test_name = if context.arguments.len() > 1 {
                context.arguments[1].clone()
            } else {
                "".to_string()
            };

            let walker = WalkDir::new(&context.arguments[0])
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

            let mut runner_context = Context::new();
            runner_context.commands = context.commands.clone();

            match runner::run_script(&script, runner_context, None) {
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
