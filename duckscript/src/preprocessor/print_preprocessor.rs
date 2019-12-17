//! # print_preprocessor
//!
//! The print pre processor implementation.
//!

pub(crate) fn run(arguments: &Option<Vec<String>>) {
    if let Some(arguments_vec) = arguments {
        for argument in arguments_vec {
            print!("{} ", argument);
        }
    }

    println!("");
}
