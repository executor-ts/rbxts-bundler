//! Core bundler functionality for compiling `.rbxm` models into Luau scripts.
//!
//! This module provides the main [`build`] function and re-exports commonly used types.

pub mod escape;
pub mod minify;
pub mod traverse;
pub mod types;
pub mod writer;

use std::fmt::Write;
use std::fs;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::time::Instant;

use anyhow::{bail, Context, Result};
use rayon::prelude::*;
use rbx_dom_weak::WeakDom;

use crate::assets;
use minify::minify;
use traverse::process_instance;

// Re-export public types for library consumers
pub use types::{BuildConfig, BuildResult, Mode, Target, TargetResult, PKG_NAME, PKG_VERSION};

// Internal re-exports for submodules
pub(crate) use types::BundlerContext;

// ─────────────────────────────────────────────────────────────────────────────
// Public API
// ─────────────────────────────────────────────────────────────────────────────

/// Build one or more targets into the provided output directory.
///
/// Returns a [`BuildResult`] containing structured information about the build,
/// including per-target success/failure status and total duration.
///
/// # Errors
///
/// Returns an error if:
/// - No targets are specified
/// - The input file does not exist
/// - The model file cannot be parsed
/// - The output directory cannot be created
///
/// # Example
///
/// ```no_run
/// use std::path::PathBuf;
/// use rbxts_bundler::{build, BuildConfig, Target};
///
/// let config = BuildConfig::new(
///     PathBuf::from("input.rbxm"),
///     PathBuf::from("dist"),
/// ).with_targets(vec![Target::Dev, Target::Rel]);
///
/// let result = build(&config).expect("Build failed");
/// assert!(result.is_success());
/// ```
pub fn build(config: &BuildConfig) -> Result<BuildResult> {
    let start_time = Instant::now();

    validate_config(config)?;

    let dom = load_model(&config.input)?;
    let stem = extract_stem(&config.input);
    let targets = prepare_targets(config, &stem);

    fs::create_dir_all(&config.out_dir).context("Failed to create output directory")?;

    let outcomes = build_targets_parallel(&targets, &dom, &config.input, config.header_content.as_ref())?;

    let target_results = outcomes
        .into_iter()
        .map(|(idx, res)| {
            let spec = &targets[idx];
            TargetResult {
                target: spec.target,
                output_file: spec.output.clone(),
                success: res.is_ok(),
                error_message: res.err().map(|e| format!("{e:#}")),
            }
        })
        .collect();

    Ok(BuildResult {
        input_path: config.input.clone(),
        target_results,
        duration: start_time.elapsed(),
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// Validation & Setup
// ─────────────────────────────────────────────────────────────────────────────

/// Validates the build configuration before processing.
fn validate_config(config: &BuildConfig) -> Result<()> {
    if config.targets.is_empty() {
        bail!("At least one target must be specified");
    }
    if !config.input.exists() {
        bail!("Input file does not exist: {}", config.input.display());
    }
    Ok(())
}

/// Loads and parses the `.rbxm` model file.
fn load_model(input: &Path) -> Result<WeakDom> {
    let file = fs::File::open(input).context("Failed to open input file")?;
    let dom = rbx_binary::from_reader(BufReader::new(file)).context("Failed to decode model")?;

    if dom.root().children().is_empty() {
        bail!("Model file contains no instances");
    }

    Ok(dom)
}

/// Extracts the file stem from the input path for naming output files.
fn extract_stem(input: &Path) -> String {
    input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("bundle")
        .to_string()
}

/// Prepares target specifications from the build configuration.
fn prepare_targets(config: &BuildConfig, stem: &str) -> Vec<TargetSpec> {
    config
        .targets
        .iter()
        .map(|target| {
            let filename = format!("{}.{}.lua", stem, target.file_suffix());
            TargetSpec {
                target: *target,
                mode: target.mode(),
                compat: target.compat(),
                output: config.out_dir.join(filename),
            }
        })
        .collect()
}

// ─────────────────────────────────────────────────────────────────────────────
// Parallel Build Execution
// ─────────────────────────────────────────────────────────────────────────────

/// Internal specification for an output target.
#[derive(Clone, Debug)]
struct TargetSpec {
    target: Target,
    mode: Mode,
    compat: bool,
    output: PathBuf,
}

/// Builds all targets in parallel using a custom thread pool.
fn build_targets_parallel(
    targets: &[TargetSpec],
    dom: &WeakDom,
    input_path: &Path,
    header: Option<&String>,
) -> Result<Vec<(usize, Result<()>)>> {
    let stack_size = estimate_thread_stack_size(dom);
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(targets.len().max(1))
        .stack_size(stack_size)
        .build()
        .context("Failed to build thread pool")?;

    let results = pool.install(|| {
        targets
            .par_iter()
            .enumerate()
            .map(|(idx, spec)| (idx, build_single_target(dom, input_path, header, spec)))
            .collect()
    });

    Ok(results)
}

/// Builds a single target and writes the output file.
fn build_single_target(
    dom: &WeakDom,
    input_path: &Path,
    header_content: Option<&String>,
    target: &TargetSpec,
) -> Result<()> {
    let ctx = BundlerContext::new(target.mode, input_path);
    let source = generate_bundle(dom, &ctx, header_content, target)?;

    let final_source = if target.mode == Mode::Production {
        let config = if target.compat {
            assets::DARKLUA_REL_COMPAT
        } else {
            assets::DARKLUA_REL
        };
        let minified = minify(&source, config)?;

        // Prepend header after minification to preserve it
        let header_raw = header_content.map_or(assets::FILE_HEADER, String::as_str);
        let header = ctx.apply_templates(header_raw);
        format!("{header}\n{minified}")
    } else {
        source
    };

    if let Some(parent) = target.output.parent() {
        fs::create_dir_all(parent).context("Failed to create output directory")?;
    }

    fs::write(&target.output, final_source).context("Failed to write output file")?;
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// Bundle Generation
// ─────────────────────────────────────────────────────────────────────────────

/// Generates the bundle content for a single target.
fn generate_bundle(
    dom: &WeakDom,
    ctx: &BundlerContext<'_>,
    header_content: Option<&String>,
    target: &TargetSpec,
) -> Result<String> {
    let mut output = String::with_capacity(64 * 1024);

    // For production builds, skip header (added after minification)
    if target.mode != Mode::Production {
        let header_raw = header_content.map_or(assets::FILE_HEADER, String::as_str);
        let header = ctx.apply_templates(header_raw);
        writeln!(output, "{header}\n")?;
    }

    // Write runtime shim
    let runtime_raw = format!("{}\n{}", assets::RUNTIME_HEADER, assets::RUNTIME_BODY);
    let runtime = ctx.apply_templates(&runtime_raw);
    writeln!(output, "{runtime}\n")?;

    // Write tree header
    let tree_header = ctx.apply_templates(assets::TREE_HEADER);
    writeln!(output, "{tree_header}")?;

    // Get darklua config for dev builds
    let darklua_config = if target.mode == Mode::Development {
        Some(if target.compat {
            assets::DARKLUA_DEV_COMPAT
        } else {
            assets::DARKLUA_DEV
        })
    } else {
        None
    };

    // Process the instance tree
    let root_children = dom.root().children();
    let main_ref = root_children[0];
    let main_instance = dom.get_by_ref(main_ref).expect("Root child must exist");

    process_instance(
        dom,
        &mut output,
        ctx.mode,
        main_ref,
        &main_instance.name,
        "nil",
        darklua_config,
    )?;

    writeln!(output, "__start()")?;

    Ok(output)
}

// ─────────────────────────────────────────────────────────────────────────────
// Thread Pool Sizing
// ─────────────────────────────────────────────────────────────────────────────

/// Computes the maximum depth of the DOM tree using iterative DFS.
#[inline]
fn max_dom_depth(dom: &WeakDom) -> usize {
    let root_children = dom.root().children();
    if root_children.is_empty() {
        return 0;
    }

    let mut max_depth = 1usize;
    let mut stack = Vec::with_capacity(64);

    for &child in root_children {
        stack.push((child, 1));
    }

    while let Some((referent, depth)) = stack.pop() {
        max_depth = max_depth.max(depth);

        if let Some(inst) = dom.get_by_ref(referent) {
            let next_depth = depth + 1;
            for &ch in inst.children() {
                stack.push((ch, next_depth));
            }
        }
    }

    max_depth
}

/// Estimates the required thread stack size based on DOM depth.
#[inline]
fn estimate_thread_stack_size(dom: &WeakDom) -> usize {
    const BASE_BYTES: usize = 4 * 1024 * 1024; // 4 MB base
    const STACK_PER_LEVEL: usize = 64 * 1024; // 64 KB per level
    const MAX_STACK_BYTES: usize = 512 * 1024 * 1024; // 512 MB cap

    let depth = max_dom_depth(dom);
    let stack_bytes = BASE_BYTES.saturating_add(depth.saturating_mul(STACK_PER_LEVEL));
    stack_bytes.min(MAX_STACK_BYTES)
}
