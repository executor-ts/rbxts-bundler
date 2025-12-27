//! Tests for `BundlerContext`.

use rbxts_bundler::bundler::types::BundlerContext;
use rbxts_bundler::bundler::Mode;
use std::path::Path;

fn ctx(path: &str) -> BundlerContext<'_> {
    BundlerContext::new(Mode::Development, Path::new(path))
}

mod creation {
    use super::*;

    #[test]
    fn stores_mode_and_path() {
        let path = Path::new("/test/input.rbxm");

        let dev = BundlerContext::new(Mode::Development, path);
        assert_eq!(dev.mode, Mode::Development);
        assert_eq!(dev.input_path, path);

        let prod = BundlerContext::new(Mode::Production, path);
        assert_eq!(prod.mode, Mode::Production);
        assert_eq!(prod.input_path, path);
    }
}

mod templates {
    use super::*;

    #[test]
    fn replaces_name() {
        let result = ctx("/test/input.rbxm").apply_templates("{{NAME}}");
        assert!(result.contains("rbxts-bundler"));
        assert!(!result.contains("{{NAME}}"));
    }

    #[test]
    fn replaces_version() {
        let result = ctx("/test/input.rbxm").apply_templates("{{VERSION}}");
        assert!(!result.contains("{{VERSION}}"));
        assert!(result.chars().any(|c| c.is_ascii_digit()));
    }

    #[test]
    fn replaces_input() {
        let result = ctx("/my/project/input.rbxm").apply_templates("{{INPUT}}");
        assert!(result.contains("input.rbxm"));
        assert!(!result.contains("{{INPUT}}"));
    }

    #[test]
    fn replaces_all_placeholders() {
        let ctx = BundlerContext::new(Mode::Production, Path::new("/test/file.rbxm"));
        let result = ctx.apply_templates("{{NAME}} v{{VERSION}} - {{INPUT}}");

        assert!(!result.contains("{{"));
        assert!(result.contains("rbxts-bundler"));
        assert!(result.contains("file.rbxm"));
    }

    #[test]
    fn preserves_text_without_placeholders() {
        let content = "no placeholders here";
        assert_eq!(ctx("/test/input.rbxm").apply_templates(content), content);
    }

    #[test]
    fn replaces_repeated_placeholders() {
        let result = ctx("/test/input.rbxm").apply_templates("{{NAME}} {{NAME}} {{NAME}}");

        assert!(!result.contains("{{NAME}}"));
        assert_eq!(result.matches("rbxts-bundler").count(), 3);
    }
}
