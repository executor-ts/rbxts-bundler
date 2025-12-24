pub mod escape;
pub mod minify;
pub mod traverse;
pub mod types;
pub mod writer;

use std::fmt::Write;
use std::path::Path;

use anyhow::{bail, Result};
use rbx_dom_weak::WeakDom;

use crate::cli::Mode;
use crate::templates;

pub use minify::minify;
use traverse::process_instance;
use types::BundlerContext;

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
        let header_raw = header_override.as_deref().unwrap_or(templates::FILE_HEADER);
        let header = self.ctx.apply_templates(header_raw);
        writeln!(self.output_buffer, "{}\n", header)?;

        let runtime_raw = format!("{}\n{}", templates::RUNTIME_HEADER, templates::RUNTIME_BODY);
        let runtime = self.ctx.apply_templates(&runtime_raw);
        writeln!(self.output_buffer, "{}\n", runtime)?;

        let tree_header = self.ctx.apply_templates(templates::TREE_HEADER);
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
