use std::fmt::Write;

const HEX_DIGITS: &[u8; 16] = b"0123456789ABCDEF";

/// Lookup table: 0 = passthrough, 1+ = needs escape handling
/// This avoids branch mispredictions for the common ASCII passthrough case.
static ESCAPE_LUT: [u8; 128] = {
    let mut lut = [0u8; 128];
    // Control characters 0x00-0x1F
    let mut i = 0;
    while i < 32 {
        lut[i] = 1;
        i += 1;
    }
    lut[0x7F] = 1; // DEL
    lut[b'"' as usize] = 2;
    lut[b'\\' as usize] = 3;
    lut[b'\n' as usize] = 4;
    lut[b'\t' as usize] = 5;
    lut[0x07] = 6; // \a
    lut[0x08] = 7; // \b
    lut[0x0C] = 8; // \f
    lut[0x0B] = 9; // \v
    lut[b'\r' as usize] = 10;
    lut
};

/// Converts Luau source text into a Luau-compatible quoted string literal.
#[must_use]
pub fn to_luau_string(source: &str) -> String {
    let mut out = String::with_capacity(source.len() + (source.len() >> 3) + 2);
    append_luau_string(source, &mut out);
    out
}

/// Appends a Luau-compatible quoted string literal to the provided buffer.
pub fn append_luau_string(source: &str, out: &mut String) {
    out.push('"');

    let bytes = source.as_bytes();
    let mut start = 0;
    let mut i = 0;

    while i < bytes.len() {
        let b = bytes[i];

        if b < 128 {
            // ASCII fast path with lookup table
            let escape_kind = ESCAPE_LUT[b as usize];
            if escape_kind == 0 {
                // Passthrough - just advance
                i += 1;
                continue;
            }

            // Flush pending unescaped bytes
            if start < i {
                out.push_str(&source[start..i]);
            }

            match escape_kind {
                2 => out.push_str("\\\""),
                3 => out.push_str("\\\\"),
                4 => out.push_str("\\n"),
                5 => out.push_str("\\t"),
                6 => out.push_str("\\a"),
                7 => out.push_str("\\b"),
                8 => out.push_str("\\f"),
                9 => out.push_str("\\v"),
                10 => {
                    // \r - convert to \n, handle \r\n
                    out.push_str("\\n");
                    if i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
                        i += 1; // skip the \n in \r\n
                    }
                }
                _ => {
                    // Other control characters: \xHH
                    out.push_str("\\x");
                    out.push(HEX_DIGITS[(b >> 4) as usize] as char);
                    out.push(HEX_DIGITS[(b & 0xF) as usize] as char);
                }
            }

            i += 1;
            start = i;
        } else {
            // Non-ASCII: decode UTF-8 char and emit \u{XXXX}
            // Safety: source is valid UTF-8
            let ch = source[i..].chars().next().unwrap();
            let char_len = ch.len_utf8();

            if start < i {
                out.push_str(&source[start..i]);
            }

            write!(out, "\\u{{{:X}}}", ch as u32).unwrap();

            i += char_len;
            start = i;
        }
    }

    // Flush remaining
    if start < bytes.len() {
        out.push_str(&source[start..]);
    }

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
