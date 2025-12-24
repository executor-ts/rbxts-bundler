use std::fmt::Write;

const HEX_DIGITS: &[u8; 16] = b"0123456789ABCDEF";

/// Converts Luau source text into a Luau-compatible quoted string literal.
pub fn to_luau_string(source: &str) -> String {
    let mut out = String::with_capacity(source.len() + (source.len() >> 3) + 2);
    append_luau_string(source, &mut out);
    out
}

/// Appends a Luau-compatible quoted string literal to the provided buffer.
pub fn append_luau_string(source: &str, out: &mut String) {
    out.push('"');

    let mut start = 0;
    let mut chars = source.char_indices().peekable();

    while let Some((i, ch)) = chars.next() {
        match ch {
            '"' => {
                out.push_str(&source[start..i]);
                out.push_str("\\\"");
                start = i + 1;
            }
            '\\' => {
                out.push_str(&source[start..i]);
                out.push_str("\\\\");
                start = i + 1;
            }
            '\n' => {
                out.push_str(&source[start..i]);
                out.push_str("\\n");
                start = i + 1;
            }
            '\t' => {
                out.push_str(&source[start..i]);
                out.push_str("\\t");
                start = i + 1;
            }
            '\u{07}' => {
                out.push_str(&source[start..i]);
                out.push_str("\\a");
                start = i + 1;
            }
            '\u{08}' => {
                out.push_str(&source[start..i]);
                out.push_str("\\b");
                start = i + 1;
            }
            '\u{0C}' => {
                out.push_str(&source[start..i]);
                out.push_str("\\f");
                start = i + 1;
            }
            '\u{0B}' => {
                out.push_str(&source[start..i]);
                out.push_str("\\v");
                start = i + 1;
            }
            '\r' => {
                out.push_str(&source[start..i]);
                out.push_str("\\n");
                if let Some(&(_, '\n')) = chars.peek() {
                    chars.next();
                    start = i + 2;
                } else {
                    start = i + 1;
                }
            }
            c if c.is_ascii() && (c < ' ' || c == '\x7F') => {
                out.push_str(&source[start..i]);
                let b = c as u8;
                out.push_str("\\x");
                out.push(HEX_DIGITS[(b >> 4) as usize] as char);
                out.push(HEX_DIGITS[(b & 0xF) as usize] as char);
                start = i + 1;
            }
            c if !c.is_ascii() => {
                out.push_str(&source[start..i]);
                write!(out, "\\u{{{:X}}}", c as u32).unwrap();
                start = i + c.len_utf8();
            }
            _ => {}
        }
    }

    out.push_str(&source[start..]);
    out.push('"');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_string() {
        assert_eq!(to_luau_string("hello"), "\"hello\"");
    }

    #[test]
    fn empty_string() {
        assert_eq!(to_luau_string(""), "\"\"");
    }

    #[test]
    fn escape_quotes() {
        assert_eq!(to_luau_string("say \"hi\""), "\"say \\\"hi\\\"\"");
    }

    #[test]
    fn escape_backslash() {
        assert_eq!(to_luau_string("a\\b"), "\"a\\\\b\"");
    }

    #[test]
    fn escape_newline() {
        assert_eq!(to_luau_string("a\nb"), "\"a\\nb\"");
    }

    #[test]
    fn escape_tab() {
        assert_eq!(to_luau_string("a\tb"), "\"a\\tb\"");
    }

    #[test]
    fn escape_crlf_combined() {
        assert_eq!(to_luau_string("a\r\nb"), "\"a\\nb\"");
    }

    #[test]
    fn escape_unicode() {
        assert_eq!(to_luau_string("🎉"), "\"\\u{1F389}\"");
    }

    #[test]
    fn escape_control_chars() {
        assert_eq!(to_luau_string("\x00"), "\"\\x00\"");
        assert_eq!(to_luau_string("\x1F"), "\"\\x1F\"");
    }
}
