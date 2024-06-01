#![deny(
    future_incompatible,
    keyword_idents,
    let_underscore,
    nonstandard_style,
    unused
)]
#![warn(unknown_lints)]

//! # duckscript
//!
//! Simple, extendable and embeddable scripting language.
//!
//! This library is the actual script parser and runner.<br>
//! On it's own, it has no actual commands as those need to be provided externally (see duckscript_sdk).
//!
//! # Installation
//! In order to use this library, just add it as a dependency:
//!
//! ```ini
//! [dependencies]
//! duckscript = "*"
//! ```
//!
//! # Contributing
//! See [contributing guide](https://github.com/sagiegurari/duckscript/blob/master/.github/CONTRIBUTING.md)
//!
//! # License
//! Developed by Sagie Gur-Ari and licensed under the
//! [Apache 2](https://github.com/sagiegurari/duckscript/blob/master/LICENSE) open source license.
//!

mod expansion;
pub mod parser;
mod preprocessor;
pub mod runner;
pub mod types;

#[cfg(test)]
mod test;

#[cfg(test)]
#[path = "./lib_test.rs"]
mod lib_test;

static VERSION: &str = env!("CARGO_PKG_VERSION");

/// Returns the library version.
pub fn version() -> String {
    VERSION.to_string()
}
