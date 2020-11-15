use super::*;
use crate::test;
use crate::test::{CommandValidation, SetCommand};

#[test]
fn common_functions() {
    let commands = create("");
    for command in commands {
        test::test_common_command_functions(command);
    }
}

#[test]
fn run_while_no_end() {
    let set_command = SetCommand {};
    let mut commands = create("");
    commands.push(Box::new(set_command));

    test::run_script_and_crash(
        commands,
        r#"
    while test_set true

    # no ending
    "#,
    );
}

#[test]
fn run_while_no_condition() {
    test::run_script_and_error(
        create(""),
        r#"
    out = while

    end_while
    "#,
        "out",
    );
}

#[test]
fn run_sub_while_no_end() {
    let set_command = SetCommand {};
    let mut commands = create("");
    commands.push(Box::new(set_command));

    test::run_script_and_crash(
        commands,
        r#"
    while test_set true

    while test_set true
    end_while
    # no ending
    "#,
    );
}

#[test]
fn run_while_true() {
    let set_command = SetCommand {};
    let mut commands = create("");
    commands.push(Box::new(set_command));

    test::run_script_and_validate(
        commands,
        r#"
    test = test_set true
    while ${test}
    out = test_set while
    test = test_set false
    end_while
    "#,
        CommandValidation::Match("out".to_string(), "while".to_string()),
    );
}

#[test]
fn run_while_false() {
    let set_command = SetCommand {};
    let mut commands = create("");
    commands.push(Box::new(set_command));

    test::run_script_and_validate(
        commands,
        r#"
    out = test_set test_false
    while test_set false
    badcommand
    end_while
    "#,
        CommandValidation::Match("out".to_string(), "test_false".to_string()),
    );
}

#[test]
fn run_nested_while() {
    let set_command = SetCommand {};
    let mut commands = create("");
    commands.push(Box::new(set_command));

    test::run_script_and_validate(
        commands,
        r#"
    top = test_set true
    while ${top}
        while test_set false
            badcommand
        end_while

        top = test_set false

        inner = test_set true
        while ${inner}
            inner = test_set false
            out = test_set win
        end_while
    end_while
    "#,
        CommandValidation::Match("out".to_string(), "win".to_string()),
    );
}
