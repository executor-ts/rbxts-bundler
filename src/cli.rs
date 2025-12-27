//! Command-line interface definitions.

use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};

use crate::bundler::{BuildConfig, Target};

/// CLI-specific target enum that maps to bundler::Target
#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum CliTarget {
    Dev,
    DevCompat,
    Rel,
    RelCompat,
}

impl From<CliTarget> for Target {
    fn from(value: CliTarget) -> Self {
        match value {
            CliTarget::Dev => Self::Dev,
            CliTarget::DevCompat => Self::DevCompat,
            CliTarget::Rel => Self::Rel,
            CliTarget::RelCompat => Self::RelCompat,
        }
    }
}

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    disable_help_subcommand = true,
    subcommand_required = true,
)]
pub struct Cli {
    /// Available subcommands
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Build one or more targets into the output directory
    Build(BuildArgs),
}

/// Verbosity level for CLI output.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum Verbosity {
    /// Suppress all output including errors
    Silent,
    /// Suppress progress/info, show only errors
    Quiet,
    /// Normal output (default)
    #[default]
    Normal,
}

#[derive(clap::Args, Debug, Clone)]
pub struct BuildArgs {
    /// Path to the input model file (.rbxm)
    pub input: PathBuf,

    /// One or more build targets
    #[arg(short = 't', long = "target", value_enum, default_values_t = [CliTarget::Dev])]
    pub targets: Vec<CliTarget>,

    /// Output directory for generated bundles
    #[arg(short = 'o', long = "out-dir")]
    pub out_dir: PathBuf,

    /// Path to a custom header file
    #[arg(long)]
    pub header: Option<PathBuf>,

    /// Suppress progress output, show only errors
    #[arg(short = 'q', long = "quiet", conflicts_with = "silent")]
    pub quiet: bool,

    /// Suppress all output including errors
    #[arg(short = 's', long = "silent", conflicts_with = "quiet")]
    pub silent: bool,
}

impl BuildArgs {
    /// Get the verbosity level from CLI flags.
    pub fn verbosity(&self) -> Verbosity {
        if self.silent {
            Verbosity::Silent
        } else if self.quiet {
            Verbosity::Quiet
        } else {
            Verbosity::Normal
        }
    }

    /// Convert CLI arguments to a BuildConfig for the bundler library.
    pub fn to_build_config(&self) -> Result<BuildConfig> {
        let header_content = self
            .header
            .as_ref()
            .map(|p| fs::read_to_string(p).context("Failed to read header file"))
            .transpose()?;

        let targets = self.targets.iter().copied().map(Target::from).collect();

        let mut config = BuildConfig::new(self.input.clone(), self.out_dir.clone())
            .with_targets(targets);

        if let Some(header) = header_content {
            config = config.with_header(header);
        }

        Ok(config)
    }
}
