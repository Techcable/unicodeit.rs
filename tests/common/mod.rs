//! Common test code

pub type ReplaceFunc = fn(&str) -> String;

#[macro_export]
macro_rules! assert_func {
    ($replace:expr) => {
        |text: &str, expected: &str| {
            assert_eq!($replace(text), expected, "Failed to convert `{text}`");
        }
    };
}

pub(crate) fn replace_optimized_func() -> ReplaceFunc {
    cfg_if::cfg_if! {
        if #[cfg(feature = "optimized-impl")] {
            unicodeit::replace_optimized
        } else if #[cfg(feature = "prefer-optimized-impl")] {
            unicodeit::replace
        } else {
            unreachable!("disabled")
        }
    }
}

pub(crate) fn replace_naive_func() -> ReplaceFunc {
    cfg_if::cfg_if! {
        if #[cfg(feature = "naive-impl")] {
            unicodeit::replace_naive
        } else if #[cfg(not(feature = "prefer-optimized-impl"))] {
            unicodeit::replace
        } else {
            unreachable!("feature disabled")
        }
    }
}

#[macro_export]
macro_rules! declare_tests {
    ($($name:ident),*) => {
        paste::paste! {
            $(
                #[test]
                #[cfg_attr(not(any(
                    feature = "optimized-impl",
                    feature = "prefer-optimized-impl",
                )), ignore)]
                fn [<test_optimized_ $name>]() {
                    [<do_test_ $name>]($crate::common::replace_optimized_func())
                }

                #[test]
                #[cfg_attr(not(any(
                    feature = "naive-impl",
                    not(feature = "optimized-impl"),
                )), ignore)]
                fn [<test_naive_ $name>]() {
                    [<do_test_ $name>]($crate::common::replace_naive_func())
                }
            )*
        }
    };
}
