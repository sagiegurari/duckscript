#[cfg(test)]
#[path = "./flags_test.rs"]
mod flags_test;

pub(crate) fn is_unix_flags_argument(argument: &str) -> bool {
    if argument.is_empty() {
        false
    } else if argument.starts_with("-") && argument.len() > 1 {
        let argument_string = argument.to_string();
        let chars = argument_string[1..].chars();

        for letter in chars {
            if !letter.is_ascii_alphabetic() {
                return false;
            }
        }

        true
    } else {
        false
    }
}

pub(crate) fn is_unix_flag_exists(flag: char, flags: &str) -> bool {
    if !is_unix_flags_argument(flags) {
        false
    } else {
        let lowercase_flags = flags[1..].to_lowercase();
        let lowercase_flag = flag.to_string().to_lowercase();

        lowercase_flags.contains(&lowercase_flag)
    }
}
