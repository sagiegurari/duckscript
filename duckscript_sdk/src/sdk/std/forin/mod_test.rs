use super::*;
use crate::test;
use crate::test::{ArrayCommand, CommandValidation, SetCommand};

#[test]
fn common_functions() {
    let commands = create("");
    for command in commands {
        test::test_common_command_functions(command);
    }
}

#[test]
fn run_forin_no_end() {
    let array_command = ArrayCommand {};
    let mut commands = create("");
    commands.push(Box::new(array_command));

    test::run_script_and_crash(
        commands,
        r#"
    args = test_array a b c
    for arg in args

    # no ending
    "#,
    );
}

#[test]
fn run_forin_no_in() {
    let array_command = ArrayCommand {};
    let mut commands = create("");
    commands.push(Box::new(array_command));

    test::run_script_and_error(
        commands,
        r#"
    args = test_array a b c
    out = for arg args

    end_for
    "#,
        "out",
    );
}

#[test]
fn run_forin_no_args() {
    test::run_script_and_error(
        create(""),
        r#"
    out = for

    end_for
    "#,
        "out",
    );
}

#[test]
fn run_forin_too_many_args() {
    let array_command = ArrayCommand {};
    let mut commands = create("");
    commands.push(Box::new(array_command));

    test::run_script_and_error(
        commands,
        r#"
    args = test_array a b c
    out = for arg in args test

    end_for
    "#,
        "out",
    );
}

#[test]
fn run_forin_valid() {
    let mut commands = create("");
    commands.push(Box::new(ArrayCommand {}));
    commands.push(Box::new(SetCommand {}));

    test::run_script_and_validate(
        commands,
        r#"
    args = test_array a b c
    for arg in args
        out = test_set "${out} ${arg}"
    end_for
    "#,
        CommandValidation::Match("out".to_string(), " a b c".to_string()),
    );
}

#[test]
fn run_forin_nested() {
    let mut commands = create("");
    commands.push(Box::new(ArrayCommand {}));
    commands.push(Box::new(SetCommand {}));

    test::run_script_and_validate(
        commands,
        r#"
    range = test_array 1 2 3
    for i in range
        for j in range
            out = test_set "${out} ${i}${j}"
        end_for
    end_for
    "#,
        CommandValidation::Match("out".to_string(), " 11 12 13 21 22 23 31 32 33".to_string()),
    );
}
