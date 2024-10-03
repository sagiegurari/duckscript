use crate::sdk::std::lib::alias::ALIAS_STATE_KEY;
use crate::utils::pckg;
use crate::utils::state::get_sub_state;
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
        pckg::concat(&self.package, "Unset")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["unalias".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        if arguments.args.len() != 1 {
            CommandResult::Error("Invalid alias name provided.".to_string())
        } else {
            let sub_state = get_sub_state(ALIAS_STATE_KEY.to_string(), arguments.state);

            let key = &arguments.args[0];
            let removed = if sub_state.contains_key(key) {
                if arguments.commands.remove(key) {
                    sub_state.remove(key);
                    true
                } else {
                    false
                }
            } else if arguments.commands.aliases.contains_key(key) {
                arguments.commands.aliases.remove(key);
                true
            } else {
                false
            };

            CommandResult::Continue(Some(removed.to_string()))
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
