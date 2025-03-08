//! Ported from Python `unicodeit/tests/test_data.py`.
//!
//! Can't be an integration test, because `crate::data` is private.

#[cfg(not(test))]
compile_error!("only for testing");

use std::collections::HashMap;

macro_rules! test_ordered {
    ($($name:ident => $table:ident),*) => {
        $(#[test]
        fn $name() {
            let mut expr_length = usize::MAX;
            for (l, _) in crate::data::$table {
                // Using byte length breaks things so we use character length
                // Does this affect things
                let char_length = l.chars().count();
                assert!(char_length <= expr_length);
                expr_length = char_length;
            }
        })*
    };
}

test_ordered! {
    order_replacements => REPLACEMENTS,
    order_sub_super_scripts => SUB_SUPER_SCRIPTS,
    order_combining_marks => COMBINING_MARKS
}

#[test]
fn combining_not_in_replacement() {
    let replacement_latex = crate::data::REPLACEMENTS
        .iter()
        .map(|&(l, u)| (l.replace("{}", ""), u))
        .collect::<HashMap<String, &'static str>>();
    for &(l, u) in crate::data::COMBINING_MARKS {
        let l = l.replace("{}", "");
        let Some(&replacement_val) = replacement_latex.get(&l) else {
            continue;
        };

        // if the same command is in "replacements",
        // it must not be the combining mark
        assert_ne!(replacement_val, u);
    }
}

#[test]
fn incomplete_combining_mark() {
    // TODO: This is also in the 'basic' test, does it need to be here?
    assert_eq!(crate::replace("\\breve{"), "\\breve{");
    #[cfg(feature = "naive-impl")]
    {
        assert_eq!(crate::replace_naive("\\breve{"), "\\breve{");
    }
}
