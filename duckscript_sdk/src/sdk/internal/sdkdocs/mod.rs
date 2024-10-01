use crate::utils::io;
use crate::utils::pckg;
use duckscript::types::command::{Command, CommandArgs, CommandResult};
use duckscript::types::env::Env;
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "SDKDocsGen")
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        if arguments.args.is_empty() {
            CommandResult::Error("Documentation output directory not provided.".to_string())
        } else {
            let (file, prefix) = if arguments.args.len() == 1 {
                (arguments.args[0].clone(), "".to_string())
            } else {
                (arguments.args[1].clone(), arguments.args[0].clone())
            };

            let names = arguments.commands.get_all_command_names();
            let mut buffer = String::new();

            // create ToC
            buffer.push_str("# Table of Contents\n");
            for name in &names {
                if name.starts_with(&prefix) {
                    match arguments.commands.get(name) {
                        Some(command) => {
                            if !command.help().is_empty() {
                                let aliases = command.aliases();
                                let aliases_line = if !aliases.is_empty() {
                                    let mut aliases_docs_buffer = aliases.join(", ");
                                    aliases_docs_buffer.insert_str(0, " (");
                                    aliases_docs_buffer.push_str(")");

                                    aliases_docs_buffer
                                } else {
                                    "".to_string()
                                };

                                buffer.push_str(&format!(
                                    "* [`{}`{}](#{})\n",
                                    name,
                                    aliases_line,
                                    name.replace(":", "_"),
                                ));
                            }
                        }
                        None => {
                            return CommandResult::Error(format!("Command: {} not found", name));
                        }
                    };
                } else {
                    if let Err(error) = writeln!(arguments.env.out, "Command: {} skipped.", &name) {
                        return CommandResult::Error(error.to_string());
                    }
                }
            }

            // create doc per command
            buffer.push_str("\n");
            for name in &names {
                if name.starts_with(&prefix) {
                    let command = match arguments.commands.get(name) {
                        Some(command) => command,
                        None => {
                            return CommandResult::Error(format!("Command: {} not found", name));
                        }
                    };

                    let help = command.help();

                    if !help.is_empty() {
                        let aliases = command.aliases();
                        let aliases_docs = if !aliases.is_empty() {
                            let mut aliases_docs_buffer = String::from("\n\n### Aliases:\n");

                            let mut first = true;
                            for alias in aliases {
                                if first {
                                    first = false;
                                } else {
                                    aliases_docs_buffer.push_str(", ");
                                }

                                aliases_docs_buffer.push_str(&alias);
                            }

                            aliases_docs_buffer
                        } else {
                            "".to_string()
                        };

                        buffer.push_str(&format!(
                            "\n<a name=\"{}\"></a>\n## `{}`\n{}{}\n",
                            name.replace(":", "_"),
                            name,
                            help,
                            aliases_docs
                        ));
                    }
                }
            }

            // footer
            buffer.push_str(include_str!("footer.md"));

            match io::write_text_file(&file, &buffer) {
                Ok(_) => CommandResult::Continue(Some(file)),
                Err(error) => CommandResult::Error(error.to_string()),
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
