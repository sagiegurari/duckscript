use std::process::{Child, Command, Stdio};

#[cfg(test)]
#[path = "./exec_test.rs"]
mod exec_test;

pub(crate) fn exec(
    arguments: &Vec<String>,
    print_output: bool,
    allow_input: bool,
    start_index: usize,
) -> Result<(String, String, i32), String> {
    let mut command = create_command(arguments, print_output, allow_input, start_index)?;

    match command.output() {
        Ok(ref output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
            let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
            let exit_code = match output.status.code() {
                Some(value) => value,
                None => {
                    return Err(format!(
                        "Unable to extract exit code for command: {}",
                        &arguments[0]
                    )
                    .to_string());
                }
            };

            Ok((stdout, stderr, exit_code))
        }
        Err(error) => Err(error.to_string()),
    }
}

pub(crate) fn spawn(
    arguments: &Vec<String>,
    print_output: bool,
    allow_input: bool,
    start_index: usize,
) -> Result<Child, String> {
    let mut command = create_command(arguments, print_output, allow_input, start_index)?;

    match command.spawn() {
        Ok(child) => Ok(child),
        Err(error) => Err(error.to_string()),
    }
}

fn create_command(
    arguments: &Vec<String>,
    print_output: bool,
    allow_input: bool,
    start_index: usize,
) -> Result<Command, String> {
    if arguments.len() <= start_index {
        Err("Command not provided.".to_string())
    } else {
        let mut command = Command::new(&arguments[start_index]);
        let argument_index = start_index + 1;

        for argument in &arguments[argument_index..] {
            command.arg(argument);
        }

        if allow_input {
            command.stdin(Stdio::inherit());
        } else {
            command.stdin(Stdio::null());
        }

        if print_output {
            command.stdout(Stdio::inherit()).stderr(Stdio::inherit());
        }

        Ok(command)
    }
}
