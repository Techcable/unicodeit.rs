#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    assert_eq!(
        unicodeit::replace_optimized(data),
        unicodeit::replace_naive(data),
        "Bad output for data {data:?}"
    );
});
