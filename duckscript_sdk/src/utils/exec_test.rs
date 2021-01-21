use super::*;

fn create_command() -> Vec<String> {
    if cfg!(windows) {
        ["badcommand", "cmd", "/c", "echo", "hello", "world"].as_ref()
    } else {
        ["badcommand", "echo", "hello", "world"].as_ref()
    }
    .iter()
    .map(ToString::to_string)
    .collect()
}

#[test]
fn exec_valid() {
    let (stdout, stderr, code) = exec(
        &create_command(),
        false,
        false,
        1,
    )
    .unwrap();

    assert_eq!(code, 0);
    assert_eq!(stdout.trim(), "hello world");
    assert!(stderr.trim().is_empty());
}

#[test]
fn exec_error() {
    let result = exec(
        &create_command(),
        false,
        false,
        0,
    );

    assert!(result.is_err());
}
