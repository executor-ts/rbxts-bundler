use anyhow::Result;
use clap::Parser;

use rbxts_bundler::bundler;
use rbxts_bundler::cli::{Cli, Commands};

fn main() -> Result<()> {
    let Cli { bundle, command } = Cli::parse();
    let args = match command {
        Some(Commands::Bundle(args)) => args,
        None => bundle,
    };

    bundler::bundle(args)
}
