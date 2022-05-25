use super::*;

#[test]
fn exec_valid() {
    let (stdout, stderr, code) = exec(
        &vec![
            "badcommand".to_string(),
            "echo".to_string(),
            "hello".to_string(),
            "world".to_string(),
        ],
        false,
        ExecInput::None,
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
        &vec![
            "badcommand".to_string(),
            "echo".to_string(),
            "hello".to_string(),
            "world".to_string(),
        ],
        false,
        ExecInput::None,
        0,
    );

    assert!(result.is_err());
}

#[test]
#[cfg(target_os = "linux")]
fn exec_with_input() {
    let (stdout, stderr, code) = exec(
        &vec!["cat".to_string()],
        false,
        ExecInput::Text("1 2 3".to_string()),
        0,
    )
    .unwrap();

    assert_eq!(code, 0);
    assert_eq!(stdout.trim(), "1 2 3");
    assert!(stderr.trim().is_empty());
}
