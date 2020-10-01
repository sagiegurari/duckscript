//! # expansion
//!
//! The expand utility functions.
//!

#[cfg(test)]
#[path = "./expansion_test.rs"]
mod expansion_test;

use crate::parser;
use crate::types::instruction::InstructionMetaInfo;
use std::collections::HashMap;

pub(crate) enum ExpandedValue {
    Single(String),
    Multi(Vec<String>),
    None,
}

fn should_break_key(value: char) -> bool {
    value == ' ' || value == '\n' || value == '\t' || value == '\r' || value == '='
}

fn push_prefix(buffer: &mut String, single_type: bool, found_prefix_fully: bool) {
    if single_type {
        buffer.push('$');
    } else {
        buffer.push('%');
    }

    if found_prefix_fully {
        buffer.push('{');
    }
}

pub(crate) fn expand_by_wrapper(
    value: &str,
    meta_info: &InstructionMetaInfo,
    variables: &HashMap<String, String>,
) -> ExpandedValue {
    let mut value_string = String::new();

    let mut prefix_index = 0;
    let mut found_prefix = false;
    let mut key = String::new();
    let mut force_push = false;
    let mut single_type = true;
    for next_char in value.chars() {
        if !found_prefix {
            if next_char == '\\' && prefix_index == 0 {
                force_push = true
            } else if force_push {
                if next_char != '$' && next_char != '%' {
                    value_string.push('\\');
                }
                value_string.push(next_char);
                force_push = false;
            } else if prefix_index == 0 && (next_char == '$' || next_char == '%') {
                prefix_index = 1;

                single_type = if next_char == '$' { true } else { false };
            } else if prefix_index == 1 && next_char == '{' {
                found_prefix = true;
                prefix_index = 0;
                key.clear();
            } else {
                if prefix_index > 0 || found_prefix {
                    push_prefix(&mut value_string, single_type, found_prefix);
                    prefix_index = 0;
                }
                value_string.push(next_char);
            }
        } else if next_char == '}' {
            match variables.get(&key) {
                Some(variable_value) => value_string.push_str(&variable_value),
                _ => (),
            };

            key.clear();
            found_prefix = false;
        } else if should_break_key(next_char) {
            if prefix_index > 0 || found_prefix {
                push_prefix(&mut value_string, single_type, found_prefix);
                prefix_index = 0;
            }
            value_string.push_str(&key);
            value_string.push(next_char);

            key.clear();
            found_prefix = false;
        } else {
            key.push(next_char);
        }
    }

    if force_push {
        value_string.push('\\');
    } else if key.len() > 0 {
        if prefix_index > 0 || found_prefix {
            push_prefix(&mut value_string, single_type, found_prefix);
        }
        value_string.push_str(&key);
    } else if prefix_index == 1 {
        push_prefix(&mut value_string, single_type, false);
        single_type = true;
    }

    if value_string.is_empty() {
        ExpandedValue::None
    } else if single_type {
        ExpandedValue::Single(value_string.to_string())
    } else {
        let chars = value_string.to_string().chars().collect();
        match parser::reparse_arguments(meta_info, &chars, 0) {
            Ok(values_option) => match values_option {
                Some(values) => ExpandedValue::Multi(values),
                None => ExpandedValue::None,
            },
            Err(_) => ExpandedValue::None,
        }
    }
}
