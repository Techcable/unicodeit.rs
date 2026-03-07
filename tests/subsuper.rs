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

fn do_test_multiple_subscript_groups(replace: ReplaceFunc) {
    let do_assert = assert_func!(replace);
    do_assert("a_{01}b_{23}", "a₀₁b₂₃");
}

fn do_test_multiple_superscript_groups(replace: ReplaceFunc) {
    let do_assert = assert_func!(replace);
    do_assert("a^{01}b^{23}", "a⁰¹b²³");
}

fn do_test_single_char_superscript_groups(replace: ReplaceFunc) {
    let do_assert = assert_func!(replace);
    do_assert("a^{b} c^{d}", "aᵇ cᵈ");
}

fn do_test_single_char_subscript_groups(replace: ReplaceFunc) {
    let do_assert = assert_func!(replace);
    do_assert("x_{i} y_{j}", "xᵢ yⱼ");
}

fn do_test_three_subscript_groups(replace: ReplaceFunc) {
    let do_assert = assert_func!(replace);
    do_assert("a_{1}b_{2}c_{3}", "a₁b₂c₃");
}

fn do_test_three_superscript_groups(replace: ReplaceFunc) {
    let do_assert = assert_func!(replace);
    do_assert("a^{1}b^{2}c^{3}", "a¹b²c³");
}

fn do_test_mixed_sub_and_super(replace: ReplaceFunc) {
    let do_assert = assert_func!(replace);
    do_assert("x_{i}^{2}", "xᵢ²");
}

fn do_test_subscript_with_operators(replace: ReplaceFunc) {
    let do_assert = assert_func!(replace);
    do_assert("a_{1+2}", "a₁₊₂");
}

fn do_test_superscript_with_operators(replace: ReplaceFunc) {
    let do_assert = assert_func!(replace);
    do_assert("a^{1+2}", "a¹⁺²");
}

fn do_test_subscript_with_parens(replace: ReplaceFunc) {
    let do_assert = assert_func!(replace);
    do_assert("a_{(1)}", "a₍₁₎");
}

fn do_test_long_subscript_group(replace: ReplaceFunc) {
    let do_assert = assert_func!(replace);
    do_assert("a_{0123456789}", "a₀₁₂₃₄₅₆₇₈₉");
}

fn do_test_adjacent_subscript_groups_no_separator(replace: ReplaceFunc) {
    let do_assert = assert_func!(replace);
    do_assert("_{01}_{23}_{45}", "₀₁₂₃₄₅");
}

fn do_test_subscript_group_surrounded_by_text(replace: ReplaceFunc) {
    let do_assert = assert_func!(replace);
    do_assert("hello _{12} world _{34} end", "hello ₁₂ world ₃₄ end");
}

declare_tests!(
    superscript_12,
    superscript_minus1,
    subscript_12,
    subscript_minus1,
    multiple_subscript_groups,
    multiple_superscript_groups,
    single_char_superscript_groups,
    single_char_subscript_groups,
    three_subscript_groups,
    three_superscript_groups,
    mixed_sub_and_super,
    subscript_with_operators,
    superscript_with_operators,
    subscript_with_parens,
    long_subscript_group,
    adjacent_subscript_groups_no_separator,
    subscript_group_surrounded_by_text
);
