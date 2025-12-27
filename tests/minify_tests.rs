//! Tests for Lua minification.

use rbxts_bundler::assets::DARKLUA_REL;
use rbxts_bundler::bundler::minify::minify;

mod success {
    use super::*;

    #[test]
    fn simple_function() {
        let source = r#"
local function hello()
    print("Hello, World!")
end
return hello
"#;
        let result = minify(source, DARKLUA_REL);
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn preserves_return() {
        let source = "local x = 1\nlocal y = 2\nlocal z = x + y\nreturn z";
        let result = minify(source, DARKLUA_REL).unwrap();
        assert!(result.contains("return"));
    }

    #[test]
    fn handles_comments() {
        let source = r#"
-- This is a comment
local function test()
    -- Another comment
    return 42
end
return test
"#;
        assert!(minify(source, DARKLUA_REL).is_ok());
    }

    #[test]
    fn handles_multiline_strings() {
        let source = r#"
local str = [[
This is a
multiline string
]]
return str
"#;
        assert!(minify(source, DARKLUA_REL).is_ok());
    }

    #[test]
    fn handles_empty_function() {
        let source = "local function empty() end\nreturn empty";
        assert!(minify(source, DARKLUA_REL).is_ok());
    }

    #[test]
    fn handles_nested_tables() {
        let source = r#"
local t = {
    a = 1,
    b = 2,
    c = { nested = true }
}
return t
"#;
        assert!(minify(source, DARKLUA_REL).is_ok());
    }
}

mod errors {
    use super::*;

    #[test]
    fn invalid_config() {
        let result = minify("local x = 1", "{ invalid json }");
        assert!(result.is_err());
    }
}
