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

#[macro_export]
macro_rules! declare_tests {
    ($($name:ident),*) => {
        paste::paste! {
            $(
                #[test]
                fn [<test_ $name>]() {
                    [<do_test_ $name>](unicodeit::replace)
                }

                #[test]
                #[cfg_attr(not(feature = "naive-impl"), ignore)]
                fn [<test_naive_ $name>]() {
                    cfg_if::cfg_if! {
                        if #[cfg(feature = "naive-impl")] {
                            [<do_test_ $name>](unicodeit::replace_naive)
                        } else {
                            unreachable!("feature disabled")
                        }
                    }
                }
            )*
        }
    };
}
