//! A naive implementation of [`replace`],
//! ported directly form the Python code.
//!
//! This implementation is much less efficient than the other one.

use regex::Regex;
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
                let offset = usize::try_from(offset).unwrap();
                buffer.push_str(&text[..s.start() + offset]);
                buffer.push_str(&new_string);
                buffer.push_str(&text[s.end() + offset..]);
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
            if text.len() <= find_index + escaped_latex.len() {
                // incomplete: unescape and continue
                text.truncate(find_index);
                text.push_str(key);
                text.push('{');
                continue;
            }
            let combined_char = text[find_index + escaped_latex.len()..]
                .chars()
                .next()
                .unwrap();

            let char_offset = combined_char.len_utf8() + 1;
            let remainder = if text.len() >= find_index + escaped_latex.len() + char_offset {
                text[find_index + escaped_latex.len() + char_offset..].to_string()
            } else {
                String::new()
            };

            text.truncate(find_index);
            text.push(combined_char);
            text.push_str(val);
            text.push_str(&remainder);
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
