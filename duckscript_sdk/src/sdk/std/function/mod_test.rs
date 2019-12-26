use super::*;
use crate::test;

#[test]
fn common_functions() {
    let commands = create("");
    for command in commands {
        test::test_common_command_functions(command);
    }
}

#[test]
fn run_no_function_end() {
    test::run_script_and_fail(
        create(""),
        r#"
    function test

    # no ending
    "#,
    );
}

#[test]
fn run_function_inside_function() {
    test::run_script_and_fail(
        create(""),
        r#"
    function test1

    function test2

    end_function
    
    end_function
    "#,
    );
}
