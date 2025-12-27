//! Tests for the `Mode` enum.

use rbxts_bundler::bundler::Mode;

mod display {
    use super::*;

    #[test]
    fn development_displays_as_debug() {
        assert_eq!(Mode::Development.to_string(), "DEBUG");
    }

    #[test]
    fn production_displays_as_release() {
        assert_eq!(Mode::Production.to_string(), "RELEASE");
    }

    #[test]
    fn debug_format_matches_variant_name() {
        assert_eq!(format!("{:?}", Mode::Development), "Development");
        assert_eq!(format!("{:?}", Mode::Production), "Production");
    }
}

mod traits {
    use super::*;

    #[test]
    fn equality() {
        assert_eq!(Mode::Development, Mode::Development);
        assert_eq!(Mode::Production, Mode::Production);
        assert_ne!(Mode::Development, Mode::Production);
    }

    #[test]
    fn copy() {
        let original = Mode::Production;
        let copied = original;
        assert_eq!(original, copied);
    }

    #[test]
    fn clone() {
        let original = Mode::Development;
        assert_eq!(original, original.clone());
    }
}
