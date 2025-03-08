//! Some simple tests, based on the typescript tests in `unicodeit/ts_src/tests.ts`

use crate::common::ReplaceFunc;

#[macro_use]
mod common;

fn do_test_basic(replace: ReplaceFunc) {
    let do_assert = assert_func!(replace);
    do_assert(r#"\epsilon"#, "\u{3b5}");
    do_assert(r#"\epsilon + \delta"#, "\u{3b5} + \u{3b4}");
    do_assert(r#"\alpha"#, "\u{3b1}");
}

fn do_test_combining(replace: ReplaceFunc) {
    let do_assert = assert_func!(replace);
    do_assert(r#"\dot{a}"#, "a\u{307}");
    do_assert(r#"\dot{\alpha}"#, "\u{3b1}\u{307}");
    do_assert(r#"\breve{}"#, "\u{2d8}");
    do_assert(r#"\breve{"#, r#"\breve{"#);
}

declare_tests!(basic, combining);
