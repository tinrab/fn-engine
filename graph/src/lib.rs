#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![warn(
    missing_docs,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unsafe_code,
    unreachable_pub
)]

//! Utility for modelling graph computation.

pub mod error;
pub mod graph;
pub mod schema;
pub mod value;
