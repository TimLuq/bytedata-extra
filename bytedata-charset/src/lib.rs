//! A library for working with charsets to transform bytes to 

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(feature = "avx512f", feature = "nightly"), feature(avx512_target_feature, stdarch_x86_avx512))]
#![cfg_attr(docsrs, feature(doc_cfg))]

#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::module_name_repetitions)]
#![warn(
    clippy::alloc_instead_of_core,
    clippy::allow_attributes,
    clippy::allow_attributes_without_reason,
    clippy::as_underscore,
    clippy::assertions_on_result_states,
    clippy::clone_on_ref_ptr,
    clippy::create_dir,
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::default_numeric_fallback,
    clippy::default_union_representation,
    clippy::deref_by_slicing,
    clippy::else_if_without_else,
    clippy::empty_drop,
    clippy::empty_enum_variants_with_brackets,
    clippy::empty_structs_with_brackets,
    clippy::error_impl_error,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::exit,
    clippy::expect_used,
    clippy::filetype_is_file,
    clippy::float_cmp_const,
    clippy::fn_to_numeric_cast_any,
    clippy::format_push_string,
    clippy::get_unwrap,
    clippy::impl_trait_in_params,
    clippy::infinite_loop,
    clippy::integer_division,
    clippy::let_underscore_must_use,
    clippy::let_underscore_untyped,
    clippy::lossy_float_literal,
    clippy::map_err_ignore,
    clippy::min_ident_chars,
    clippy::missing_assert_message,
    clippy::missing_asserts_for_indexing,
    clippy::missing_inline_in_public_items,
    clippy::mixed_read_write_in_expression,
    clippy::multiple_inherent_impl,
    clippy::multiple_unsafe_ops_per_block,
    clippy::mutex_atomic,
    clippy::needless_raw_strings,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::partial_pub_fields,
    clippy::pattern_type_mismatch,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::pub_without_shorthand,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::redundant_type_annotations,
    clippy::ref_patterns,
    clippy::renamed_function_params,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::self_named_module_files,
    clippy::semicolon_outside_block,
    clippy::shadow_unrelated,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    clippy::str_to_string,
    clippy::string_add,
    clippy::string_lit_chars_any,
    clippy::string_slice,
    clippy::string_to_string,
    clippy::tests_outside_test_module,
    clippy::todo,
    clippy::try_err,
    clippy::undocumented_unsafe_blocks,
    clippy::unimplemented,
    clippy::unnecessary_safety_comment,
    clippy::unnecessary_safety_doc,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unseparated_literal_suffix,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::verbose_file_reads,
    clippy::wildcard_enum_match_arm
)]
#![warn(
    missing_abi,
    missing_docs,
    missing_copy_implementations,
    missing_crate_level_docs,
    missing_debug_implementations,
    missing_unsafe_on_extern
)]
#![warn(
    absolute_paths_not_starting_with_crate,
    deprecated_safe,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    keyword_idents_2024,
    macro_use_extern_crate,
    meta_variable_misuse,
    non_ascii_idents,
    non_local_definitions,
    redundant_lifetimes,
    single_use_lifetimes,
    trivial_numeric_casts,
    unit_bindings,
    unnameable_types,
    unreachable_pub
)]
#![allow(clippy::allow_attributes_without_reason)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod traits;

mod detect;
mod result;
mod endian;

mod ascii7;
mod utf8;
mod utf16;

pub use detect::*;
pub use traits::*;
pub use ascii7::{ASCII7, Ascii7Encoding};
pub use utf8::*;
pub use utf16::*;

#[cfg(feature = "utf-32")]
mod utf32;
#[cfg(feature = "utf-32")]
pub use utf32::*;

#[cfg(feature = "ascii7-compat")]
#[cfg_attr(docsrs, doc(cfg(feature = "ascii7-compat")))]
pub mod ascii7_compat;
#[cfg(feature = "iso-8859")]
#[cfg_attr(docsrs, doc(cfg(feature = "iso-8859")))]
pub mod iso_8859;
#[cfg(feature = "windows")]
#[cfg_attr(docsrs, doc(cfg(feature = "windows")))]
pub mod windows;

#[cfg(feature = "multi-byte")]
#[cfg_attr(docsrs, doc(cfg(feature = "multi-byte")))]
pub mod multi_byte;

#[cfg(feature = "single-byte")]
#[cfg_attr(docsrs, doc(cfg(feature = "single-byte")))]
pub mod single_byte;

mod decode_stream;
pub use decode_stream::*;

pub use result::DecodeResult;
pub use result::EncodeResult;
pub use endian::CharsetEndian;
