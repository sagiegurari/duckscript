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
        pckg::concat(&self.package, "GetOSFamily")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["os_family".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, _arguments: CommandArgs) -> CommandResult {
        let value = if cfg!(windows) {
            "windows".to_string()
        } else if cfg!(target_os = "macos") || cfg!(target_os = "ios") {
            "mac".to_string()
        } else {
            "linux".to_string()
        };

        CommandResult::Continue(Some(value))
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
