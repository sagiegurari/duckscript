//! # env
//!
//! The runtime env structures.
//!

#[cfg(test)]
#[path = "./env_test.rs"]
mod env_test;

use std::io::{stderr, stdout, Write};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

/// The runtime env
pub struct Env {
    /// The output writer
    pub out: Box<dyn Write>,
    /// The error writer
    pub err: Box<dyn Write>,
    /// The halt token - Set to true to exit after current instruction
    pub halt: Arc<AtomicBool>,
}

impl Env {
    /// Creates and returns a new instance.
    pub fn default() -> Env {
        Env {
            out: Box::new(stdout()),
            err: Box::new(stderr()),
            halt: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Creates and returns a new instance.
    pub fn new(out: Option<Box<dyn Write>>, err: Option<Box<dyn Write>>, halt: Option<Arc<AtomicBool>>) -> Env {
        Env {
            out: out.unwrap_or_else(|| Box::new(stdout())),
            err: err.unwrap_or_else(|| Box::new(stderr())),
            halt: halt.unwrap_or_else(|| Arc::new(AtomicBool::new(false))),
        }
    }
}
