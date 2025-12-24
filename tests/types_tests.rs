//! Tests for the BundlerContext type.

use std::path::Path;
use rbxts_bundler::bundler::types::BundlerContext;
use rbxts_bundler::cli::Mode;

#[test]
fn test_context_new_development() {
    let path = Path::new("/test/input.rbxm");
    let ctx = BundlerContext::new(Mode::Development, path);
    assert_eq!(ctx.mode, Mode::Development);
    assert_eq!(ctx.input_path, path);
}

#[test]
fn test_context_new_production() {
    let path = Path::new("/test/input.rbxm");
    let ctx = BundlerContext::new(Mode::Production, path);
    assert_eq!(ctx.mode, Mode::Production);
    assert_eq!(ctx.input_path, path);
}

#[test]
fn test_apply_templates_name() {
    let path = Path::new("/test/input.rbxm");
    let ctx = BundlerContext::new(Mode::Development, path);
    let result = ctx.apply_templates("Built with {{NAME}}");
    assert!(result.contains("rbxts-bundler"));
    assert!(!result.contains("{{NAME}}"));
}

#[test]
fn test_apply_templates_version() {
    let path = Path::new("/test/input.rbxm");
    let ctx = BundlerContext::new(Mode::Development, path);
    let result = ctx.apply_templates("Version: {{VERSION}}");
    // Version should be replaced with actual version
    assert!(!result.contains("{{VERSION}}"));
    // Should contain some version pattern (digits and dots)
    assert!(result.chars().any(|c| c.is_ascii_digit()));
}

#[test]
fn test_apply_templates_input() {
    let path = Path::new("/my/project/input.rbxm");
    let ctx = BundlerContext::new(Mode::Development, path);
    let result = ctx.apply_templates("Source: {{INPUT}}");
    assert!(result.contains("input.rbxm"));
    assert!(!result.contains("{{INPUT}}"));
}

#[test]
fn test_apply_templates_all_placeholders() {
    let path = Path::new("/test/file.rbxm");
    let ctx = BundlerContext::new(Mode::Production, path);
    let template = "-- {{NAME}} v{{VERSION}}\n-- Source: {{INPUT}}";
    let result = ctx.apply_templates(template);
    
    assert!(!result.contains("{{NAME}}"));
    assert!(!result.contains("{{VERSION}}"));
    assert!(!result.contains("{{INPUT}}"));
    assert!(result.contains("rbxts-bundler"));
    assert!(result.contains("file.rbxm"));
}

#[test]
fn test_apply_templates_no_placeholders() {
    let path = Path::new("/test/input.rbxm");
    let ctx = BundlerContext::new(Mode::Development, path);
    let content = "This has no placeholders";
    let result = ctx.apply_templates(content);
    assert_eq!(result, content);
}

#[test]
fn test_apply_templates_repeated_placeholders() {
    let path = Path::new("/test/input.rbxm");
    let ctx = BundlerContext::new(Mode::Development, path);
    let template = "{{NAME}} - {{NAME}} - {{NAME}}";
    let result = ctx.apply_templates(template);
    
    // All occurrences should be replaced
    assert!(!result.contains("{{NAME}}"));
    assert_eq!(result.matches("rbxts-bundler").count(), 3);
}
