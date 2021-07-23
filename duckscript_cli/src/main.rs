#![deny(
    absolute_paths_not_starting_with_crate,
    ambiguous_associated_items,
    anonymous_parameters,
    arithmetic_overflow,
    array_into_iter,
    asm_sub_register,
    bad_asm_style,
    bindings_with_variant_name,
    cenum_impl_drop_cast,
    clashing_extern_declarations,
    coherence_leak_check,
    conflicting_repr_hints,
    confusable_idents,
    const_err,
    const_evaluatable_unchecked,
    const_item_mutation,
    dead_code,
    deprecated,
    deprecated_in_future,
    deref_nullptr,
    drop_bounds,
    dyn_drop,
    ellipsis_inclusive_range_patterns,
    explicit_outlives_requirements,
    exported_private_dependencies,
    forbidden_lint_groups,
    function_item_references,
    ill_formed_attribute_input,
    illegal_floating_point_literal_pattern,
    improper_ctypes,
    improper_ctypes_definitions,
    incomplete_features,
    incomplete_include,
    indirect_structural_match,
    ineffective_unstable_trait_impl,
    inline_no_sanitize,
    invalid_doc_attributes,
    invalid_type_param_default,
    invalid_value,
    irrefutable_let_patterns,
    keyword_idents,
    large_assignments,
    late_bound_lifetime_arguments,
    legacy_derive_helpers,
    macro_expanded_macro_exports_accessed_by_absolute_paths,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_docs,
    missing_fragment_specifier,
    mixed_script_confusables,
    mutable_borrow_reservation_conflict,
    mutable_transmutes,
    no_mangle_const_items,
    no_mangle_generic_items,
    non_ascii_idents,
    non_camel_case_types,
    non_fmt_panics,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    nontrivial_structural_match,
    noop_method_call,
    order_dependent_trait_objects,
    overflowing_literals,
    overlapping_range_endpoints,
    path_statements,
    patterns_in_fns_without_body,
    pointer_structural_match,
    private_in_public,
    proc_macro_back_compat,
    proc_macro_derive_resolution_fallback,
    pub_use_of_private_extern_crate,
    redundant_semicolons,
    rust_2021_incompatible_closure_captures,
    rust_2021_incompatible_or_patterns,
    rust_2021_prefixes_incompatible_syntax,
    rust_2021_prelude_collisions,
    semicolon_in_expressions_from_macros,
    soft_unstable,
    stable_features,
    temporary_cstring_as_ptr,
    trivial_bounds,
    trivial_casts,
    trivial_numeric_casts,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    unaligned_references,
    uncommon_codepoints,
    unconditional_panic,
    unconditional_recursion,
    uninhabited_static,
    unknown_crate_types,
    unnameable_test_items,
    unreachable_code,
    unreachable_patterns,
    unreachable_pub,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unstable_features,
    unstable_name_collisions,
    unsupported_calling_conventions,
    unsupported_naked_functions,
    unused_allocation,
    unused_assignments,
    unused_attributes,
    unused_braces,
    unused_comparisons,
    unused_crate_dependencies,
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
    useless_deprecated
)]
#![warn(macro_use_extern_crate, unknown_lints)]
#![allow(
    bare_trait_objects,
    box_pointers,
    elided_lifetimes_in_paths,
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

mod linter;

use duckscript::runner;
use duckscript::types::error::ScriptError;
use duckscript::types::runtime::Context;
use duckscriptsdk;
use std::env;
use std::process::exit;

static VERSION: &str = env!("CARGO_PKG_VERSION");
static AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
static DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
    match run_cli() {
        Err(error) => {
            println!("Error: {}", error);
            exit(1);
        }
        _ => (),
    };
}

fn run_cli() -> Result<(), ScriptError> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        run_repl()
    } else if args[1] == "--version" {
        println!(
            "Duckscript Runtime: {}\nDuckscript SDK: {}\nDuckscript CLI: {}",
            duckscript::version(),
            duckscriptsdk::version(),
            VERSION
        );

        Ok(())
    } else if args[1] == "--help" || args[1] == "-h" {
        let usage = include_str!("help.txt");
        println!(
            "duckscript {}\n{}\n{}\n\n{}",
            VERSION, AUTHOR, DESCRIPTION, usage
        );

        Ok(())
    } else {
        let (value, is_file, run) = if args.len() == 2 {
            (args[1].clone(), true, true)
        } else {
            if args[1] == "-e" || args[1] == "--eval" {
                (args[2].clone(), false, true)
            } else if args[1] == "-l" || args[1] == "--lint" {
                (args[2].clone(), true, false)
            } else {
                (args[1].clone(), true, true)
            }
        };

        if run {
            run_script(&value, is_file)
        } else {
            linter::lint_file(&value)
        }
    }
}

fn create_context() -> Result<Context, ScriptError> {
    let mut context = Context::new();
    duckscriptsdk::load(&mut context.commands)?;

    Ok(context)
}

fn run_script(value: &str, is_file: bool) -> Result<(), ScriptError> {
    let context = create_context()?;

    if is_file {
        runner::run_script_file(value, context)?;
    } else {
        runner::run_script(value, context)?;
    }

    Ok(())
}

fn run_repl() -> Result<(), ScriptError> {
    let context = create_context()?;

    runner::repl(context)?;

    Ok(())
}
