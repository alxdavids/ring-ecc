#![allow(
    missing_copy_implementations,
    missing_debug_implementations,
    non_camel_case_types,
    non_snake_case,
    unsafe_code
)]
#![deny(
    unstable_features,
    unused_qualifications,
    variant_size_differences,
)]
#![forbid(
    anonymous_parameters,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_results,
)]
#![cfg_attr(feature = "internal_benches", allow(unstable_features), feature(test))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
mod test;

#[macro_use]
pub mod arithmetic;

#[macro_use]
mod polyfill;

mod bits;
mod c;
mod io;
mod cpu;

pub mod ec;
mod endian;
pub mod error;
pub mod limb;
pub mod rand;

mod sealed {
    /// Traits that are designed to only be implemented internally in *ring*.
    //
    // Usage:
    // ```
    // use crate::sealed;
    //
    // pub trait MyType: sealed::Sealed {
    //     // [...]
    // }
    //
    // impl sealed::Sealed for MyType {}
    // ```
    pub trait Sealed {}
}
