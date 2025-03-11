#![deny(missing_docs)]
//! Converts latex to Unicode characters.
//!
//! Port of [unicodeit.net](https://www.unicodeit.net) to rust.

pub(crate) mod data;
#[cfg(test)]
mod data_test;
#[cfg(any(feature = "naive-impl", not(feature = "prefer-optimized-impl")))]
mod naive_replace;
#[cfg(any(feature = "optimized-impl", feature = "prefer-optimized-impl"))]
mod optimized_replace;

/// Describe the version of the data used in the crate.
///
/// The format of this string is implementation-specific,
/// and may change at any time.
pub fn version_info() -> String {
    format!(
        "unicodeit.rs{}-data{}",
        env!("CARGO_PKG_VERSION"),
        data::UNICODEIT_VERSION
    )
}

/// Replace the LaTeX characters with Unicode equivalents wherever possible.
///
/// This function is a port of the [unicodeit](https://www.unicodeit.net) library to rust,
/// which tries to exactly mimic the behavior of the original library.
#[inline]
pub fn replace(text: &str) -> String {
    cfg_if::cfg_if! {
        if #[cfg(feature = "prefer-optimized-impl")] {
            naive_replace::replace(text)
        } else {
            naive_replace::replace(text)
        }
    }
}

#[cfg(feature = "naive-impl")]
pub use naive_replace::replace as replace_naive;

#[cfg(feature = "optimized-impl")]
pub use optimized_replace::replace as replace_optimized;
