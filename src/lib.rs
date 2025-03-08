#![deny(missing_docs)]
//! Converts latex to Unicode characters.
//!
//! Port of [unicodeit.net](https://www.unicodeit.net) to rust.

pub(crate) mod data;
#[cfg(test)]
mod data_test;
#[cfg(feature = "naive-impl")]
mod naive_replace;
mod replace;

/// Describe the version of the data used in the crate
pub fn version_info() -> String {
    format!(
        "{}-data{}",
        env!("CARGO_PKG_VERSION"),
        data::UNICODEIT_VERSION
    )
}

pub use replace::replace;

#[cfg(feature = "naive-impl")]
pub use naive_replace::replace as replace_naive;
