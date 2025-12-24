//! Tests for CLI argument parsing and Mode enum.

use rbxts_bundler::cli::Mode;

#[test]
fn test_mode_display_development() {
    let mode = Mode::Development;
    assert_eq!(format!("{}", mode), "DEBUG");
}

#[test]
fn test_mode_display_production() {
    let mode = Mode::Production;
    assert_eq!(format!("{}", mode), "RELEASE");
}

#[test]
fn test_mode_equality() {
    assert_eq!(Mode::Development, Mode::Development);
    assert_eq!(Mode::Production, Mode::Production);
    assert_ne!(Mode::Development, Mode::Production);
}

#[test]
fn test_mode_copy() {
    let mode1 = Mode::Production;
    let mode2 = mode1; // Copy
    assert_eq!(mode1, mode2);
}

#[test]
fn test_mode_clone() {
    let mode1 = Mode::Development;
    let mode2 = mode1.clone();
    assert_eq!(mode1, mode2);
}

#[test]
fn test_mode_debug() {
    let mode = Mode::Development;
    let debug_str = format!("{:?}", mode);
    assert_eq!(debug_str, "Development");
    
    let mode = Mode::Production;
    let debug_str = format!("{:?}", mode);
    assert_eq!(debug_str, "Production");
}
