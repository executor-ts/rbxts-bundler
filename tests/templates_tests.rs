//! Tests for template constants.

use rbxts_bundler::assets::{
    DARKLUA_DEV, DARKLUA_DEV_COMPAT, DARKLUA_PROD, DARKLUA_PROD_COMPAT, FILE_HEADER,
    RUNTIME_HEADER, TREE_HEADER,
};

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
fn test_darklua_dev_is_valid_json() {
    let result: Result<serde_json::Value, _> = serde_json::from_str(DARKLUA_DEV);
    assert!(result.is_ok(), "DARKLUA_DEV should be valid JSON");
}

#[test]
fn test_darklua_dev_compat_is_valid_json() {
    let result: Result<serde_json::Value, _> = serde_json::from_str(DARKLUA_DEV_COMPAT);
    assert!(result.is_ok(), "DARKLUA_DEV_COMPAT should be valid JSON");
}

#[test]
fn test_darklua_prod_is_valid_json() {
    let result: Result<serde_json::Value, _> = serde_json::from_str(DARKLUA_PROD);
    assert!(result.is_ok(), "DARKLUA_PROD should be valid JSON");
}

#[test]
fn test_darklua_prod_compat_is_valid_json() {
    let result: Result<serde_json::Value, _> = serde_json::from_str(DARKLUA_PROD_COMPAT);
    assert!(result.is_ok(), "DARKLUA_PROD_COMPAT should be valid JSON");
}

#[test]
fn test_darklua_configs_have_rules() {
    for (name, config_str) in [
        ("DARKLUA_DEV", DARKLUA_DEV),
        ("DARKLUA_DEV_COMPAT", DARKLUA_DEV_COMPAT),
        ("DARKLUA_PROD", DARKLUA_PROD),
        ("DARKLUA_PROD_COMPAT", DARKLUA_PROD_COMPAT),
    ] {
        let config: serde_json::Value = serde_json::from_str(config_str)
            .unwrap_or_else(|_| panic!("{} should be valid JSON", name));
        assert!(config.is_object(), "{} should be a JSON object", name);
        assert!(
            config.get("rules").is_some(),
            "{} should have a rules field",
            name
        );
    }
}
