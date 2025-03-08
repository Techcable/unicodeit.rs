//! The core crate logic.

use aho_corasick::{AhoCorasick, MatchKind};
use std::sync::OnceLock;

fn replace_with<'a, M, R>(
    original_text: &'a str,
    potential_matches: impl Iterator<Item = M>,
    mut initial_match_index: impl FnMut(&M) -> usize,
    mut func: impl FnMut(&M, &'a str) -> Option<(usize, R)>,
    mut do_replace: impl FnMut(R, &mut String),
) -> String {
    let mut buffer = String::new();
    let mut last_index = 0usize; // last index of source text handled
    for potential_match in potential_matches {
        let match_index = initial_match_index(&potential_match);
        if match_index < last_index {
            continue;
        }
        let remaining_text = &original_text[match_index..];
        match func(&potential_match, remaining_text) {
            Some((matched_length, replacement)) => {
                buffer.push_str(&original_text[last_index..match_index]);
                do_replace(replacement, &mut buffer);
                last_index = match_index + matched_length;
            }
            None => {
                // just skip this match
            }
        }
    }
    buffer.push_str(&original_text[last_index..]);
    buffer
}

/// Replace the LaTeX characters with Unicode equivalents wherever possible.
pub fn replace(text: &str) -> String {
    // Catch cases like \not\subset and \not\in and convert them to
    // use the combining character slash as in \slash{\subset}
    let mut text: String = {
        // original code: re.sub(r'\\not(\\[A-z]+)', r'\\slash{\1}', f)
        const SEARCH_STR: &str = r#"\not\"#;
        replace_with(
            text,
            memchr::memmem::find_iter(text.as_bytes(), SEARCH_STR),
            |&index| index,
            |&_not_index, remaining_text| {
                debug_assert!(remaining_text.starts_with(SEARCH_STR));
                let possible_command_text = &remaining_text[SEARCH_STR.len()..];
                let command_len = possible_command_text
                    .find(|c: char| !c.is_ascii_alphabetic())
                    .unwrap_or(possible_command_text.len());
                if command_len > 0 {
                    Some((
                        command_len + SEARCH_STR.len(),
                        &possible_command_text[..command_len],
                    ))
                } else {
                    None
                }
            },
            |target_command, buffer| {
                buffer.push_str("\\slash{");
                buffer.push_str(target_command);
                buffer.push('}');
            },
        )
    };

    // escape combining marks with a space after the backslash
    text = {
        static COMBINING_MARKS_INITIAL_SEARCH: OnceLock<AhoCorasick> = OnceLock::new();
        replace_with(
            &text,
            COMBINING_MARKS_INITIAL_SEARCH
                .get_or_init(|| {
                    AhoCorasick::builder()
                        .match_kind(MatchKind::LeftmostFirst)
                        .build(
                            crate::data::COMBINING_MARKS
                                .iter()
                                .map(|(mark, _)| format!("{mark}{{")),
                        )
                        .unwrap()
                })
                .find_iter(&text),
            |m| m.start(),
            |m, _remaining_text| Some((m.len(), m.pattern().as_usize())),
            |mark_index, buffer| {
                buffer.push_str("\\ ");
                buffer.push_str(&crate::data::COMBINING_MARKS[mark_index].0[1..]);
                buffer.push('{');
            },
        )
    };

    // replace
    text = {
        static REPLACEMENTS_SEARCH: OnceLock<AhoCorasick> = OnceLock::new();

        replace_with(
            &text,
            REPLACEMENTS_SEARCH
                .get_or_init(|| {
                    AhoCorasick::builder()
                        .match_kind(MatchKind::LeftmostFirst)
                        .build(crate::data::REPLACEMENTS.iter().map(|(key, _)| key))
                        .unwrap()
                })
                .find_iter(&text),
            |m| m.start(),
            |m, _| Some((m.len(), m.pattern().as_usize())),
            |replacement_index, buffer| {
                buffer.push_str(crate::data::REPLACEMENTS[replacement_index].1);
            },
        )
    };

    // this is done in a second pass after primary replacements, unlike in python port
    // original comment: `check whether it was escaped for combining marks but has empty braces`
    text = {
        static REPLACEMENTS_WITH_BRACKET: OnceLock<AhoCorasick> = OnceLock::new();
        replace_with(
            &text,
            REPLACEMENTS_WITH_BRACKET
                .get_or_init(|| {
                    AhoCorasick::builder()
                        .match_kind(MatchKind::LeftmostFirst)
                        .build(
                            crate::data::REPLACEMENTS_WITH_BRACKET_SUFFIX
                                .iter()
                                .map(|&(key, _)| format!("\\ {}", &key[1..])),
                        )
                        .unwrap()
                })
                .find_iter(&text),
            |m| m.start(),
            |m, _remaining_text| Some((m.len(), m.pattern().as_usize())),
            |bracket_replacement_index, buffer| {
                buffer.push_str(
                    crate::data::REPLACEMENTS_WITH_BRACKET_SUFFIX[bracket_replacement_index].1,
                );
            },
        )
    };

    fn do_sub_super_group_expansion(
        text: &str,
        group_start: &str,
        is_target_char: impl Fn(char) -> bool,
    ) -> String {
        assert_eq!(group_start.len(), 2);
        assert!(group_start.ends_with("{"));
        let control_char = group_start.as_bytes()[0] as char;
        assert!(matches!(control_char, '^' | '_'));

        // original code takes f_{34a5} -> f_3_4_a_5
        replace_with(
            text,
            memchr::memmem::find_iter(text.as_bytes(), group_start),
            |&index| index,
            |&_start_index, remaining_text| {
                let end_index = memchr::memchr(b'}', remaining_text.as_bytes())?;
                let target_text = &remaining_text[2..end_index];
                if target_text.chars().all(&is_target_char) {
                    Some((target_text.len() + 3, target_text))
                } else {
                    None
                }
            },
            |target_text, buffer| {
                for c in target_text.chars() {
                    buffer.push(control_char);
                    buffer.push(c);
                }
            },
        )
    }

    // expand groups of subscripts: \_{01234}
    text = do_sub_super_group_expansion(&text, "_{", |c: char| {
        matches!(
            c,
            '0'..='9' | '\u{03B2}' | '\u{03B3}' | '\u{03C1}' | '\u{03C6}' | '\u{03C7}' | '\u{2212}'
        ) || (c.is_ascii() && memchr::memchr(c as u8, b"+-=()<>-aeoxjhklmnpstiruv").is_some())
    });

    // expand groups of superscripts: \^{01234}
    text = do_sub_super_group_expansion(&text, "^{", |c: char| {
        matches!(
            c,
            '0'..='9'
                | '\u{03B2}'
                | '\u{03B3}'
                | '\u{03B4}'
                | '\u{03C6}'
                | '\u{03C7}'
                | '\u{222B}'
                | '\u{2212}'
        ) || (c.is_ascii()
            && memchr::memchr(
                c as u8,
                b"+-=()<>ABDEGHIJKLMNOPRTUWabcdefghijklmnoprstuvwxyz",
            )
            .is_some())
    });

    // now replace subsuperscripts
    {
        static SUBSUPERSCRIPT_SEARCH: OnceLock<AhoCorasick> = OnceLock::new();

        text = replace_with(
            &text,
            SUBSUPERSCRIPT_SEARCH
                .get_or_init(|| {
                    AhoCorasick::builder()
                        .match_kind(MatchKind::LeftmostFirst)
                        .build(crate::data::SUB_SUPER_SCRIPTS.iter().map(|(key, _)| key))
                        .unwrap()
                })
                .find_iter(&text),
            |m| m.start(),
            |m, _| Some((m.len(), m.pattern().as_usize())),
            |replacement_index, buffer| {
                buffer.push_str(crate::data::SUB_SUPER_SCRIPTS[replacement_index].1);
            },
        )
    }

    // process combining marks first
    // differs from the other operations as we process all replacements of a single type
    // before moving on to the next one
    {
        assert_eq!(
            crate::data::COMBINING_MARKS.len(),
            crate::data::COMBINING_MARKS_ESCAPED_LATEX.len()
        );
        static COMBINING_MARKS_ESCAPED_LATEX_SEARCH: OnceLock<AhoCorasick> = OnceLock::new();
        let escaped_latex_search = COMBINING_MARKS_ESCAPED_LATEX_SEARCH.get_or_init(|| {
            AhoCorasick::builder()
                .match_kind(MatchKind::LeftmostFirst)
                .build(crate::data::COMBINING_MARKS_ESCAPED_LATEX.iter().copied())
                .unwrap()
        });
        let mut replace_buffer = String::with_capacity(16);
        while let Some(initial_find) = escaped_latex_search.find(&text) {
            let (original_command, result_text) =
                crate::data::COMBINING_MARKS[initial_find.pattern().as_usize()];
            let escaped_latex =
                crate::data::COMBINING_MARKS_ESCAPED_LATEX[initial_find.pattern().as_usize()];
            let finder = memchr::memmem::Finder::new(escaped_latex);
            while let Some(found_index) = finder.find(text.as_bytes()) {
                if text.len() <= found_index + escaped_latex.len() {
                    // incomplete: unescape and continue
                    text.truncate(found_index);
                    text.push_str(original_command);
                    text.push('{');
                    continue;
                }
                match text[found_index + escaped_latex.len()..].chars().next() {
                    None => {
                        // incomplete: unescape and continue
                        text.truncate(found_index);
                        text.push_str(original_command);
                        text.push('{');
                        continue;
                    }
                    Some(combined_char) => {
                        let char_offset = combined_char.len_utf8() + 1;
                        let resume_index =
                            (found_index + escaped_latex.len() + char_offset).min(text.len());
                        replace_buffer.clear();
                        replace_buffer.push(combined_char);
                        assert!(replace_buffer.len() + result_text.len() < 16);
                        replace_buffer.push_str(result_text);
                        text.replace_range(found_index..resume_index, &replace_buffer);
                    }
                }
            }
        }
    }

    text
}
