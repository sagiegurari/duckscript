#![deny(
    absolute_paths_not_starting_with_crate,
    ambiguous_associated_items,
    anonymous_parameters,
    arithmetic_overflow,
    array_into_iter,
    bindings_with_variant_name,
    coherence_leak_check,
    conflicting_repr_hints,
    const_err,
    dead_code,
    deprecated,
    deprecated_in_future,
    ellipsis_inclusive_range_patterns,
    explicit_outlives_requirements,
    exported_private_dependencies,
    ill_formed_attribute_input,
    illegal_floating_point_literal_pattern,
    improper_ctypes,
    incomplete_features,
    indirect_structural_match,
    inline_no_sanitize,
    invalid_codeblock_attribute,
    invalid_type_param_default,
    invalid_value,
    irrefutable_let_patterns,
    keyword_idents,
    late_bound_lifetime_arguments,
    macro_expanded_macro_exports_accessed_by_absolute_paths,
    meta_variable_misuse,
    missing_copy_implementations,
    missing_crate_level_docs,
    missing_docs,
    missing_fragment_specifier,
    mutable_borrow_reservation_conflict,
    mutable_transmutes,
    no_mangle_const_items,
    no_mangle_generic_items,
    non_ascii_idents,
    non_camel_case_types,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    order_dependent_trait_objects,
    overflowing_literals,
    overlapping_patterns,
    path_statements,
    patterns_in_fns_without_body,
    private_doc_tests,
    private_in_public,
    proc_macro_derive_resolution_fallback,
    pub_use_of_private_extern_crate,
    redundant_semicolons,
    safe_packed_borrows,
    soft_unstable,
    stable_features,
    trivial_bounds,
    trivial_casts,
    trivial_numeric_casts,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    uncommon_codepoints,
    unconditional_panic,
    unconditional_recursion,
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
    unused_braces,
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

//! # duckscript_sdk
//!
//! This library contains the standard command library for duckscript.
//!
//! The standard library is not required to use duckscript but it contains common commands and will expand over time.
//!
//! The library is split to 2 parts:
//! * std - The public standard library commands
//! * internal - Used internally by duckscript but also available publicly
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

pub(crate) mod sdk;
mod types;
mod utils;

#[cfg(test)]
mod test;

use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

#[cfg(test)]
#[path = "./lib_test.rs"]
mod lib_test;

static VERSION: &str = env!("CARGO_PKG_VERSION");

/// Returns the library version.
pub fn version() -> String {
    VERSION.to_string()
}

/// Loads all core commands
pub fn load(commands: &mut Commands) -> Result<(), ScriptError> {
    sdk::load(commands)
}
