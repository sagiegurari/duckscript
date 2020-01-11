#![deny(
    absolute_paths_not_starting_with_crate,
    ambiguous_associated_items,
    anonymous_parameters,
    const_err,
    dead_code,
    deprecated,
    deprecated_in_future,
    duplicate_macro_exports,
    ellipsis_inclusive_range_patterns,
    exceeding_bitshifts,
    explicit_outlives_requirements,
    exported_private_dependencies,
    ill_formed_attribute_input,
    illegal_floating_point_literal_pattern,
    improper_ctypes,
    indirect_structural_match,
    invalid_type_param_default,
    irrefutable_let_patterns,
    keyword_idents,
    late_bound_lifetime_arguments,
    legacy_constructor_visibility,
    legacy_directory_ownership,
    macro_expanded_macro_exports_accessed_by_absolute_paths,
    missing_copy_implementations,
    missing_docs,
    missing_fragment_specifier,
    mutable_borrow_reservation_conflict,
    mutable_transmutes,
    nested_impl_trait,
    no_mangle_const_items,
    no_mangle_generic_items,
    non_ascii_idents,
    non_camel_case_types,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    order_dependent_trait_objects,
    overflowing_literals,
    parenthesized_params_in_types_and_modules,
    path_statements,
    patterns_in_fns_without_body,
    plugin_as_library,
    private_doc_tests,
    private_in_public,
    proc_macro_derive_resolution_fallback,
    pub_use_of_private_extern_crate,
    safe_extern_statics,
    safe_packed_borrows,
    stable_features,
    trivial_bounds,
    trivial_casts,
    trivial_numeric_casts,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    unconditional_recursion,
    unions_with_drop_fields,
    unknown_crate_types,
    unnameable_test_items,
    unreachable_code,
    unreachable_patterns,
    unreachable_pub,
    unsafe_code,
    unstable_features,
    unstable_name_collisions,
    unused_allocation,
    unused_assignments,
    unused_attributes,
    unused_comparisons,
    unused_doc_comments,
    unused_extern_crates,
    unused_features,
    unused_import_braces,
    unused_imports,
    unused_labels,
    unused_lifetimes,
    unused_macros,
    unused_must_use,
    unused_mut,
    unused_parens,
    unused_qualifications,
    unused_unsafe,
    unused_variables,
    where_clauses_object_safety,
    while_true
)]
#![warn(macro_use_extern_crate, unknown_lints)]
#![allow(
    bare_trait_objects,
    box_pointers,
    elided_lifetimes_in_paths,
    intra_doc_link_resolution_failure,
    missing_doc_code_examples,
    missing_debug_implementations,
    single_use_lifetimes,
    unused_results,
    variant_size_differences,
    warnings,
    renamed_and_removed_lints
)]

//! # duckscript_cli
//!
//! The duckscript command line executable.
//!
//! This executable enables to run the duckscript runner with the default sdk.
//!
//! # Installation
//! See [main duckscript documentation](https://github.com/sagiegurari/duckscript)
//!
//! # Contributing
//! See [contributing guide](https://github.com/sagiegurari/duckscript/blob/master/.github/CONTRIBUTING.md)
//!
//! # License
//! Developed by Sagie Gur-Ari and licensed under the
//! [Apache 2](https://github.com/sagiegurari/duckscript/blob/master/LICENSE) open source license.
//!

mod error;

use crate::error::{CliError, ErrorInfo};
use duckscript::runner;
use duckscript::types::error::ScriptError;
use duckscript::types::runtime::Context;
use duckscriptsdk;
use std::env;

static VERSION: &str = env!("CARGO_PKG_VERSION");
static AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
static DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
    match run_cli() {
        Err(error) => println!("Error: {}", error),
        _ => (),
    };
}

fn run_cli() -> Result<(), CliError> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        Err(CliError {
            info: ErrorInfo::Cli("script file path not provided."),
        })
    } else if args[1] == "--help" || args[1] == "-h" {
        let usage = include_str!("help.txt");
        println!(
            "duckscript {}\n{}\n{}\n\n{}",
            VERSION, AUTHOR, DESCRIPTION, usage
        );

        Ok(())
    } else {
        let (value, is_file) = if args.len() == 2 {
            (args[1].clone(), true)
        } else {
            if args[1] == "-e" || args[1] == "--eval" {
                (args[2].clone(), false)
            } else {
                (args[1].clone(), true)
            }
        };

        match run_script(&value, is_file) {
            Err(error) => Err(CliError {
                info: ErrorInfo::Script(error),
            }),
            _ => Ok(()),
        }
    }
}

fn run_script(value: &str, is_file: bool) -> Result<(), ScriptError> {
    let mut context = Context::new();
    duckscriptsdk::load(&mut context.commands)?;

    if is_file {
        runner::run_script_file(value, context)?;
    } else {
        runner::run_script(value, context)?;
    }

    Ok(())
}
