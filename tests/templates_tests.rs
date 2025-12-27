//! Tests for embedded template assets.

use rbxts_bundler::assets::{
    DARKLUA_DEV, DARKLUA_DEV_COMPAT, DARKLUA_REL, DARKLUA_REL_COMPAT,
    FILE_HEADER, RUNTIME_HEADER, TREE_HEADER,
};

mod headers {
    use super::*;

    #[test]
    fn file_header_has_placeholders() {
        assert!(FILE_HEADER.contains("{{NAME}}"));
        assert!(FILE_HEADER.contains("{{VERSION}}"));
        assert!(FILE_HEADER.contains("{{INPUT}}"));
    }

    #[test]
    fn file_header_is_lua_comments() {
        for line in FILE_HEADER.lines() {
            assert!(line.starts_with("--"), "not a comment: {line}");
        }
    }

    #[test]
    fn runtime_header_is_lua_comments() {
        for line in RUNTIME_HEADER.lines() {
            assert!(line.starts_with("--"), "not a comment: {line}");
        }
    }

    #[test]
    fn tree_header_is_lua_comments() {
        for line in TREE_HEADER.lines() {
            assert!(line.starts_with("--"), "not a comment: {line}");
        }
    }
}

mod darklua_configs {
    use super::*;

    const CONFIGS: [(&str, &str); 4] = [
        ("DEV", DARKLUA_DEV),
        ("DEV_COMPAT", DARKLUA_DEV_COMPAT),
        ("REL", DARKLUA_REL),
        ("REL_COMPAT", DARKLUA_REL_COMPAT),
    ];

    #[test]
    fn valid_json() {
        for (name, config) in CONFIGS {
            let result: Result<serde_json::Value, _> = serde_json::from_str(config);
            assert!(result.is_ok(), "{name}: {:?}", result.err());
        }
    }

    #[test]
    fn has_rules_field() {
        for (name, config) in CONFIGS {
            let json: serde_json::Value = serde_json::from_str(config).unwrap();
            assert!(json.is_object(), "{name} not an object");
            assert!(json.get("rules").is_some(), "{name} missing rules");
        }
    }
}
