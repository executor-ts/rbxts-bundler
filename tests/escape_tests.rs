//! Tests for the Luau string escaping functionality.

use rbxts_bundler::bundler::escape::{append_luau_string, to_luau_string};

#[test]
fn test_simple_string() {
    let result = to_luau_string("hello");
    assert_eq!(result, "\"hello\"");
}

#[test]
fn test_empty_string() {
    let result = to_luau_string("");
    assert_eq!(result, "\"\"");
}

#[test]
fn test_escape_double_quote() {
    let result = to_luau_string("say \"hello\"");
    assert_eq!(result, "\"say \\\"hello\\\"\"");
}

#[test]
fn test_escape_backslash() {
    let result = to_luau_string("path\\to\\file");
    assert_eq!(result, "\"path\\\\to\\\\file\"");
}

#[test]
fn test_escape_newline() {
    let result = to_luau_string("line1\nline2");
    assert_eq!(result, "\"line1\\nline2\"");
}

#[test]
fn test_escape_tab() {
    let result = to_luau_string("col1\tcol2");
    assert_eq!(result, "\"col1\\tcol2\"");
}

#[test]
fn test_escape_carriage_return() {
    let result = to_luau_string("line1\rline2");
    assert_eq!(result, "\"line1\\nline2\"");
}

#[test]
fn test_escape_crlf() {
    let result = to_luau_string("line1\r\nline2");
    assert_eq!(result, "\"line1\\nline2\"");
}

#[test]
fn test_escape_bell() {
    let result = to_luau_string("alert\u{07}sound");
    assert_eq!(result, "\"alert\\asound\"");
}

#[test]
fn test_escape_backspace() {
    let result = to_luau_string("back\u{08}space");
    assert_eq!(result, "\"back\\bspace\"");
}

#[test]
fn test_escape_form_feed() {
    let result = to_luau_string("form\u{0C}feed");
    assert_eq!(result, "\"form\\ffeed\"");
}

#[test]
fn test_escape_vertical_tab() {
    let result = to_luau_string("vert\u{0B}tab");
    assert_eq!(result, "\"vert\\vtab\"");
}

#[test]
fn test_escape_null_byte() {
    let result = to_luau_string("null\x00byte");
    assert_eq!(result, "\"null\\x00byte\"");
}

#[test]
fn test_escape_control_char() {
    let result = to_luau_string("ctrl\x1Fchar");
    assert_eq!(result, "\"ctrl\\x1Fchar\"");
}

#[test]
fn test_escape_delete_char() {
    let result = to_luau_string("del\x7Fchar");
    assert_eq!(result, "\"del\\x7Fchar\"");
}

#[test]
fn test_escape_unicode() {
    let result = to_luau_string("emoji 🎉 here");
    assert_eq!(result, "\"emoji \\u{1F389} here\"");
}

#[test]
fn test_escape_unicode_chinese() {
    let result = to_luau_string("你好世界");
    assert_eq!(result, "\"\\u{4F60}\\u{597D}\\u{4E16}\\u{754C}\"");
}

#[test]
fn test_escape_mixed() {
    let result = to_luau_string("line1\nline2\t\"quoted\"\\ end");
    assert_eq!(result, "\"line1\\nline2\\t\\\"quoted\\\"\\\\ end\"");
}

#[test]
fn test_append_luau_string() {
    let mut buffer = String::from("prefix: ");
    append_luau_string("test", &mut buffer);
    assert_eq!(buffer, "prefix: \"test\"");
}

#[test]
fn test_append_preserves_existing() {
    let mut buffer = String::from("local x = ");
    append_luau_string("hello\nworld", &mut buffer);
    buffer.push_str("; return x");
    assert_eq!(buffer, "local x = \"hello\\nworld\"; return x");
}

#[test]
fn test_real_lua_source() {
    let source = r#"local function hello()
    print("Hello, World!")
end
return hello"#;
    let result = to_luau_string(source);
    assert!(result.starts_with('"'));
    assert!(result.ends_with('"'));
    assert!(result.contains("\\n"));
    assert!(result.contains("\\\""));
}
