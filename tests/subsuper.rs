//! Ported from Python `unicodeit/tests/test_subsuper.py`.

use crate::common::ReplaceFunc;

mod common;

fn do_test_superscript_12(replace: ReplaceFunc) {
    let do_assert = assert_func!(replace);
    do_assert("a^{12}", "a¹²");
}

fn do_test_superscript_minus1(replace: ReplaceFunc) {
    let do_assert = assert_func!(replace);
    do_assert("cm^{-1}", "cm⁻¹")
}

fn do_test_subscript_12(replace: ReplaceFunc) {
    let do_assert = assert_func!(replace);
    do_assert("a_{12}", "a₁₂");
}

fn do_test_subscript_minus1(replace: ReplaceFunc) {
    let do_assert = assert_func!(replace);
    do_assert("cm_{-1}", "cm₋₁")
}

declare_tests!(
    superscript_12,
    superscript_minus1,
    subscript_12,
    subscript_minus1
);
