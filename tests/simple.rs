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

fn do_test_past_bugs(replace: ReplaceFunc) {
    let do_assert = assert_func!(replace);
    // these were all found with `cargo fuzz`
    do_assert(r#"G_{}"#, "G_{}");
    do_assert(r#"_{(-,-}"#, "₍₋_,₋");
    do_assert(r#"_{(-,-}ZT'\\"#, r#"₍₋_,₋ZT'\\"#);
    do_assert(r#"^{zz}z^{zz}z\u{6}{}\u{6}{}}"#, r#"ᶻᶻzᶻᶻz\u{6}{}\u{6}{}}"#);
    do_assert(
        "-[\\ vac{\u{1}\0\0\0\0\0~\\ vec{a---{",
        "−[\\ vac{\u{1}\0\0\0\0\0~a⃗−−{",
    );
    do_assert(
        "-b\u{b}\\ hat{\\}{-\u{b}z\u{1a}\0\0\0{^(-^{(---;z(NBA}~--;",
        "−b\u{b}}\u{302}−\u{b}z\u{1a}\0\0\0{⁽−⁽⁻⁻⁻^;ᶻ⁽ᴺᴮᴬ~−−;",
    );
}

declare_tests!(basic, combining, past_bugs);
