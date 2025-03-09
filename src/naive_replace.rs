//! A naive implementation of [`replace`],
//! ported directly form the Python code.
//!
//! This implementation is much less efficient than the other one.

use regex::Regex;
use std::ops::Range;
use std::sync::LazyLock;

/// A naive implementation of the [`crate::replace`] function,
/// which more directly matches the Python code.
///
/// The behavior of this function should exactly match the behavior
/// of the original library, but requires the `regex` crate to opperate.
/// In addition to the cost of using `regex`,
/// each invocation requires several thousand reallocations of the input string.
pub fn replace(text: &str) -> String {
    // Catch cases like \not\subset and \not\in and convert them to
    // use the combining character slash as in \slash{\subset}
    let mut text = {
        static REGEX: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r##"\\not(\\[A-z]+)"##).unwrap());
        REGEX.replace_all(text, r#"\slash{$1}""#).into_owned()
    };
    // escape combining marks with a space after the backslash
    let mut scratch_buffer = String::new();
    for &(key, _val) in crate::data::COMBINING_MARKS {
        scratch_buffer.clear();
        scratch_buffer.push_str(key);
        scratch_buffer.push('{');
        if contains(&text, &scratch_buffer) {
            text = text.replace(&scratch_buffer, &format!("\\ {}{{", &key[1..]));
        }
    }

    // replace
    for &(key, val) in crate::data::REPLACEMENTS {
        if contains(&text, key) {
            text = text.replace(key, val);
        }

        // check whether it was escaped for combining marks but has empty braces
        if key.ends_with("{}") {
            scratch_buffer.clear();
            scratch_buffer.push_str("\\ ");
            scratch_buffer.push_str(&key[1..]);
            if contains(&text, &scratch_buffer) {
                text = text.replace(&scratch_buffer, val);
            }
        }
    }

    fn do_sub_or_super_expand(
        find_regex: &Regex,
        sub_regex: &Regex,
        orig_text: &str,
        replace_char: char,
    ) -> String {
        /// Emulate python signed slice logic
        fn signed_slice(s: &str, r: Range<isize>) -> &str {
            let signed_index = |i: isize| {
                if i >= 0 {
                    Some(i as usize)
                } else {
                    // overflow should never really happen here
                    let res = (s.len() as isize).checked_add(i).unwrap();
                    if res >= 0 {
                        Some(res as usize)
                    } else {
                        None // underflow
                    }
                }
            };
            let start = signed_index(r.start);
            let end = signed_index(r.end);
            let real_index = match (start, end) {
                // underflow
                (None, Some(end)) => 0..end.max(r.len()),
                (Some(start), Some(end)) if start <= end => start..end,
                // end underflow || start > end => empty slice
                (Some(_), None) | (None, None) | (Some(_), Some(_)) => 0..0,
            };
            &s[real_index]
        }
        assert!(matches!(replace_char, '^' | '_'));
        let mut offset = 0isize;
        let mut text = orig_text.to_string();
        for s in find_regex.find_iter(orig_text) {
            let mut count = 0usize;
            let target_text = s.as_str();
            let new_string = sub_regex.replace_all(
                &target_text[2..target_text.len() - 1],
                |c: &regex::Captures| {
                    count += 1;
                    format!("{replace_char}{}", &c[0])
                },
            );
            // f = f[:s.start() + offset] + newstring + f[s.end() + offset:]
            let mut buffer = String::with_capacity(text.len() + new_string.len());
            {
                buffer.push_str(signed_slice(&text, 0..(s.start() as isize + offset)));
                buffer.push_str(&new_string);
                buffer.push_str(signed_slice(
                    &text,
                    (s.end() as isize + offset)..(text.len() as isize),
                ));
            }
            text = buffer;
            let count = isize::try_from(count).unwrap();
            offset += (count * 2) - (count + 3);
        }
        text
    }
    // expand groups of subscripts: \_{01234}
    {
        static REGEX_FIND: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(
            r#"_\{[0-9\+-=\(\)<>\-aeoxjhklmnpstiruv\u{03B2}\u{03B3}\u{03C1}\u{03C6}\u{03C7}\u{2212}]+\}"#
        ).unwrap()
        });
        static REGEX_SUB: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(
            r#"([0-9\+-=\(\)<>\-aeoxjhklmnpstiruv\u{03B2}\u{03B3}\u{03C1}\u{03C6}\u{03C7}\u{2212}])"#
        ).unwrap()
        });
        text = do_sub_or_super_expand(&REGEX_FIND, &REGEX_SUB, &text, '_');
    }

    // expand groups of superscripts: \^{01234}
    {
        static REGEX_FIND: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(concat!(
                r#"\^\{[0-9\+-=\(\)<>ABDEGHIJKLMNOPRTUWabcdefghijklmnoprstuvwxyz"#,
                r#"\u{3B2}\u{3B3}\u{3B4}\u{3C6}\u{3C7}\u{222B}\u{2212}]+\}"#,
            ))
            .unwrap()
        });
        static REGEX_SUB: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(concat!(
                r#"([0-9\+-=\(\)<>ABDEGHIJKLMNOPRTUWabcdefghijklmnoprstuvwxyz"#,
                r#"\u{3B2}\u{3B3}\u{3B4}\u{3C6}\u{3C7}\u{222B}\u{2212}])"#,
            ))
            .unwrap()
        });
        text = do_sub_or_super_expand(&REGEX_FIND, &REGEX_SUB, &text, '^');
    }

    // now replace subsuperscripts
    for &(key, val) in crate::data::SUB_SUPER_SCRIPTS {
        if contains(&text, key) {
            text = text.replace(key, val);
        }
    }

    // process combining marks first
    for &(key, val) in crate::data::COMBINING_MARKS {
        scratch_buffer.clear();
        scratch_buffer.push_str("\\ ");
        scratch_buffer.push_str(&key[1..]);
        scratch_buffer.push('{');
        let escaped_latex = &scratch_buffer;
        while let Some(find_index) = find(&text, escaped_latex) {
            let after_find = &text[find_index + escaped_latex.len()..];
            let mut chars = after_find.chars();
            let Some(combined_char) = chars.next() else {
                // incomplete: unescape and continue
                text.truncate(find_index);
                text.push_str(key);
                text.push('{');
                continue;
            };

            let char_offset = if chars.next().is_some() {
                after_find.len() - chars.as_str().len()
            } else {
                after_find.len()
            };
            text.replace_range(
                find_index..(find_index + escaped_latex.len() + char_offset),
                &format!("{combined_char}{val}"),
            );
        }
    }

    text
}

// optimized string search

#[inline]
fn find(haystack: &str, needle: &str) -> Option<usize> {
    // surprisingly this is a massive 300% performance improvement over using the stdlib,
    memchr::memmem::find(haystack.as_bytes(), needle.as_bytes())
}

#[inline]
fn contains(haystack: &str, needle: &str) -> bool {
    find(haystack, needle).is_some()
}
