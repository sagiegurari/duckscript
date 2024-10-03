use crate::utils::pckg;
use duckscript::types::command::{Command, CommandArgs, CommandResult};
use rand::{thread_rng, Rng};

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "Range")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["random_range".to_string(), "rand_range".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        if arguments.args.len() < 2 {
            CommandResult::Error("Missing random min/max values.".to_string())
        } else {
            match arguments.args[0].parse() {
                Ok(min) => match arguments.args[1].parse() {
                    Ok(max) => {
                        if min > max {
                            CommandResult::Error(
                                format!("Min value: {} bigger than max value: {}", min, max)
                                    .to_string(),
                            )
                        } else {
                            let mut rng = thread_rng();

                            let min_128: i128 = min;
                            let max_128: i128 = max;
                            let rand_value: i128 = rng.gen_range(min_128..max_128);

                            CommandResult::Continue(Some(rand_value.to_string()))
                        }
                    }
                    Err(_) => CommandResult::Error(
                        format!("Non numeric max value: {} provided.", &arguments.args[1])
                            .to_string(),
                    ),
                },
                Err(_) => CommandResult::Error(
                    format!("Non numeric min value: {} provided.", &arguments.args[0]).to_string(),
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
