pub mod escape;
pub mod minify;
pub mod traverse;
pub mod types;
pub mod writer;

use std::fmt::Write;
use std::fs;
use std::io::BufReader;
use std::path::Path;

use anyhow::{bail, Context, Result};
use colored::Colorize;
use rbx_dom_weak::WeakDom;

use crate::assets;
use crate::cli::{Args, Mode};
use crate::logging::Logger;

pub use minify::minify;
use traverse::process_instance;
use types::BundlerContext;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn bundle(args: Args) -> Result<()> {
    let mode = if args.release {
        Mode::Production
    } else {
        Mode::Development
    };
    let total_steps = if mode == Mode::Production { 5 } else { 4 };
    let logger = Logger::new(total_steps, args.silent);

    if !args.silent {
        eprintln!("{} v{} [{}]", PKG_NAME.cyan().bold(), PKG_VERSION, mode);
    }

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

    logger.step("Processing instance tree...");

    let header_content = args
        .header
        .as_ref()
        .map(|p| fs::read_to_string(p).context("Failed to read header file"))
        .transpose()?;

    let bundler = Bundler::new(&dom, mode, &args.input);

    logger.step("Assembling source code...");
    let mut source = bundler.build(header_content)?;

    if mode == Mode::Production {
        logger.step("Minifying output...");
        source = minify(&source, assets::DARKLUA_CONFIG)?;
    }

    logger.step("Writing to disk...");

    if let Some(parent) = args.output.parent() {
        fs::create_dir_all(parent).context("Failed to create output directory")?;
    }

    fs::write(&args.output, source).context("Failed to write output file")?;
    logger.finish(&format!("Bundle generated at {}", args.output.display()));

    Ok(())
}

pub struct Bundler<'a> {
    dom: &'a WeakDom,
    ctx: BundlerContext<'a>,
    output_buffer: String,
}

impl<'a> Bundler<'a> {
    pub fn new(dom: &'a WeakDom, mode: Mode, input_path: &'a Path) -> Self {
        Self {
            dom,
            ctx: BundlerContext::new(mode, input_path),
            output_buffer: String::with_capacity(64 * 1024),
        }
    }

    pub fn build(mut self, header_override: Option<String>) -> Result<String> {
        let header_raw = header_override.as_deref().unwrap_or(assets::FILE_HEADER);
        let header = self.ctx.apply_templates(header_raw);
        writeln!(self.output_buffer, "{}\n", header)?;

        let runtime_raw = format!("{}\n{}", assets::RUNTIME_HEADER, assets::RUNTIME_BODY);
        let runtime = self.ctx.apply_templates(&runtime_raw);
        writeln!(self.output_buffer, "{}\n", runtime)?;

        let tree_header = self.ctx.apply_templates(assets::TREE_HEADER);
        writeln!(self.output_buffer, "{}", tree_header)?;

        let root_children = self.dom.root().children();
        if root_children.is_empty() {
            bail!("Model file contains no instances");
        }

        let main_ref = root_children[0];
        let main_instance = self.dom.get_by_ref(main_ref).unwrap();

        process_instance(
            self.dom,
            &mut self.output_buffer,
            self.ctx.mode,
            main_ref,
            &main_instance.name,
            "nil",
        )?;

        writeln!(self.output_buffer, "__start()")?;

        Ok(self.output_buffer)
    }
}
