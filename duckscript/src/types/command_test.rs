use super::*;
use crate::test::{TestCommand1, TestCommand2, TestCommand3, TestCommand4};

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

    assert!(commands.get("test1").is_none());
    assert!(commands.get("test2").is_none());

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
fn commands_set_get_not_found() {
    let mut commands = Commands::new();

    commands.set(Box::new(TestCommand1 {})).unwrap();
    commands.set(Box::new(TestCommand2 {})).unwrap();

    assert!(commands.get("test3").is_none());
}
