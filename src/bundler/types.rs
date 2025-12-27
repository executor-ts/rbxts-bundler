//! Core types for the bundler library.

use std::fmt;
use std::path::{Path, PathBuf};
use std::time::Duration;

pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");
pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Build mode for the bundler.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Mode {
    /// Development mode - keeps debug information and formatting.
    Development,
    /// Production mode - minifies and optimizes the output.
    Production,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mode::Development => write!(f, "DEBUG"),
            Mode::Production => write!(f, "RELEASE"),
        }
    }
}

/// Target configuration for a bundle output.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Target {
    /// Development build without compatibility shims.
    Dev,
    /// Development build with compatibility shims.
    DevCompat,
    /// Release/production build without compatibility shims.
    Rel,
    /// Release/production build with compatibility shims.
    RelCompat,
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Target::Dev => write!(f, "dev"),
            Target::DevCompat => write!(f, "dev-compat"),
            Target::Rel => write!(f, "rel"),
            Target::RelCompat => write!(f, "rel-compat"),
        }
    }
}

impl Target {
    /// Returns the build mode for this target.
    #[must_use]
    pub const fn mode(&self) -> Mode {
        match self {
            Target::Dev | Target::DevCompat => Mode::Development,
            Target::Rel | Target::RelCompat => Mode::Production,
        }
    }

    /// Returns whether this target uses compatibility shims.
    #[must_use]
    pub const fn compat(&self) -> bool {
        matches!(self, Target::DevCompat | Target::RelCompat)
    }

    /// Returns the file suffix for this target.
    #[must_use]
    pub const fn file_suffix(&self) -> &'static str {
        match self {
            Target::Dev => "debug",
            Target::DevCompat => "debug.c",
            Target::Rel => "release",
            Target::RelCompat => "release.c",
        }
    }
}

/// Configuration for a build operation.
#[derive(Debug, Clone)]
pub struct BuildConfig {
    /// Path to the input model file (.rbxm).
    pub input: PathBuf,
    /// Output directory for generated bundles.
    pub out_dir: PathBuf,
    /// One or more build targets.
    pub targets: Vec<Target>,
    /// Optional path to a custom header file content.
    pub header_content: Option<String>,
}

impl BuildConfig {
    /// Create a new build configuration.
    pub fn new(input: PathBuf, out_dir: PathBuf) -> Self {
        Self {
            input,
            out_dir,
            targets: vec![Target::Dev],
            header_content: None,
        }
    }

    /// Set the targets to build.
    pub fn with_targets(mut self, targets: Vec<Target>) -> Self {
        self.targets = targets;
        self
    }

    /// Set a custom header content.
    pub fn with_header(mut self, header: String) -> Self {
        self.header_content = Some(header);
        self
    }
}

/// Result of building a single target.
#[derive(Debug, Clone)]
pub struct TargetResult {
    /// The target that was built.
    pub target: Target,
    /// Path to the output file.
    pub output_file: PathBuf,
    /// Whether the build succeeded.
    pub success: bool,
    /// Error message if the build failed.
    pub error_message: Option<String>,
}

/// Result of a complete build operation.
#[derive(Debug)]
pub struct BuildResult {
    /// Path to the input file.
    pub input_path: PathBuf,
    /// Results for each target.
    pub target_results: Vec<TargetResult>,
    /// Total duration of the build.
    pub duration: Duration,
}

impl BuildResult {
    /// Returns true if all targets built successfully.
    #[must_use]
    pub fn is_success(&self) -> bool {
        self.target_results.iter().all(|r| r.success)
    }

    /// Returns the number of successful targets.
    #[must_use]
    pub fn success_count(&self) -> usize {
        self.target_results.iter().filter(|r| r.success).count()
    }

    /// Returns the first error message if any target failed.
    #[must_use]
    pub fn first_error(&self) -> Option<&str> {
        self.target_results
            .iter()
            .find(|r| !r.success)
            .and_then(|r| r.error_message.as_deref())
    }
}

/// Shared context for bundler operations.
pub struct BundlerContext<'a> {
    pub mode: Mode,
    pub input_path: &'a Path,
}

impl<'a> BundlerContext<'a> {
    pub fn new(mode: Mode, input_path: &'a Path) -> Self {
        Self { mode, input_path }
    }

    /// Replaces template placeholders with actual values.
    #[must_use]
    pub fn apply_templates(&self, content: &str) -> String {
        let input_display = if self.input_path.is_relative() {
            // Already relative, use as-is
            self.input_path.display().to_string()
        } else {
            // Absolute path - try to make it relative to cwd
            std::env::current_dir()
                .ok()
                .and_then(|cwd| self.input_path.strip_prefix(&cwd).ok())
                .map(|rel| rel.display().to_string())
                .unwrap_or_else(|| {
                    // Fallback to just the filename to avoid exposing user directory paths
                    self.input_path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown")
                        .to_string()
                })
        };
        
        content
            .replace("{{NAME}}", PKG_NAME)
            .replace("{{VERSION}}", PKG_VERSION)
            .replace("{{INPUT}}", &input_display)
    }
}
