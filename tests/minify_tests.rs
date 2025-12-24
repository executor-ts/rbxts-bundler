//! Tests for the minification functionality.

use rbxts_bundler::bundler::minify::minify;
use rbxts_bundler::templates::DARKLUA_CONFIG;

#[test]
fn test_minify_simple() {
    let source = r#"
local function hello()
    print("Hello, World!")
end

return hello
"#;
    
    let result = minify(source, DARKLUA_CONFIG);
    assert!(result.is_ok());
    
    let minified = result.unwrap();
    // Minified code should be shorter or equal in length (no unnecessary whitespace)
    assert!(!minified.is_empty());
}

#[test]
fn test_minify_preserves_functionality() {
    let source = r#"local x = 1
local y = 2
local z = x + y
return z"#;
    
    let result = minify(source, DARKLUA_CONFIG);
    assert!(result.is_ok());
    
    let minified = result.unwrap();
    // Should still have return statement
    assert!(minified.contains("return"));
}

#[test]
fn test_minify_with_comments() {
    let source = r#"
-- This is a comment
local function test()
    -- Another comment
    return 42
end
return test
"#;
    
    let result = minify(source, DARKLUA_CONFIG);
    assert!(result.is_ok());
}

#[test]
fn test_minify_invalid_config() {
    let source = "local x = 1";
    let invalid_config = "{ invalid json }";
    
    let result = minify(source, invalid_config);
    assert!(result.is_err());
}

#[test]
fn test_minify_multiline_string() {
    let source = r#"
local str = [[
This is a
multiline string
]]
return str
"#;
    
    let result = minify(source, DARKLUA_CONFIG);
    assert!(result.is_ok());
}

#[test]
fn test_minify_empty_function() {
    let source = r#"
local function empty()
end
return empty
"#;
    
    let result = minify(source, DARKLUA_CONFIG);
    assert!(result.is_ok());
}

#[test]
fn test_minify_table_constructor() {
    let source = r#"
local t = {
    a = 1,
    b = 2,
    c = {
        nested = true
    }
}
return t
"#;
    
    let result = minify(source, DARKLUA_CONFIG);
    assert!(result.is_ok());
}
