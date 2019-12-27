use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_arguments() {
    test::run_script_and_fail(vec![create("")], "goto");
}

#[test]
fn run_mutliple_arguments() {
    test::run_script_and_fail(vec![create("")], "goto :label1 :label2");
}

#[test]
fn run_not_label_format() {
    test::run_script_and_fail(vec![create("")], "goto label");
}

#[test]
fn run_label_not_found() {
    test::run_script_and_fail(vec![create("")], "goto :unknown");
}

#[test]
fn run_no_args() {
    struct SetCommand {}

    impl Command for SetCommand {
        fn name(&self) -> String {
            "set".to_string()
        }

        fn run(&self, arguments: Vec<String>) -> CommandResult {
            CommandResult::Continue(Some(arguments[0].clone()))
        }
    }
    let set_command = SetCommand {};

    let context = test::run_script_and_validate(
        vec![create(""), Box::new(set_command)],
        r#"
goto :valid

out1 = set 1

:valid out2 = set 2
        "#,
        CommandValidation::Match("out2".to_string(), "2".to_string()),
    );

    assert_eq!(context.variables.len(), 1);
}
