use crate::sdk::std::print::run_print;
use crate::utils::pckg;
use duckscript::types::command::{Command, CommandArgs, CommandResult};

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "Println")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["println".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        let result = run_print(arguments.env, arguments.args);

        if let CommandResult::Continue(ref _value) = result {
            match writeln!(arguments.env.out, "") {
                Ok(_) => result,
                Err(error) => CommandResult::Error(error.to_string()),
            }
        } else {
            result
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
