//! # expansion
//!
//! The expand utility functions.
//!

#[cfg(test)]
#[path = "./expansion_test.rs"]
mod expansion_test;

use std::collections::HashMap;

fn should_break_key(value: char) -> bool {
    value == ' ' || value == '\n' || value == '\t' || value == '\r' || value == '='
}

pub(crate) fn expand_by_wrapper(
    value: &str,
    prefix: &str,
    suffix: char,
    variables: &HashMap<String, String>,
) -> String {
    let mut value_string = String::new();

    let prefix_length = prefix.len();
    let mut prefix_index = 0;
    let prefix_chars: Vec<char> = prefix.chars().collect();

    let mut found_prefix = false;
    let mut key = String::new();
    for next_char in value.chars() {
        if !found_prefix {
            if next_char == prefix_chars[prefix_index] {
                prefix_index = prefix_index + 1;

                if prefix_index == prefix_length {
                    found_prefix = true;
                    prefix_index = 0;
                    key.clear();
                }
            } else {
                if prefix_index > 0 {
                    value_string.push_str(&prefix[..prefix_index]);
                    prefix_index = 0;
                }
                value_string.push(next_char);
            }
        } else if next_char == suffix {
            match variables.get(&key) {
                Some(variable_value) => value_string.push_str(&variable_value),
                _ => (),
            };

            key.clear();
            found_prefix = false;
        } else if should_break_key(next_char) {
            value_string.push_str(&prefix);
            value_string.push_str(&key);
            value_string.push(next_char);

            key.clear();
            found_prefix = false;
        } else {
            key.push(next_char);
        }
    }

    if key.len() > 0 {
        value_string.push_str(&prefix);
        value_string.push_str(&key);
    }

    value_string
}
