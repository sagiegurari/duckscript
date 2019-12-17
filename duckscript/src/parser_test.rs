use super::*;
use crate::test;

#[test]
fn parse_pre_process_line_empty() {
    let chars = "".chars().collect();
    let output = parse_pre_process_line(&chars, InstructionMetaInfo::new(), 0);

    assert!(output.is_err());
}

#[test]
fn parse_pre_process_line_all_spaces() {
    let chars = "    ".chars().collect();
    let output = parse_pre_process_line(&chars, InstructionMetaInfo::new(), 0);

    assert!(output.is_err());
}

#[test]
fn parse_pre_process_line_just_command() {
    let chars = "test_command".chars().collect();
    let output = parse_pre_process_line(&chars, InstructionMetaInfo::new(), 0);

    assert!(output.is_ok());
    let instruction = test::get_pre_process_instruction(&output.unwrap());
    assert_eq!(instruction.command.unwrap(), "test_command");
    assert!(instruction.arguments.is_none());
}

#[test]
fn parse_pre_process_line_just_command_with_spaces() {
    let chars = "   test_command   ".chars().collect();
    let output = parse_pre_process_line(&chars, InstructionMetaInfo::new(), 0);

    assert!(output.is_ok());
    let instruction = test::get_pre_process_instruction(&output.unwrap());
    assert_eq!(instruction.command.unwrap(), "test_command");
    assert!(instruction.arguments.is_none());
}

#[test]
fn parse_pre_process_line_just_command_with_arguments() {
    let chars = r#"   test_command   arg1 arg2 "arg3.1 arg3.2""#.chars().collect();
    let output = parse_pre_process_line(&chars, InstructionMetaInfo::new(), 0);

    assert!(output.is_ok());
    let instruction = test::get_pre_process_instruction(&output.unwrap());
    assert_eq!(instruction.command.unwrap(), "test_command");
    assert_eq!(
        instruction.arguments.unwrap(),
        vec!["arg1", "arg2", "arg3.1 arg3.2"]
    );
}

#[test]
fn parse_next_argument_empty() {
    let chars = "".chars().collect();
    let result = parse_next_argument(&InstructionMetaInfo::new(), &chars, 0);

    assert!(result.is_ok());

    let (index, value) = result.unwrap();

    assert_eq!(index, 0);
    assert!(value.is_none());
}

#[test]
fn parse_next_argument_spaces_only() {
    let chars = "   ".chars().collect();
    let result = parse_next_argument(&InstructionMetaInfo::new(), &chars, 0);

    assert!(result.is_ok());

    let (index, value) = result.unwrap();

    assert_eq!(index, 3);
    assert!(value.is_none());
}

#[test]
fn parse_next_argument_value_only() {
    let chars = "test".chars().collect();
    let result = parse_next_argument(&InstructionMetaInfo::new(), &chars, 0);

    assert!(result.is_ok());

    let (index, value) = result.unwrap();

    assert_eq!(index, 4);
    assert_eq!(value.unwrap(), "test");
}

#[test]
fn parse_next_argument_value_after_spaces() {
    let chars = "  test".chars().collect();
    let result = parse_next_argument(&InstructionMetaInfo::new(), &chars, 0);

    assert!(result.is_ok());

    let (index, value) = result.unwrap();

    assert_eq!(index, 6);
    assert_eq!(value.unwrap(), "test");
}

#[test]
fn parse_next_argument_value_in_middle_of_spaces() {
    let chars = "  test  ".chars().collect();
    let result = parse_next_argument(&InstructionMetaInfo::new(), &chars, 0);

    assert!(result.is_ok());

    let (index, value) = result.unwrap();

    assert_eq!(index, 6);
    assert_eq!(value.unwrap(), "test");
}

#[test]
fn parse_next_argument_value_in_middle_of_spaces_start_in_middle_of_value() {
    let chars = "  test  ".chars().collect();
    let result = parse_next_argument(&InstructionMetaInfo::new(), &chars, 3);

    assert!(result.is_ok());

    let (index, value) = result.unwrap();

    assert_eq!(index, 6);
    assert_eq!(value.unwrap(), "est");
}

#[test]
fn parse_next_argument_value_with_quots() {
    let chars = r#"  "test"  "#.chars().collect();
    let result = parse_next_argument(&InstructionMetaInfo::new(), &chars, 0);

    assert!(result.is_ok());

    let (index, value) = result.unwrap();

    assert_eq!(index, 8);
    assert_eq!(value.unwrap(), "test");
}

#[test]
fn parse_next_argument_empty_with_quots() {
    let chars = r#"  ""  "#.chars().collect();
    let result = parse_next_argument(&InstructionMetaInfo::new(), &chars, 0);

    assert!(result.is_ok());

    let (index, value) = result.unwrap();

    assert_eq!(index, 4);
    assert_eq!(value.unwrap(), "");
}

#[test]
fn parse_next_argument_value_with_control() {
    let chars = r#"  \"test\"\\  "#.chars().collect();
    let result = parse_next_argument(&InstructionMetaInfo::new(), &chars, 0);

    assert!(result.is_ok());

    let (index, value) = result.unwrap();

    assert_eq!(index, 12);
    assert_eq!(value.unwrap(), "\"test\"\\");
}

#[test]
fn parse_next_argument_value_with_space_in_middle() {
    let chars = r#"  "test \"test\""  "#.chars().collect();
    let result = parse_next_argument(&InstructionMetaInfo::new(), &chars, 0);

    assert!(result.is_ok());

    let (index, value) = result.unwrap();

    assert_eq!(index, 17);
    assert_eq!(value.unwrap(), "test \"test\"");
}

#[test]
fn parse_next_argument_value_with_comment_in_middle() {
    let chars = r#"  "test \"test\" #test test"  "#.chars().collect();
    let result = parse_next_argument(&InstructionMetaInfo::new(), &chars, 0);

    assert!(result.is_ok());

    let (index, value) = result.unwrap();

    assert_eq!(index, 28);
    assert_eq!(value.unwrap(), "test \"test\" #test test");
}

#[test]
fn parse_next_argument_value_with_comment_afterwards() {
    let chars = r#"  test#comment"#.chars().collect();
    let result = parse_next_argument(&InstructionMetaInfo::new(), &chars, 0);

    assert!(result.is_ok());

    let (index, value) = result.unwrap();

    assert_eq!(index, 14);
    assert_eq!(value.unwrap(), "test");
}

#[test]
fn parse_next_argument_value_with_comment_afterwards_wrapped_with_quotes() {
    let chars = r#"  "test#comment""#.chars().collect();
    let result = parse_next_argument(&InstructionMetaInfo::new(), &chars, 0);

    assert!(result.is_ok());

    let (index, value) = result.unwrap();

    assert_eq!(index, 16);
    assert_eq!(value.unwrap(), "test#comment");
}

#[test]
fn parse_next_argument_value_with_control_error() {
    let chars = r#"  \a  "#.chars().collect();
    let result = parse_next_argument(&InstructionMetaInfo::new(), &chars, 0);

    assert!(result.is_err());
}

#[test]
fn parse_next_argument_value_with_control_end_error() {
    let chars = r#"  \"#.chars().collect();
    let result = parse_next_argument(&InstructionMetaInfo::new(), &chars, 0);

    assert!(result.is_err());
}

#[test]
fn parse_next_argument_value_with_quote_end_error() {
    let chars = r#"  ""#.chars().collect();
    let result = parse_next_argument(&InstructionMetaInfo::new(), &chars, 0);

    assert!(result.is_err());
}

#[test]
fn parse_arguments_empty() {
    let chars = "".chars().collect();
    let result = parse_arguments(&InstructionMetaInfo::new(), &chars, 0);

    assert!(result.is_ok());

    let arguments = result.unwrap();

    assert!(arguments.is_none());
}

#[test]
fn parse_arguments_only_spaces() {
    let chars = "        ".chars().collect();
    let result = parse_arguments(&InstructionMetaInfo::new(), &chars, 0);

    assert!(result.is_ok());

    let arguments = result.unwrap();

    assert!(arguments.is_none());
}

#[test]
fn parse_arguments_only_value() {
    let chars = "test".chars().collect();
    let result = parse_arguments(&InstructionMetaInfo::new(), &chars, 0);

    assert!(result.is_ok());

    let arguments = result.unwrap();

    assert_eq!(arguments.unwrap(), vec!["test"]);
}

#[test]
fn parse_arguments_value_in_middle_of_spaces() {
    let chars = "  test  ".chars().collect();
    let result = parse_arguments(&InstructionMetaInfo::new(), &chars, 0);

    assert!(result.is_ok());

    let arguments = result.unwrap();

    assert_eq!(arguments.unwrap(), vec!["test"]);
}

#[test]
fn parse_arguments_value_with_equals() {
    let chars = "  a=b  ".chars().collect();
    let result = parse_arguments(&InstructionMetaInfo::new(), &chars, 0);

    assert!(result.is_ok());

    let arguments = result.unwrap();

    assert_eq!(arguments.unwrap(), vec!["a=b"]);
}

#[test]
fn parse_arguments_multiple_values() {
    let chars = r#"  test  "test \"test\"" ${value} %{value}"#.chars().collect();
    let result = parse_arguments(&InstructionMetaInfo::new(), &chars, 0);

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
    let result = find_label(&InstructionMetaInfo::new(), &chars, 0);

    assert!(result.is_ok());

    let (index, value) = result.unwrap();

    assert_eq!(index, chars.len());
    assert!(value.is_none());
}

#[test]
fn find_label_only_spaces() {
    let chars = "        ".chars().collect();
    let result = find_label(&InstructionMetaInfo::new(), &chars, 0);

    assert!(result.is_ok());

    let (index, value) = result.unwrap();

    assert_eq!(index, chars.len());
    assert!(value.is_none());
}

#[test]
fn find_label_none_label_between_spaces() {
    let chars = "   label     ".chars().collect();
    let result = find_label(&InstructionMetaInfo::new(), &chars, 0);

    assert!(result.is_ok());

    let (index, value) = result.unwrap();

    assert_eq!(index, 3);
    assert!(value.is_none());
}

#[test]
fn find_label_label_between_spaces() {
    let chars = "   :label     ".chars().collect();
    let result = find_label(&InstructionMetaInfo::new(), &chars, 0);

    assert!(result.is_ok());

    let (index, value) = result.unwrap();

    assert_eq!(index, 9);
    assert_eq!(value.unwrap(), ":label");
}

#[test]
fn find_label_label_with_comment_afterwards() {
    let chars = "   :label#comment".chars().collect();
    let result = find_label(&InstructionMetaInfo::new(), &chars, 0);

    assert!(result.is_ok());

    let (index, value) = result.unwrap();

    assert_eq!(index, chars.len());
    assert_eq!(value.unwrap(), ":label");
}

#[test]
fn find_label_label_with_quote_error() {
    let chars = r#"   :"label"     "#.chars().collect();
    let result = find_label(&InstructionMetaInfo::new(), &chars, 0);

    assert!(result.is_err());
}

#[test]
fn find_label_label_with_control_error() {
    let chars = r#"   :\\label     "#.chars().collect();
    let result = find_label(&InstructionMetaInfo::new(), &chars, 0);

    assert!(result.is_err());
}

#[test]
fn find_label_post_value_index() {
    let chars = r#"   :label     "#.chars().collect();
    let result = find_label(&InstructionMetaInfo::new(), &chars, chars.len() - 2);

    assert!(result.is_ok());

    let (index, value) = result.unwrap();

    assert_eq!(index, chars.len());
    assert!(value.is_none());
}

#[test]
fn find_output_and_command_empty() {
    let chars = "".chars().collect();
    let mut instruction = ScriptInstruction::new();
    let result = find_output_and_command(&InstructionMetaInfo::new(), &chars, 0, &mut instruction);

    assert!(result.is_ok());

    let index = result.unwrap();

    assert_eq!(index, chars.len());
    assert!(instruction.output.is_none());
    assert!(instruction.command.is_none());
}

#[test]
fn find_output_and_command_only_spaces() {
    let chars = "        ".chars().collect();
    let mut instruction = ScriptInstruction::new();
    let result = find_output_and_command(&InstructionMetaInfo::new(), &chars, 0, &mut instruction);

    assert!(result.is_ok());

    let index = result.unwrap();

    assert_eq!(index, chars.len());
    assert!(instruction.output.is_none());
    assert!(instruction.command.is_none());
}

#[test]
fn find_output_and_command_single_value_between_spaces() {
    let chars = "   command     ".chars().collect();
    let mut instruction = ScriptInstruction::new();
    let result = find_output_and_command(&InstructionMetaInfo::new(), &chars, 0, &mut instruction);

    assert!(result.is_ok());

    let index = result.unwrap();

    assert_eq!(index, 10);
    assert!(instruction.output.is_none());
    assert_eq!(instruction.command.unwrap(), "command");
}

#[test]
fn find_output_and_command_single_value_only() {
    let chars = "command".chars().collect();
    let mut instruction = ScriptInstruction::new();
    let result = find_output_and_command(&InstructionMetaInfo::new(), &chars, 0, &mut instruction);

    assert!(result.is_ok());

    let index = result.unwrap();

    assert_eq!(index, chars.len());
    assert!(instruction.output.is_none());
    assert_eq!(instruction.command.unwrap(), "command");
}

#[test]
fn find_output_and_command_single_value_with_comment_afterwards() {
    let chars = "   command#comment".chars().collect();
    let mut instruction = ScriptInstruction::new();
    let result = find_output_and_command(&InstructionMetaInfo::new(), &chars, 0, &mut instruction);

    assert!(result.is_ok());

    let index = result.unwrap();

    assert_eq!(index, chars.len());
    assert!(instruction.output.is_none());
    assert_eq!(instruction.command.unwrap(), "command");
}

#[test]
fn find_output_and_command_single_value_with_quote_error() {
    let chars = r#"   "command"     "#.chars().collect();
    let mut instruction = ScriptInstruction::new();
    let result = find_output_and_command(&InstructionMetaInfo::new(), &chars, 0, &mut instruction);

    assert!(result.is_err());
}

#[test]
fn find_output_and_command_single_value_with_control_error() {
    let chars = r#"   c\\ommand     "#.chars().collect();
    let mut instruction = ScriptInstruction::new();
    let result = find_output_and_command(&InstructionMetaInfo::new(), &chars, 0, &mut instruction);

    assert!(result.is_err());
}

#[test]
fn find_output_and_command_post_value_index() {
    let chars = r#"   command     "#.chars().collect();
    let mut instruction = ScriptInstruction::new();
    let result = find_output_and_command(
        &InstructionMetaInfo::new(),
        &chars,
        chars.len() - 2,
        &mut instruction,
    );

    assert!(result.is_ok());

    let index = result.unwrap();

    assert_eq!(index, chars.len());
    assert!(instruction.output.is_none());
    assert!(instruction.command.is_none());
}

#[test]
fn find_output_and_command_single_value_with_equals_between_spaces() {
    let chars = "   variable  =   ".chars().collect();
    let mut instruction = ScriptInstruction::new();
    let result = find_output_and_command(&InstructionMetaInfo::new(), &chars, 0, &mut instruction);

    assert!(result.is_ok());

    let index = result.unwrap();

    assert_eq!(index, 14);
    assert_eq!(instruction.output.unwrap(), "variable");
    assert!(instruction.command.is_none());
}

#[test]
fn find_output_and_command_single_value_with_equals_only() {
    let chars = "variable=".chars().collect();
    let mut instruction = ScriptInstruction::new();
    let result = find_output_and_command(&InstructionMetaInfo::new(), &chars, 0, &mut instruction);

    assert!(result.is_ok());

    let index = result.unwrap();

    assert_eq!(index, chars.len());
    assert_eq!(instruction.output.unwrap(), "variable");
    assert!(instruction.command.is_none());
}

#[test]
fn find_output_and_command_single_value_with_equals2_between_spaces() {
    let chars = "   variable=   ".chars().collect();
    let mut instruction = ScriptInstruction::new();
    let result = find_output_and_command(&InstructionMetaInfo::new(), &chars, 0, &mut instruction);

    assert!(result.is_ok());

    let index = result.unwrap();

    assert_eq!(index, 12);
    assert_eq!(instruction.output.unwrap(), "variable");
    assert!(instruction.command.is_none());
}

#[test]
fn find_output_and_command_single_value_with_equals_with_comment_afterwards() {
    let chars = "   variable=#comment".chars().collect();
    let mut instruction = ScriptInstruction::new();
    let result = find_output_and_command(&InstructionMetaInfo::new(), &chars, 0, &mut instruction);

    assert!(result.is_ok());

    let index = result.unwrap();

    assert_eq!(index, 12);
    assert_eq!(instruction.output.unwrap(), "variable");
    assert!(instruction.command.is_none());
}

#[test]
fn find_output_and_command_single_value_with_equals_with_quote_error() {
    let chars = r#"   "variable"  =   "#.chars().collect();
    let mut instruction = ScriptInstruction::new();
    let result = find_output_and_command(&InstructionMetaInfo::new(), &chars, 0, &mut instruction);

    assert!(result.is_err());
}

#[test]
fn find_output_and_command_single_value_with_equals_with_control_error() {
    let chars = r#"   v\\ariable  =   "#.chars().collect();
    let mut instruction = ScriptInstruction::new();
    let result = find_output_and_command(&InstructionMetaInfo::new(), &chars, 0, &mut instruction);

    assert!(result.is_err());
}

#[test]
fn find_output_and_command_post_value_index_with_equals() {
    let chars = r#"   variable =    "#.chars().collect();
    let mut instruction = ScriptInstruction::new();
    let result = find_output_and_command(
        &InstructionMetaInfo::new(),
        &chars,
        chars.len() - 2,
        &mut instruction,
    );

    assert!(result.is_ok());

    let index = result.unwrap();

    assert_eq!(index, chars.len());
    assert!(instruction.output.is_none());
    assert!(instruction.command.is_none());
}

#[test]
fn find_output_and_command_output_command_only() {
    let chars = r#"variable=command"#.chars().collect();
    let mut instruction = ScriptInstruction::new();
    let result = find_output_and_command(&InstructionMetaInfo::new(), &chars, 0, &mut instruction);

    assert!(result.is_ok());

    let index = result.unwrap();

    assert_eq!(index, chars.len());
    assert_eq!(instruction.output.unwrap(), "variable");
    assert_eq!(instruction.command.unwrap(), "command");
}

#[test]
fn find_output_and_command_output_command_with_spaces() {
    let chars = r#"    variable   =   command   "#.chars().collect();
    let mut instruction = ScriptInstruction::new();
    let result = find_output_and_command(&InstructionMetaInfo::new(), &chars, 0, &mut instruction);

    assert!(result.is_ok());

    let index = result.unwrap();

    assert_eq!(index, 26);
    assert_eq!(instruction.output.unwrap(), "variable");
    assert_eq!(instruction.command.unwrap(), "command");
}

#[test]
fn parse_command_line_empty() {
    let chars = r#""#.chars().collect();
    let result = parse_command_line(&chars, InstructionMetaInfo::new(), 0);

    assert!(result.is_ok());

    let instruction = result.unwrap();
    test::assert_empty_instruction(&instruction);
}

#[test]
fn parse_command_line_all_spaces() {
    let chars = r#"     "#.chars().collect();
    let result = parse_command_line(&chars, InstructionMetaInfo::new(), 0);

    assert!(result.is_ok());

    let instruction = result.unwrap();
    test::assert_empty_instruction(&instruction);
}

#[test]
fn parse_command_line_comment() {
    let chars = r#"  #test   "#.chars().collect();
    let result = parse_command_line(&chars, InstructionMetaInfo::new(), 0);

    assert!(result.is_ok());

    let instruction = result.unwrap();
    test::assert_empty_instruction(&instruction);
}

#[test]
fn parse_command_line_only_label() {
    let chars = r#":label"#.chars().collect();
    let result = parse_command_line(&chars, InstructionMetaInfo::new(), 0);

    assert!(result.is_ok());

    let instruction = result.unwrap();
    let instruction_type = test::get_script_instruction(&instruction);

    assert_eq!(instruction_type.label.unwrap(), ":label");
    assert!(instruction_type.output.is_none());
    assert!(instruction_type.command.is_none());
    assert!(instruction_type.arguments.is_none());
}

#[test]
fn parse_command_line_only_output_variable() {
    let chars = r#"variable="#.chars().collect();
    let result = parse_command_line(&chars, InstructionMetaInfo::new(), 0);

    assert!(result.is_ok());

    let instruction = result.unwrap();
    let instruction_type = test::get_script_instruction(&instruction);

    assert!(instruction_type.label.is_none());
    assert_eq!(instruction_type.output.unwrap(), "variable");
    assert!(instruction_type.command.is_none());
    assert!(instruction_type.arguments.is_none());
}

#[test]
fn parse_command_line_only_command() {
    let chars = r#"command"#.chars().collect();
    let result = parse_command_line(&chars, InstructionMetaInfo::new(), 0);

    assert!(result.is_ok());

    let instruction = result.unwrap();
    let instruction_type = test::get_script_instruction(&instruction);

    assert!(instruction_type.label.is_none());
    assert!(instruction_type.output.is_none());
    assert_eq!(instruction_type.command.unwrap(), "command");
    assert!(instruction_type.arguments.is_none());
}

#[test]
fn parse_command_line_all_no_spaces() {
    let chars = r#":label variable=command arg1 arg2 arg3"#.chars().collect();
    let result = parse_command_line(&chars, InstructionMetaInfo::new(), 0);

    assert!(result.is_ok());

    let instruction = result.unwrap();
    let instruction_type = test::get_script_instruction(&instruction);

    assert_eq!(instruction_type.label.unwrap(), ":label");
    assert_eq!(instruction_type.output.unwrap(), "variable");
    assert_eq!(instruction_type.command.unwrap(), "command");
    assert_eq!(
        instruction_type.arguments.unwrap(),
        vec!["arg1", "arg2", "arg3"]
    );
}

#[test]
fn parse_command_line_all_with_spaces() {
    let chars = r#"   :label   variable   =  command    arg1   arg2 arg3   "#
        .chars()
        .collect();
    let result = parse_command_line(&chars, InstructionMetaInfo::new(), 0);

    assert!(result.is_ok());

    let instruction = result.unwrap();
    let instruction_type = test::get_script_instruction(&instruction);

    assert_eq!(instruction_type.label.unwrap(), ":label");
    assert_eq!(instruction_type.output.unwrap(), "variable");
    assert_eq!(instruction_type.command.unwrap(), "command");
    assert_eq!(
        instruction_type.arguments.unwrap(),
        vec!["arg1", "arg2", "arg3"]
    );
}

#[test]
fn parse_command_line_all_complex() {
    let chars =
        r#":label_test variable_test = command_test arg1 arg2 "  arg3 arg3 arg3  " #some comment"#
            .chars()
            .collect();
    let result = parse_command_line(&chars, InstructionMetaInfo::new(), 0);

    assert!(result.is_ok());

    let instruction = result.unwrap();
    let instruction_type = test::get_script_instruction(&instruction);

    assert_eq!(instruction_type.label.unwrap(), ":label_test");
    assert_eq!(instruction_type.output.unwrap(), "variable_test");
    assert_eq!(instruction_type.command.unwrap(), "command_test");
    assert_eq!(
        instruction_type.arguments.unwrap(),
        vec!["arg1", "arg2", "  arg3 arg3 arg3  "]
    );
}

#[test]
fn parse_lines_all() {
    let text = r#"
!print test pre process line

:label_test variable_test = command_test arg1 arg2 "  arg3 arg3 arg3  " #some comment

#comment
!print test
"#;
    let result = parse_lines(&text, InstructionMetaInfo::new());

    assert!(result.is_ok());

    let instructions = result.unwrap();

    assert_eq!(instructions.len(), 7);

    let mut line_number = 1;
    for instruction in &instructions {
        assert_eq!(line_number, instruction.meta_info.line.unwrap());
        line_number = line_number + 1;
    }

    test::assert_empty_instruction(&instructions[0]);
    test::assert_empty_instruction(&instructions[2]);
    test::assert_empty_instruction(&instructions[4]);
    test::assert_empty_instruction(&instructions[5]);

    let mut pre_process_instruction = test::get_pre_process_instruction(&instructions[1]);
    assert_eq!(pre_process_instruction.command.unwrap(), "print");
    assert_eq!(
        pre_process_instruction.arguments.unwrap(),
        vec!["test", "pre", "process", "line"]
    );
    pre_process_instruction = test::get_pre_process_instruction(&instructions[6]);
    assert_eq!(pre_process_instruction.command.unwrap(), "print");
    assert_eq!(pre_process_instruction.arguments.unwrap(), vec!["test"]);

    let script_instruction = test::get_script_instruction(&instructions[3]);
    assert_eq!(script_instruction.label.unwrap(), ":label_test");
    assert_eq!(script_instruction.output.unwrap(), "variable_test");
    assert_eq!(script_instruction.command.unwrap(), "command_test");
    assert_eq!(
        script_instruction.arguments.unwrap(),
        vec!["arg1", "arg2", "  arg3 arg3 arg3  "]
    );
}

#[test]
fn parse_file_not_found() {
    let result = parse_file("./src/test/scripts/not_found.ds");

    assert!(result.is_err());
}

#[test]
fn parse_file_simple() {
    let result = parse_file("./src/test/scripts/simple.ds");

    assert!(result.is_ok());

    let instructions = result.unwrap();

    assert_eq!(instructions.len(), 7);

    let mut line_number = 1;
    for instruction in &instructions {
        assert_eq!(line_number, instruction.meta_info.line.unwrap());
        line_number = line_number + 1;
    }

    test::assert_empty_instruction(&instructions[0]);
    test::assert_empty_instruction(&instructions[2]);
    test::assert_empty_instruction(&instructions[4]);
    test::assert_empty_instruction(&instructions[5]);

    let mut pre_process_instruction = test::get_pre_process_instruction(&instructions[1]);
    assert_eq!(pre_process_instruction.command.unwrap(), "print");
    assert_eq!(
        pre_process_instruction.arguments.unwrap(),
        vec!["test", "pre", "process", "line"]
    );
    pre_process_instruction = test::get_pre_process_instruction(&instructions[6]);
    assert_eq!(pre_process_instruction.command.unwrap(), "print");
    assert_eq!(pre_process_instruction.arguments.unwrap(), vec!["test"]);

    let script_instruction = test::get_script_instruction(&instructions[3]);
    assert_eq!(script_instruction.label.unwrap(), ":label_test");
    assert_eq!(script_instruction.output.unwrap(), "variable_test");
    assert_eq!(script_instruction.command.unwrap(), "command_test");
    assert_eq!(
        script_instruction.arguments.unwrap(),
        vec!["arg1", "arg2", "  arg3 arg3 arg3  "]
    );
}

#[test]
fn parse_file_single_extend() {
    let result = parse_file("./src/test/scripts/include_file.ds");

    assert!(result.is_ok());

    let instructions = result.unwrap();

    assert_eq!(instructions.len(), 8);

    let mut line_number = 1;
    let mut skip_line = true;
    for instruction in &instructions {
        assert_eq!(line_number, instruction.meta_info.line.unwrap());
        if skip_line {
            skip_line = false;
        } else {
            line_number = line_number + 1;
        }
    }

    test::assert_empty_instruction(&instructions[1]);
    test::assert_empty_instruction(&instructions[3]);
    test::assert_empty_instruction(&instructions[6]);
    test::assert_empty_instruction(&instructions[5]);

    let mut pre_process_instruction = test::get_pre_process_instruction(&instructions[0]);
    assert_eq!(pre_process_instruction.command.unwrap(), "include_files");
    assert_eq!(
        pre_process_instruction.arguments.unwrap(),
        vec!["./simple.ds"]
    );
    pre_process_instruction = test::get_pre_process_instruction(&instructions[2]);
    assert_eq!(pre_process_instruction.command.unwrap(), "print");
    assert_eq!(
        pre_process_instruction.arguments.unwrap(),
        vec!["test", "pre", "process", "line"]
    );
    pre_process_instruction = test::get_pre_process_instruction(&instructions[7]);
    assert_eq!(pre_process_instruction.command.unwrap(), "print");
    assert_eq!(pre_process_instruction.arguments.unwrap(), vec!["test"]);

    let script_instruction = test::get_script_instruction(&instructions[4]);
    assert_eq!(script_instruction.label.unwrap(), ":label_test");
    assert_eq!(script_instruction.output.unwrap(), "variable_test");
    assert_eq!(script_instruction.command.unwrap(), "command_test");
    assert_eq!(
        script_instruction.arguments.unwrap(),
        vec!["arg1", "arg2", "  arg3 arg3 arg3  "]
    );
}
