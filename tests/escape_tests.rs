//! Tests for Luau string escaping.

use rbxts_bundler::bundler::escape::{append_luau_string, to_luau_string};

mod basic {
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
    fn no_special_chars() {
        assert_eq!(to_luau_string("abc123"), "\"abc123\"");
    }
}

mod sequences {
    use super::*;

    #[test]
    fn double_quotes() {
        assert_eq!(to_luau_string("say \"hello\""), "\"say \\\"hello\\\"\"");
    }

    #[test]
    fn backslashes() {
        assert_eq!(to_luau_string("path\\to\\file"), "\"path\\\\to\\\\file\"");
    }

    #[test]
    fn newlines() {
        assert_eq!(to_luau_string("line1\nline2"), "\"line1\\nline2\"");
    }

    #[test]
    fn tabs() {
        assert_eq!(to_luau_string("col1\tcol2"), "\"col1\\tcol2\"");
    }

    #[test]
    fn carriage_returns() {
        assert_eq!(to_luau_string("line1\rline2"), "\"line1\\nline2\"");
        assert_eq!(to_luau_string("line1\r\nline2"), "\"line1\\nline2\"");
    }
}

mod control_chars {
    use super::*;

    #[test]
    fn bell() {
        assert_eq!(to_luau_string("alert\u{07}sound"), "\"alert\\asound\"");
    }

    #[test]
    fn backspace() {
        assert_eq!(to_luau_string("back\u{08}space"), "\"back\\bspace\"");
    }

    #[test]
    fn form_feed() {
        assert_eq!(to_luau_string("form\u{0C}feed"), "\"form\\ffeed\"");
    }

    #[test]
    fn vertical_tab() {
        assert_eq!(to_luau_string("vert\u{0B}tab"), "\"vert\\vtab\"");
    }

    #[test]
    fn null_byte() {
        assert_eq!(to_luau_string("null\x00byte"), "\"null\\x00byte\"");
    }

    #[test]
    fn other() {
        assert_eq!(to_luau_string("ctrl\x1Fchar"), "\"ctrl\\x1Fchar\"");
    }

    #[test]
    fn delete() {
        assert_eq!(to_luau_string("del\x7Fchar"), "\"del\\x7Fchar\"");
    }
}

mod unicode {
    use super::*;

    #[test]
    fn emoji() {
        assert_eq!(to_luau_string("emoji 🎉 here"), "\"emoji \\u{1F389} here\"");
    }

    #[test]
    fn chinese() {
        assert_eq!(
            to_luau_string("你好世界"),
            "\"\\u{4F60}\\u{597D}\\u{4E16}\\u{754C}\""
        );
    }
}

mod complex {
    use super::*;

    #[test]
    fn mixed() {
        assert_eq!(
            to_luau_string("line1\nline2\t\"quoted\"\\ end"),
            "\"line1\\nline2\\t\\\"quoted\\\"\\\\ end\""
        );
    }

    #[test]
    fn lua_source() {
        let source = "local function hello()\n    print(\"Hello!\")\nend";
        let result = to_luau_string(source);

        assert!(result.starts_with('"'));
        assert!(result.ends_with('"'));
        assert!(result.contains("\\n"));
        assert!(result.contains("\\\""));
    }
}

mod append {
    use super::*;

    #[test]
    fn to_buffer() {
        let mut buffer = String::from("prefix: ");
        append_luau_string("test", &mut buffer);
        assert_eq!(buffer, "prefix: \"test\"");
    }

    #[test]
    fn preserves_existing() {
        let mut buffer = String::from("local x = ");
        append_luau_string("hello\nworld", &mut buffer);
        buffer.push_str("; return x");
        assert_eq!(buffer, "local x = \"hello\\nworld\"; return x");
    }
}
