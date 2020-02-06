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
        &vec![
            "badcommand".to_string(),
            "echo".to_string(),
            "hello".to_string(),
            "world".to_string(),
        ],
        false,
        false,
        0,
    );

    assert!(result.is_err());
}
