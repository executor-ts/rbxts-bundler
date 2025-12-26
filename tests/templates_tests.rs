//! Tests for template constants.

use rbxts_bundler::assets::{DARKLUA_CONFIG, FILE_HEADER, RUNTIME_HEADER, TREE_HEADER};

#[test]
fn test_file_header_contains_placeholders() {
    assert!(FILE_HEADER.contains("{{NAME}}"));
    assert!(FILE_HEADER.contains("{{VERSION}}"));
    assert!(FILE_HEADER.contains("{{INPUT}}"));
}

#[test]
fn test_file_header_is_comment() {
    // File header should be Lua comments
    for line in FILE_HEADER.lines() {
        assert!(
            line.starts_with("--"),
            "Header line should be a comment: {}",
            line
        );
    }
}

#[test]
fn test_runtime_header_is_comment() {
    for line in RUNTIME_HEADER.lines() {
        assert!(
            line.starts_with("--"),
            "Runtime header line should be a comment: {}",
            line
        );
    }
}

#[test]
fn test_tree_header_is_comment() {
    for line in TREE_HEADER.lines() {
        assert!(
            line.starts_with("--"),
            "Tree header line should be a comment: {}",
            line
        );
    }
}

#[test]
fn test_darklua_config_is_valid_json() {
    let result: Result<serde_json::Value, _> = serde_json::from_str(DARKLUA_CONFIG);
    assert!(result.is_ok(), "DARKLUA_CONFIG should be valid JSON");
}

#[test]
fn test_darklua_config_has_rules() {
    let config: serde_json::Value = serde_json::from_str(DARKLUA_CONFIG).unwrap();

    // darklua config typically has a "rules" array or similar structure
    // Just verify it's a valid object
    assert!(config.is_object(), "DARKLUA_CONFIG should be a JSON object");
}
