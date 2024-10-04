use crate::utils::pckg;
use duckscript::types::command::{Command, CommandInvocationContext, CommandResult};
use fsio::path::as_path::AsPath;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

cfg_if::cfg_if! {
    if #[cfg(any(target_os = "redox", unix, target_os = "vxworks"))] {
        use std::fs;
        use std::os::unix::fs::PermissionsExt;

        fn set_mode_unix(path: &str, mode: u32) -> std::io::Result<()> {
            let metadata = fs::metadata(path)?;
            let mut permissions = metadata.permissions();
            permissions.set_mode(mode);
            fs::set_permissions(path, permissions)
        }

        fn set_mode(path: &str, mode: &str) -> CommandResult {
            match u32::from_str_radix(mode, 8) {
                Ok(mode_u32) => {
                    match set_mode_unix(path, mode_u32) {
                        Ok(_) => CommandResult::Continue(Some(mode_u32.to_string())),
                        Err(error) => CommandResult::Error(error.to_string()),
                    }
                }
                Err(error) => CommandResult::Error(error.to_string()),
            }
        }
    } else {
        fn set_mode(_path: &str, _mode: &str) -> CommandResult {
            CommandResult::Error("Setting file mode is not supported on this platform.".to_string())
        }
    }
}

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "SetMode")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["chmod".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, context: CommandInvocationContext) -> CommandResult {
        if context.arguments.len() < 2 {
            CommandResult::Error("Path/Mode not provided.".to_string())
        } else {
            let path = &context.arguments[1].as_path();
            if path.exists() {
                set_mode(&context.arguments[1], &context.arguments[0])
            } else {
                CommandResult::Error(
                    format!("Path: {} not found.", &context.arguments[1]).to_string(),
                )
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
