//
// TODO: remove these nightly features once stabilized
//
#![feature(unqualified_local_imports)]
#![feature(must_not_suspend)]
#![feature(non_exhaustive_omitted_patterns_lint)]
//
// Forbids
//
#![forbid(unsafe_code)]
#![forbid(unused_must_use)]
//
// -> Clippy
//
#![deny(clippy::as_conversions)]
#![deny(clippy::cfg_not_test)]
#![deny(clippy::empty_drop)]
#![deny(clippy::float_cmp)]
#![deny(clippy::float_cmp_const)]
#![deny(clippy::fn_to_numeric_cast_any)]
//
// Warnings
//
#![warn(unnameable_types)]
#![warn(unqualified_local_imports)]
#![warn(unused_crate_dependencies)]
#![warn(must_not_suspend)]
#![warn(non_exhaustive_omitted_patterns)]
#![warn(redundant_lifetimes)]
#![warn(redundant_imports)]
#![warn(single_use_lifetimes)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unit_bindings)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]
//
// -> Clippy
//
// #![warn(clippy::arithmetic_side_effects)]
#![warn(clippy::assigning_clones)]
#![warn(clippy::big_endian_bytes)]
#![warn(clippy::bool_to_int_with_if)]
#![warn(clippy::cargo_common_metadata)]
#![warn(clippy::case_sensitive_file_extension_comparisons)]
#![warn(clippy::clear_with_drain)]
#![warn(clippy::cloned_instead_of_copied)]
#![warn(clippy::coerce_container_to_any)]
#![warn(clippy::collection_is_never_read)]
#![warn(clippy::comparison_chain)]
#![warn(clippy::copy_iterator)]
#![warn(clippy::dbg_macro)]
#![warn(clippy::debug_assert_with_mut_call)]
#![warn(clippy::default_trait_access)]
#![warn(clippy::deref_by_slicing)]
#![warn(clippy::derive_partial_eq_without_eq)]
#![warn(clippy::doc_broken_link)]
#![warn(clippy::doc_comment_double_space_linebreaks)]
#![warn(clippy::doc_include_without_cfg)]
#![warn(clippy::doc_link_code)]
#![warn(clippy::doc_link_with_quotes)]
#![warn(clippy::doc_markdown)]
#![warn(clippy::elidable_lifetime_names)]
#![warn(clippy::empty_enum_variants_with_brackets)]
#![warn(clippy::empty_enums)]
#![warn(clippy::empty_structs_with_brackets)]
#![warn(clippy::enum_glob_use)]
#![warn(clippy::equatable_if_let)]
#![warn(clippy::error_impl_error)]
#![warn(clippy::exit)]
#![warn(clippy::expl_impl_clone_on_copy)]
#![warn(clippy::explicit_into_iter_loop)]
#![warn(clippy::explicit_iter_loop)]
#![warn(clippy::filter_map_next)]
#![warn(clippy::flat_map_option)]
#![warn(clippy::fn_params_excessive_bools)]
#![warn(clippy::format_collect)]
#![warn(clippy::from_iter_instead_of_collect)]
#![warn(clippy::future_not_send)]
#![warn(clippy::if_then_some_else_none)]
#![warn(clippy::ignore_without_reason)]
#![warn(clippy::ignored_unit_patterns)]
#![warn(clippy::implicit_clone)]
#![warn(clippy::imprecise_flops)]
#![warn(clippy::index_refutable_slice)]
#![warn(clippy::indexing_slicing)]
#![warn(clippy::inefficient_to_string)]
#![warn(clippy::infinite_loop)]
#![warn(clippy::inline_always)]
// #![warn(clippy::integer_division)]
#![warn(clippy::items_after_statements)]
#![warn(clippy::iter_not_returning_iterator)]
#![warn(clippy::iter_on_empty_collections)]
#![warn(clippy::iter_on_single_items)]
#![warn(clippy::iter_with_drain)]
#![warn(clippy::large_digit_groups)]
#![warn(clippy::large_futures)]
#![warn(clippy::large_include_file)]
#![warn(clippy::large_stack_arrays)]
#![warn(clippy::large_stack_frames)]
#![warn(clippy::large_types_passed_by_value)]
#![warn(clippy::linkedlist)]
#![warn(clippy::literal_string_with_formatting_args)]
#![warn(clippy::lossy_float_literal)]
#![warn(clippy::macro_use_imports)]
#![warn(clippy::manual_assert)]
#![warn(clippy::manual_instant_elapsed)]
#![warn(clippy::manual_is_power_of_two)]
#![warn(clippy::manual_is_variant_and)]
#![warn(clippy::manual_let_else)]
#![warn(clippy::manual_midpoint)]
#![warn(clippy::manual_string_new)]
#![warn(clippy::many_single_char_names)]
// #![warn(clippy::map_err_ignore)]
#![warn(clippy::map_unwrap_or)]
#![warn(clippy::map_with_unused_argument_over_ranges)]
#![warn(clippy::match_bool)]
#![warn(clippy::match_same_arms)]
#![warn(clippy::match_wild_err_arm)]
#![warn(clippy::match_wildcard_for_single_variants)]
#![warn(clippy::maybe_infinite_iter)]
#![warn(clippy::mem_forget)]
#![warn(clippy::mismatching_type_param_order)]
#![warn(clippy::missing_asserts_for_indexing)]
// #![warn(clippy::missing_const_for_fn)]
#![warn(clippy::missing_errors_doc)]
#![warn(clippy::missing_fields_in_debug)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::mixed_read_write_in_expression)]
// #![warn(clippy::multiple_crate_versions)]
#![warn(clippy::multiple_inherent_impl)]
#![warn(clippy::multiple_unsafe_ops_per_block)]
#![warn(clippy::must_use_candidate)]
#![warn(clippy::mut_mut)]
#![warn(clippy::mutex_atomic)]
#![warn(clippy::mutex_integer)]
#![warn(clippy::naive_bytecount)]
#![warn(clippy::needless_bitwise_bool)]
#![warn(clippy::needless_collect)]
#![warn(clippy::needless_continue)]
#![warn(clippy::needless_for_each)]
#![warn(clippy::needless_pass_by_ref_mut)]
#![warn(clippy::needless_pass_by_value)]
#![warn(clippy::needless_raw_string_hashes)]
#![warn(clippy::needless_raw_strings)]
#![warn(clippy::negative_feature_names)]
#![warn(clippy::no_effect_underscore_binding)]
#![warn(clippy::non_std_lazy_statics)]
#![warn(clippy::non_zero_suggestions)]
#![warn(clippy::nonstandard_macro_braces)]
#![warn(clippy::option_as_ref_cloned)]
// #![warn(clippy::option_if_let_else)]
#![warn(clippy::option_option)]
#![warn(clippy::or_fun_call)]
#![warn(clippy::partial_pub_fields)]
#![warn(clippy::path_buf_push_overwrite)]
#![warn(clippy::pathbuf_init_then_push)]
#![warn(clippy::precedence_bits)]
#![warn(clippy::pub_without_shorthand)]
#![warn(clippy::range_minus_one)]
#![warn(clippy::range_plus_one)]
#![warn(clippy::rc_buffer)]
#![warn(clippy::rc_mutex)]
#![warn(clippy::read_zero_byte_vec)]
#![warn(clippy::redundant_clone)]
#![warn(clippy::redundant_closure_for_method_calls)]
#![warn(clippy::redundant_else)]
#![warn(clippy::redundant_feature_names)]
#![warn(clippy::redundant_pub_crate)]
#![warn(clippy::redundant_type_annotations)]
#![warn(clippy::ref_binding_to_reference)]
#![warn(clippy::ref_option)]
#![warn(clippy::ref_option_ref)]
#![warn(clippy::ref_patterns)]
#![warn(clippy::renamed_function_params)]
#![warn(clippy::rest_pat_in_fully_bound_structs)]
#![warn(clippy::return_and_then)]
#![warn(clippy::return_self_not_must_use)]
#![warn(clippy::same_functions_in_if_condition)]
#![warn(clippy::same_name_method)]
#![warn(clippy::self_named_module_files)]
#![warn(clippy::self_only_used_in_recursion)]
#![warn(clippy::semicolon_if_nothing_returned)]
#![warn(clippy::semicolon_inside_block)]
#![warn(clippy::set_contains_or_insert)]
// #![warn(clippy::shadow_unrelated)]
#![warn(clippy::should_panic_without_expect)]
#![warn(clippy::single_char_pattern)]
#![warn(clippy::single_option_map)]
#![warn(clippy::str_split_at_newline)]
#![warn(clippy::str_to_string)]
#![warn(clippy::string_add)]
#![warn(clippy::string_add_assign)]
#![warn(clippy::string_lit_as_bytes)]
#![warn(clippy::string_lit_chars_any)]
#![warn(clippy::suboptimal_flops)]
#![warn(clippy::suspicious_operation_groupings)]
#![warn(clippy::suspicious_xor_used_as_pow)]
#![warn(clippy::tests_outside_test_module)]
#![warn(clippy::too_long_first_doc_paragraph)]
#![warn(clippy::trailing_empty_array)]
#![warn(clippy::trait_duplication_in_bounds)]
#![warn(clippy::trivial_regex)]
#![warn(clippy::try_err)]
#![warn(clippy::type_repetition_in_bounds)]
#![warn(clippy::unchecked_time_subtraction)]
#![warn(clippy::undocumented_unsafe_blocks)]
#![warn(clippy::unicode_not_nfc)]
#![warn(clippy::uninlined_format_args)]
#![warn(clippy::unnecessary_box_returns)]
#![warn(clippy::unnecessary_debug_formatting)]
#![warn(clippy::unnecessary_join)]
#![warn(clippy::unnecessary_literal_bound)]
#![warn(clippy::unnecessary_safety_comment)]
#![warn(clippy::unnecessary_safety_doc)]
#![warn(clippy::unnecessary_self_imports)]
#![warn(clippy::unnecessary_semicolon)]
#![warn(clippy::unnecessary_struct_initialization)]
#![warn(clippy::unnecessary_wraps)]
#![warn(clippy::unnested_or_patterns)]
#![warn(clippy::unreadable_literal)]
#![warn(clippy::unsafe_derive_deserialize)]
#![warn(clippy::unseparated_literal_suffix)]
#![warn(clippy::unused_async)]
#![warn(clippy::unused_peekable)]
#![warn(clippy::unused_result_ok)]
#![warn(clippy::unused_rounding)]
#![warn(clippy::unused_self)]
// #![warn(clippy::unused_trait_names)]
// #![warn(clippy::unwrap_in_result)]
// #![warn(clippy::unwrap_used)]
#![warn(clippy::use_self)]
#![warn(clippy::used_underscore_binding)]
#![warn(clippy::used_underscore_items)]
#![warn(clippy::useless_let_if_seq)]
#![warn(clippy::verbose_bit_mask)]
#![warn(clippy::verbose_file_reads)]
#![warn(clippy::while_float)]
#![warn(clippy::wildcard_dependencies)]
// #![warn(clippy::wildcard_enum_match_arm)]
#![warn(clippy::zero_sized_map_values)]

use std::process::ExitCode;

use anyhow::{Context, Result, bail};
use clap::Parser;
use log::error;
use tokio::{fs, task::spawn_blocking};

use self::{cmd::CmdArgs, logger::Logger, manager::DataManager};

mod arts;
mod cmd;
mod index;
mod indexer;
mod logger;
mod manager;
mod server;
mod utils;

#[tokio::main]
async fn main() -> ExitCode {
    let args = CmdArgs::parse();

    Logger::new(args.verbosity).init().unwrap();

    match inner_main(args).await {
        Ok(()) => ExitCode::SUCCESS,

        Err(err) => {
            error!("{err:?}");
            ExitCode::FAILURE
        }
    }
}

async fn inner_main(args: CmdArgs) -> Result<()> {
    let CmdArgs {
        music_dir,
        data_dir,
        verbosity: _,
        addr,
        port,
    } = args;

    if !fs::try_exists(&music_dir).await.is_ok_and(|b| b) {
        bail!(
            "Music directory '{}' is not a valid directory",
            music_dir.display()
        );
    }

    if !fs::try_exists(&data_dir).await.is_ok_and(|b| b) {
        fs::create_dir_all(&data_dir)
            .await
            .with_context(|| format!("Failed to create data directory '{}'", data_dir.display()))?;
    }

    let data_manager = spawn_blocking(move || DataManager::load(&data_dir, music_dir))
        .await
        .unwrap()?;

    server::launch((addr, port).into(), data_manager).await
}
