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
fn run_if_no_end() {
    let set_command = SetCommand {};
    let mut commands = create("");
    commands.push(Box::new(set_command));

    test::run_script_and_fail(
        commands,
        r#"
    if test_set true

    # no ending
    "#,
    );
}

#[test]
fn run_if_else_no_end() {
    let set_command = SetCommand {};
    let mut commands = create("");
    commands.push(Box::new(set_command));

    test::run_script_and_fail(
        commands,
        r#"
    if test_set true

    else

    # no ending
    "#,
    );
}

#[test]
fn run_if_else_if_else_no_end() {
    let set_command = SetCommand {};
    let mut commands = create("");
    commands.push(Box::new(set_command));

    test::run_script_and_fail(
        commands,
        r#"
    if test_set true

    ifelse test_set true

    else

    # no ending
    "#,
    );
}

#[test]
fn run_if_no_condition() {
    test::run_script_and_fail(
        create(""),
        r#"
    if

    end_if
    "#,
    );
}

#[test]
fn run_if_else_if_no_condition() {
    let set_command = SetCommand {};
    let mut commands = create("");
    commands.push(Box::new(set_command));

    test::run_script_and_fail(
        commands,
        r#"
    if test_set false

    elseif

    end_if
    "#,
    );
}

#[test]
fn run_sub_if_no_end() {
    let set_command = SetCommand {};
    let mut commands = create("");
    commands.push(Box::new(set_command));

    test::run_script_and_fail(
        commands,
        r#"
    if test_set true

    if test_set true
    end_if
    # no ending
    "#,
    );
}

#[test]
fn run_sub_else_if_no_end() {
    let set_command = SetCommand {};
    let mut commands = create("");
    commands.push(Box::new(set_command));

    test::run_script_and_fail(
        commands,
        r#"
    if test_set true

    elseif test_set true

    if test_set true
    end_if
    # no ending
    "#,
    );
}

#[test]
fn run_sub_else_no_end() {
    let set_command = SetCommand {};
    let mut commands = create("");
    commands.push(Box::new(set_command));

    test::run_script_and_fail(
        commands,
        r#"
    if test_set true

    elseif test_set true

    else

    if test_set true
    end_if
    # no ending
    "#,
    );
}

#[test]
fn run_if_true() {
    let set_command = SetCommand {};
    let mut commands = create("");
    commands.push(Box::new(set_command));

    test::run_script_and_validate(
        commands,
        r#"
    if test_set true
    out = test_set if
    else
    end_if
    "#,
        CommandValidation::Match("out".to_string(), "if".to_string()),
    );
}

#[test]
fn run_if_false() {
    let set_command = SetCommand {};
    let mut commands = create("");
    commands.push(Box::new(set_command));

    test::run_script_and_validate(
        commands,
        r#"
    if test_set false
    badcommand
    else
    out = test_set else
    end_if
    "#,
        CommandValidation::Match("out".to_string(), "else".to_string()),
    );
}

#[test]
fn run_if_false_upper_case() {
    let set_command = SetCommand {};
    let mut commands = create("");
    commands.push(Box::new(set_command));

    test::run_script_and_validate(
        commands,
        r#"
    if test_set FALSE
    badcommand
    else
    out = test_set else
    end_if
    "#,
        CommandValidation::Match("out".to_string(), "else".to_string()),
    );
}

#[test]
fn run_if_0() {
    let set_command = SetCommand {};
    let mut commands = create("");
    commands.push(Box::new(set_command));

    test::run_script_and_validate(
        commands,
        r#"
    if test_set 0
    badcommand
    else
    out = test_set else
    end_if
    "#,
        CommandValidation::Match("out".to_string(), "else".to_string()),
    );
}

#[test]
fn run_if_no() {
    let set_command = SetCommand {};
    let mut commands = create("");
    commands.push(Box::new(set_command));

    test::run_script_and_validate(
        commands,
        r#"
    if test_set no
    badcommand
    else
    out = test_set else
    end_if
    "#,
        CommandValidation::Match("out".to_string(), "else".to_string()),
    );
}

#[test]
fn run_if_no_upper_case() {
    let set_command = SetCommand {};
    let mut commands = create("");
    commands.push(Box::new(set_command));

    test::run_script_and_validate(
        commands,
        r#"
    if test_set NO
    badcommand
    else
    out = test_set else
    end_if
    "#,
        CommandValidation::Match("out".to_string(), "else".to_string()),
    );
}

#[test]
fn run_if_none() {
    let set_command = SetCommand {};
    let mut commands = create("");
    commands.push(Box::new(set_command));

    test::run_script_and_validate(
        commands,
        r#"
    if test_set
    badcommand
    else
    out = test_set else
    end_if
    "#,
        CommandValidation::Match("out".to_string(), "else".to_string()),
    );
}

#[test]
fn run_if_elseif_false() {
    let set_command = SetCommand {};
    let mut commands = create("");
    commands.push(Box::new(set_command));

    test::run_script_and_validate(
        commands,
        r#"
    if test_set false
    badcommand
    elseif test_set false
    badcommand
    else
    out = test_set else
    end_if
    "#,
        CommandValidation::Match("out".to_string(), "else".to_string()),
    );
}

#[test]
fn run_if_elseif_true() {
    let set_command = SetCommand {};
    let mut commands = create("");
    commands.push(Box::new(set_command));

    test::run_script_and_validate(
        commands,
        r#"
    if test_set false
    badcommand
    elseif test_set true
    out = test_set elseif
    else
    badcommand
    end_if
    "#,
        CommandValidation::Match("out".to_string(), "elseif".to_string()),
    );
}

#[test]
fn run_nested_if() {
    let set_command = SetCommand {};
    let mut commands = create("");
    commands.push(Box::new(set_command));

    test::run_script_and_validate(
        commands,
        r#"
    if test_set true
        if test_set false
            badcommand
        elseif test_set true
            if test_set false
                badcommand
            elseif test_set false
                badcommand
            else
                out = test_set win
            end_if
        else
            badcommand
        end_if
    else
        badcommand
    end_if
    "#,
        CommandValidation::Match("out".to_string(), "win".to_string()),
    );
}
