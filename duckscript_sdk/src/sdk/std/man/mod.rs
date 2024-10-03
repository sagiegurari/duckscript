use crate::utils::pckg;
use duckscript::types::command::{Command, CommandArgs, CommandResult};
use duckscript::types::env::Env;

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

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        if arguments.args.is_empty() {
            print_help(arguments.env, self.help(), &self.name())
        } else {
            let name = &arguments.args[0];

            match arguments.commands.get(name) {
                Some(command) => {
                    let help_doc = command.help();
                    print_help(arguments.env, help_doc, name)
                }
                None => {
                    if name == &self.name() || self.aliases().contains(name) {
                        print_help(arguments.env, self.help(), &self.name())
                    } else {
                        writeln!(arguments.env.out, "Command: {} not found.", name).unwrap();
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
