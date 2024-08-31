//! # env
//!
//! The runtime env structures.
//!

#[cfg(test)]
#[path = "./env_test.rs"]
mod env_test;

use std::io::{stderr, stdout, Write};

/// The runtime env
pub struct Env {
    /// The output writer
    pub out: Box<dyn Write>,
    /// The error writer
    pub err: Box<dyn Write>,
}

impl Env {
    /// Creates and returns a new instance.
    pub fn default() -> Env {
        Env {
            out: Box::new(stdout()),
            err: Box::new(stderr()),
        }
    }

    /// Creates and returns a new instance.
    pub fn new(out: Option<Box<dyn Write>>, err: Option<Box<dyn Write>>) -> Env {
        Env {
            out: match out {
                Some(value) => value,
                None => Box::new(stdout()),
            },
            err: match err {
                Some(value) => value,
                None => Box::new(stderr()),
            },
        }
    }
}
