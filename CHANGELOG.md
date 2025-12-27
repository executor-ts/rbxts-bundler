# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2025-12-27

### Added
- **Library API**: Can now be used as a Rust crate with `build()`, `BuildConfig`, `BuildResult`, and `Target` types
- **Multi-target builds**: Build multiple targets in a single command (`-t dev -t rel`)
- **New CLI structure**: Introduced `build` subcommand with improved argument handling
- **Parallel target compilation**: Multiple targets are built concurrently using rayon
- **New output naming**: Files are now named `<input>.<target>.lua` (e.g., `build.debug.lua`, `build.release.c.lua`)
- **Quiet mode**: New `-q/--quiet` flag to suppress progress output while showing errors

### Changed
- CLI now uses subcommand pattern (`rbxts-bundler build` instead of direct flags)
- Renamed targets: `dev`, `dev-compat`, `rel`, `rel-compat` (previously used `--release` flag)
- Output directory is now specified with `-o/--out-dir` instead of direct output path
- Improved error messages and build status reporting
- Optimized string escaping with lookup table for better performance

### Removed
- Old CLI interface (`-i/--input`, `-o/--output`, `-r/--release` flags)
- Standalone `--darklua-config` option (now uses built-in configs per target)

## [0.1.0] - 2025-12-23

### Added
- Initial release
- CLI tool for bundling `.rbxm` files into portable Luau scripts
- Support for debug and release build modes
- Optional compatibility mode for broader Luau runtime support
- Parallel processing with `rayon`
- Minification support for release builds
- Cross-platform binaries (Linux, macOS, Windows)

[Unreleased]: https://github.com/executor-ts/rbxts-bundler/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/executor-ts/rbxts-bundler/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/executor-ts/rbxts-bundler/releases/tag/v0.1.0
