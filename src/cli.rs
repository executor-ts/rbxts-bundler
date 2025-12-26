//! Command-line interface definitions.

use std::fmt;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    disable_help_subcommand = true,
    subcommand_required = false,
    arg_required_else_help = true
)]
pub struct Cli {
    /// Bundle the input model into a single Luau file
    #[command(flatten)]
    pub bundle: Args,

    /// Available subcommands
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Bundle the input model into a single Luau file
    Bundle(Args),
}

#[derive(clap::Args, Debug, Clone)]
pub struct Args {
    /// Path to the input model file (.rbxm)
    #[arg(short = 'i', long)]
    pub input: PathBuf,

    /// Path to the output file (.lua)
    #[arg(short = 'o', long)]
    pub output: PathBuf,

    /// Enable release mode (minification, optimization)
    #[arg(short = 'r', long, default_value_t = false)]
    pub release: bool,

    /// Path to a custom header file
    #[arg(long)]
    pub header: Option<PathBuf>,

    /// Suppress standard output
    #[arg(short = 's', long, default_value_t = false)]
    pub silent: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Mode {
    Development,
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
