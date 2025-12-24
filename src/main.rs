use std::borrow::Cow;
use std::fs;
use std::io::BufReader;

use anyhow::{bail, Context, Result};
use clap::Parser;
use colored::Colorize;

use rbxts_bundler::bundler::{minify, Bundler};
use rbxts_bundler::cli::{Args, Mode};
use rbxts_bundler::logging::Logger;
use rbxts_bundler::templates;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<()> {
    let args = Args::parse();
    let mode = if args.release { Mode::Production } else { Mode::Development };
    let total_steps = if mode == Mode::Production { 5 } else { 4 };
    let logger = Logger::new(total_steps, args.silent);

    if !args.silent {
        eprintln!("{} v{} [{}]", PKG_NAME.cyan().bold(), PKG_VERSION, mode);
    }

    // Load model
    logger.step(&format!("Loading model from {}", args.input.display()));

    if !args.input.exists() {
        logger.fail(&format!("Model file not found: {}", args.input.display()));
        bail!("Input file does not exist");
    }

    let file = fs::File::open(&args.input).context("Failed to open input file")?;
    let dom = rbx_binary::from_reader(BufReader::new(file)).context("Failed to decode model")?;

    if dom.root().children().is_empty() {
        logger.fail("Model file is empty");
        bail!("Model file contains no instances");
    }

    // Process tree
    logger.step("Processing instance tree...");

    let header_content = args
        .header
        .as_ref()
        .map(|p| fs::read_to_string(p).context("Failed to read header file"))
        .transpose()?;

    let bundler = Bundler::new(&dom, mode, &args.input);

    // Assemble source
    logger.step("Assembling source code...");
    let mut source = bundler.build(header_content)?;

    // Minify (production only)
    if mode == Mode::Production {
        logger.step("Minifying output...");

        let config_str = match &args.darklua_config {
            Some(path) => Cow::Owned(fs::read_to_string(path).context("Failed to read darklua config")?),
            None => Cow::Borrowed(templates::DARKLUA_CONFIG),
        };

        source = minify(&source, &config_str)?;
    }

    // Write output
    logger.step("Writing to disk...");

    if let Some(parent) = args.output.parent() {
        fs::create_dir_all(parent).context("Failed to create output directory")?;
    }

    fs::write(&args.output, source).context("Failed to write output file")?;
    logger.finish(&format!("Bundle generated at {}", args.output.display()));

    Ok(())
}