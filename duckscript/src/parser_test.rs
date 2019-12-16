use super::*;
use crate::test;

#[test]
fn parse_pre_process_line_empty() {
    let chars = "".chars().collect();
    let output = parse_pre_process_line(&chars, InstructionMetaInfo::new(), 0);

    assert!(output.is_err());
    assert_eq!(output.err(), Some(ScriptError::PreProcessNoCommandFound));
}

#[test]
fn parse_pre_process_line_all_spaces() {
    let chars = "    ".chars().collect();
    let output = parse_pre_process_line(&chars, InstructionMetaInfo::new(), 0);

    assert!(output.is_err());
    assert_eq!(output.err(), Some(ScriptError::PreProcessNoCommandFound));
}

#[test]
fn parse_pre_process_line_just_command() {
    let chars = "test_command".chars().collect();
    let output = parse_pre_process_line(&chars, InstructionMetaInfo::new(), 0);

    assert!(output.is_ok());
    let instruction = test::get_pre_process_instruction(output.unwrap());
    assert_eq!(instruction.command.unwrap(), "test_command");
    assert!(instruction.arguments.is_none());
}

#[test]
fn parse_pre_process_line_just_command_with_spaces() {
    let chars = "   test_command   ".chars().collect();
    let output = parse_pre_process_line(&chars, InstructionMetaInfo::new(), 0);

    assert!(output.is_ok());
    let instruction = test::get_pre_process_instruction(output.unwrap());
    assert_eq!(instruction.command.unwrap(), "test_command");
    assert!(instruction.arguments.is_none());
}

#[test]
fn parse_pre_process_line_just_command_with_arguments() {
    let chars = r#"   test_command   arg1 arg2 "arg3.1 arg3.2""#.chars().collect();
    let output = parse_pre_process_line(&chars, InstructionMetaInfo::new(), 0);

    assert!(output.is_ok());
    let instruction = test::get_pre_process_instruction(output.unwrap());
    assert_eq!(instruction.command.unwrap(), "test_command");
    assert_eq!(
        instruction.arguments.unwrap(),
        vec!["arg1", "arg2", "arg3.1 arg3.2"]
    );
}

#[test]
fn parse_next_argument_empty() {
    let chars = "".chars().collect();
    let result = parse_next_argument(&chars, 0);

    assert!(result.is_ok());

    let (index, value) = result.unwrap();

    assert_eq!(index, 0);
    assert!(value.is_none());
}

#[test]
fn parse_next_argument_spaces_only() {
    let chars = "   ".chars().collect();
    let result = parse_next_argument(&chars, 0);

    assert!(result.is_ok());

    let (index, value) = result.unwrap();

    assert_eq!(index, 3);
    assert!(value.is_none());
}

#[test]
fn parse_next_argument_value_only() {
    let chars = "test".chars().collect();
    let result = parse_next_argument(&chars, 0);

    assert!(result.is_ok());

    let (index, value) = result.unwrap();

    assert_eq!(index, 4);
    assert_eq!(value.unwrap(), "test");
}

#[test]
fn parse_next_argument_value_after_spaces() {
    let chars = "  test".chars().collect();
    let result = parse_next_argument(&chars, 0);

    assert!(result.is_ok());

    let (index, value) = result.unwrap();

    assert_eq!(index, 6);
    assert_eq!(value.unwrap(), "test");
}

#[test]
fn parse_next_argument_value_in_middle_of_spaces() {
    let chars = "  test  ".chars().collect();
    let result = parse_next_argument(&chars, 0);

    assert!(result.is_ok());

    let (index, value) = result.unwrap();

    assert_eq!(index, 6);
    assert_eq!(value.unwrap(), "test");
}

#[test]
fn parse_next_argument_value_in_middle_of_spaces_start_in_middle_of_value() {
    let chars = "  test  ".chars().collect();
    let result = parse_next_argument(&chars, 3);

    assert!(result.is_ok());

    let (index, value) = result.unwrap();

    assert_eq!(index, 6);
    assert_eq!(value.unwrap(), "est");
}

#[test]
fn parse_next_argument_value_with_quots() {
    let chars = r#"  "test"  "#.chars().collect();
    let result = parse_next_argument(&chars, 0);

    assert!(result.is_ok());

    let (index, value) = result.unwrap();

    assert_eq!(index, 8);
    assert_eq!(value.unwrap(), "test");
}

#[test]
fn parse_next_argument_empty_with_quots() {
    let chars = r#"  ""  "#.chars().collect();
    let result = parse_next_argument(&chars, 0);

    assert!(result.is_ok());

    let (index, value) = result.unwrap();

    assert_eq!(index, 4);
    assert_eq!(value.unwrap(), "");
}

#[test]
fn parse_next_argument_value_with_control() {
    let chars = r#"  \"test\"\\  "#.chars().collect();
    let result = parse_next_argument(&chars, 0);

    assert!(result.is_ok());

    let (index, value) = result.unwrap();

    assert_eq!(index, 12);
    assert_eq!(value.unwrap(), "\"test\"\\");
}

#[test]
fn parse_next_argument_value_with_space_in_middle() {
    let chars = r#"  "test \"test\""  "#.chars().collect();
    let result = parse_next_argument(&chars, 0);

    assert!(result.is_ok());

    let (index, value) = result.unwrap();

    assert_eq!(index, 17);
    assert_eq!(value.unwrap(), "test \"test\"");
}

#[test]
fn parse_next_argument_value_with_comment_in_middle() {
    let chars = r#"  "test \"test\" #test test"  "#.chars().collect();
    let result = parse_next_argument(&chars, 0);

    assert!(result.is_ok());

    let (index, value) = result.unwrap();

    assert_eq!(index, 28);
    assert_eq!(value.unwrap(), "test \"test\" #test test");
}

#[test]
fn parse_next_argument_value_with_comment_afterwards() {
    let chars = r#"  test#comment"#.chars().collect();
    let result = parse_next_argument(&chars, 0);

    assert!(result.is_ok());

    let (index, value) = result.unwrap();

    assert_eq!(index, 14);
    assert_eq!(value.unwrap(), "test");
}

#[test]
fn parse_next_argument_value_with_comment_afterwards_wrapped_with_quotes() {
    let chars = r#"  "test#comment""#.chars().collect();
    let result = parse_next_argument(&chars, 0);

    assert!(result.is_ok());

    let (index, value) = result.unwrap();

    assert_eq!(index, 16);
    assert_eq!(value.unwrap(), "test#comment");
}

#[test]
fn parse_next_argument_value_with_control_error() {
    let chars = r#"  \a  "#.chars().collect();
    let result = parse_next_argument(&chars, 0);

    assert!(result.is_err());
}

#[test]
fn parse_next_argument_value_with_control_end_error() {
    let chars = r#"  \"#.chars().collect();
    let result = parse_next_argument(&chars, 0);

    assert!(result.is_err());
}

#[test]
fn parse_next_argument_value_with_quote_end_error() {
    let chars = r#"  ""#.chars().collect();
    let result = parse_next_argument(&chars, 0);

    assert!(result.is_err());
}

#[test]
fn parse_arguments_empty() {
    let chars = "".chars().collect();
    let result = parse_arguments(&chars, 0);

    assert!(result.is_ok());

    let arguments = result.unwrap();

    assert!(arguments.is_none());
}

#[test]
fn parse_arguments_only_spaces() {
    let chars = "        ".chars().collect();
    let result = parse_arguments(&chars, 0);

    assert!(result.is_ok());

    let arguments = result.unwrap();

    assert!(arguments.is_none());
}

#[test]
fn parse_arguments_only_value() {
    let chars = "test".chars().collect();
    let result = parse_arguments(&chars, 0);

    assert!(result.is_ok());

    let arguments = result.unwrap();

    assert_eq!(arguments.unwrap(), vec!["test"]);
}

#[test]
fn parse_arguments_value_in_middle_of_spaces() {
    let chars = "  test  ".chars().collect();
    let result = parse_arguments(&chars, 0);

    assert!(result.is_ok());

    let arguments = result.unwrap();

    assert_eq!(arguments.unwrap(), vec!["test"]);
}

#[test]
fn parse_arguments_multiple_values() {
    let chars = r#"  test  "test \"test\"" ${value} %{value}"#.chars().collect();
    let result = parse_arguments(&chars, 0);

    assert!(result.is_ok());

    let arguments = result.unwrap();

    assert_eq!(
        arguments.unwrap(),
        vec!["test", "test \"test\"", "${value}", "%{value}"]
    );
}

#[test]
fn find_label_empty() {
    let chars = "".chars().collect();
    let result = find_label(&chars, 0);

    assert!(result.is_ok());

    let (index, value) = result.unwrap();

    assert_eq!(index, chars.len());
    assert!(value.is_none());
}

#[test]
fn find_label_only_spaces() {
    let chars = "        ".chars().collect();
    let result = find_label(&chars, 0);

    assert!(result.is_ok());

    let (index, value) = result.unwrap();

    assert_eq!(index, chars.len());
    assert!(value.is_none());
}

#[test]
fn find_label_label_between_spaces() {
    let chars = "   :label     ".chars().collect();
    let result = find_label(&chars, 0);

    assert!(result.is_ok());

    let (index, value) = result.unwrap();

    assert_eq!(index, 9);
    assert_eq!(value.unwrap(), ":label");
}
