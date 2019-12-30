use super::*;
use crate::test;
use crate::test::{TestCommand1, TestCommand2, TestCommand3, TestCommand4};
use crate::types::runtime::Context;
use std::collections::HashMap;

#[test]
fn command_default_aliases() {
    struct InnerCommand {}

    impl Command for InnerCommand {
        fn name(&self) -> String {
            "".to_string()
        }
    }

    let command = InnerCommand {};
    let help = command.help();

    assert!(!help.is_empty());
}

#[test]
fn command_default_help() {
    struct InnerCommand {}

    impl Command for InnerCommand {
        fn name(&self) -> String {
            "".to_string()
        }
    }

    let command = InnerCommand {};
    let help = command.help();

    assert!(!help.is_empty());
}

#[test]
fn command_default_run() {
    struct InnerCommand {}

    impl Command for InnerCommand {
        fn name(&self) -> String {
            "".to_string()
        }
    }

    let command = InnerCommand {};
    let result = command.run(vec![]);

    test::validate_continue_result(&result, None);
}

#[test]
fn command_default_run_with_context() {
    struct InnerCommand {}

    impl Command for InnerCommand {
        fn name(&self) -> String {
            "".to_string()
        }
    }

    let mut context = Context::new();
    let command = InnerCommand {};
    let result = command.run_with_context(
        vec![],
        &mut HashMap::new(),
        &mut HashMap::new(),
        None,
        &vec![],
        &mut context.commands,
        0,
    );

    test::validate_continue_result(&result, None);
}

#[test]
fn commands_new() {
    let commands = Commands::new();

    assert!(commands.commands.is_empty());
    assert!(commands.aliases.is_empty());
}

#[test]
#[should_panic]
fn commands_set_name_exists() {
    let mut commands = Commands::new();

    commands.set(Box::new(TestCommand1 {})).unwrap();
    commands.set(Box::new(TestCommand3 {})).unwrap();
}

#[test]
#[should_panic]
fn commands_set_alias_exists() {
    let mut commands = Commands::new();

    commands.set(Box::new(TestCommand1 {})).unwrap();
    commands.set(Box::new(TestCommand4 {})).unwrap();
}

#[test]
fn commands_set_alias_valid() {
    let mut commands = Commands::new();

    commands.set(Box::new(TestCommand1 {})).unwrap();
    commands.set(Box::new(TestCommand2 {})).unwrap();
}

#[test]
fn commands_set_get_exists() {
    let mut commands = Commands::new();

    assert!(commands.get_for_use("test1").is_none());
    assert!(commands.get_for_use("test2").is_none());

    commands.set(Box::new(TestCommand1 {})).unwrap();
    commands.set(Box::new(TestCommand2 {})).unwrap();

    assert_eq!(commands.get("test1").unwrap().name(), "test1");
    assert_eq!(commands.get("test2").unwrap().name(), "test2");

    assert_eq!(commands.get("test11").unwrap().name(), "test1");
    assert_eq!(commands.get("test12").unwrap().name(), "test1");
    assert_eq!(commands.get("test21").unwrap().name(), "test2");
    assert_eq!(commands.get("test22").unwrap().name(), "test2");
}

#[test]
fn commands_set_get_return_exists() {
    let mut commands = Commands::new();

    assert!(commands.get_for_use("test1").is_none());
    assert!(commands.get_for_use("test2").is_none());

    commands.set(Box::new(TestCommand1 {})).unwrap();
    commands.set(Box::new(TestCommand2 {})).unwrap();

    let mut command = commands.get_for_use("test1").unwrap();
    assert_eq!(command.name(), "test1");
    commands.return_after_usage(command);
    command = commands.get_for_use("test2").unwrap();
    assert_eq!(command.name(), "test2");
    commands.return_after_usage(command);

    command = commands.get_for_use("test11").unwrap();
    assert_eq!(command.name(), "test1");
    commands.return_after_usage(command);
    command = commands.get_for_use("test12").unwrap();
    assert_eq!(command.name(), "test1");
    commands.return_after_usage(command);
    command = commands.get_for_use("test21").unwrap();
    assert_eq!(command.name(), "test2");
    commands.return_after_usage(command);
    command = commands.get_for_use("test22").unwrap();
    assert_eq!(command.name(), "test2");
    commands.return_after_usage(command);
}

#[test]
fn commands_set_get_not_found() {
    let mut commands = Commands::new();

    commands.set(Box::new(TestCommand1 {})).unwrap();
    commands.set(Box::new(TestCommand2 {})).unwrap();

    assert!(commands.get_for_use("test3").is_none());
}

#[test]
fn commands_set_get_all_command_names_empty() {
    let commands = Commands::new();

    let names = commands.get_all_command_names();

    assert!(names.is_empty());
}

#[test]
fn commands_set_get_all_command_names_exists() {
    let mut commands = Commands::new();

    commands.set(Box::new(TestCommand1 {})).unwrap();
    commands.set(Box::new(TestCommand2 {})).unwrap();

    let names = commands.get_all_command_names();

    assert_eq!(names, vec!["test1", "test2"]);
}

#[test]
fn commands_remove() {
    let command = TestCommand1 {};
    let mut commands = Commands::new();

    commands.set(Box::new(command)).unwrap();

    assert_eq!(commands.get("test11").unwrap().name(), "test1");
    commands.remove("test11");
    assert!(commands.get("test11").is_none());
}
