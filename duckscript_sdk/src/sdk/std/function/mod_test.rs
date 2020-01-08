use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    let commands = create("");
    for command in commands {
        test::test_common_command_functions(command);
    }
}

#[test]
fn run_no_function_end() {
    test::run_script_and_crash(
        create(""),
        r#"
    function test

    # no ending
    "#,
    );
}

#[test]
fn run_function_inside_function() {
    test::run_script_and_crash(
        create(""),
        r#"
    function test1

    function test2

    end_function

    end_function
    "#,
    );
}

#[test]
fn run_function_only_end_function() {
    test::run_script_and_validate(
        create(""),
        r#"
    end_function

    end_function
    "#,
        CommandValidation::None,
    );
}

#[test]
fn run_function_only_return() {
    test::run_script_and_validate(
        create(""),
        r#"
    return

    return
    "#,
        CommandValidation::None,
    );
}

#[test]
fn run_function_reached_end() {
    test::run_script_and_validate(
        create(""),
        r#"
    function test_fn

    end_function

    out = test_fn
    "#,
        CommandValidation::None,
    );
}

#[test]
fn run_function_return_output() {
    test::run_script_and_validate(
        create(""),
        r#"
    function test_fn
    return test
    end_function

    out = test_fn
    "#,
        CommandValidation::Match("out".to_string(), "test".to_string()),
    );
}

#[test]
fn run_function_return_none() {
    test::run_script_and_validate(
        create(""),
        r#"
    function test_fn
    return
    end_function

    out = test_fn
    "#,
        CommandValidation::None,
    );
}

#[test]
fn run_function_call_multiple_functions_and_return_output() {
    test::run_script_and_validate(
        create(""),
        r#"
    function test_fn2
    return fn2
    end_function

    function test_fn
    out1 = test_fn1
    out2 = test_fn2
    return "test ${out1} ${out2}"
    end_function

    function test_fn1
    return fn1
    end_function

    out = test_fn
    "#,
        CommandValidation::Match("out".to_string(), "test fn1 fn2".to_string()),
    );
}

#[test]
fn run_function_call_multiple_functions_pass_arguments_and_return_output() {
    test::run_script_and_validate(
        create(""),
        r#"
    function test_fn2
    return "fn2 ${1} ${2} ${3}"
    end_function

    function test_fn
    out1 = test_fn1 a b c
    out2 = test_fn2 d e f
    return "test ${out1} ${out2} ${4}"
    end_function

    function test_fn1
    return "fn1 ${1} ${2} ${3}"
    end_function

    out = test_fn 1 2 3 4
    "#,
        CommandValidation::Match("out".to_string(), "test fn1 a b c fn2 d e f 4".to_string()),
    );
}

#[test]
fn run_function_return_outside_function() {
    test::run_script_and_validate(
        create(""),
        r#"
    function test_fn
    return test
    end_function
    
    return 5

    out = test_fn
    "#,
        CommandValidation::Match("out".to_string(), "test".to_string()),
    );
}
