use crate::utils::pckg;
use crate::utils::state::{get_handles_sub_state, mutate_map};
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
        pckg::concat(&self.package, "MapClear")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["map_clear".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        if arguments.args.is_empty() {
            CommandResult::Error("Map handle not provided.".to_string())
        } else {
            let state = get_handles_sub_state(arguments.state);

            let key = arguments.args[0].clone();

            let result = mutate_map(key, state, |map| {
                map.clear();

                Ok(None)
            });

            match result {
                Ok(_) => CommandResult::Continue(Some("true".to_string())),
                Err(error) => CommandResult::Error(error),
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
