use std::path::Path;

use crate::cli::Mode;

pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");
pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

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
    pub fn apply_templates(&self, content: &str) -> String {
        content
            .replace("{{NAME}}", PKG_NAME)
            .replace("{{VERSION}}", PKG_VERSION)
            .replace("{{INPUT}}", &self.input_path.display().to_string())
    }
}
