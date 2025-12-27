use anyhow::{Context, Result};
use rbx_dom_weak::WeakDom;

use super::escape::to_luau_string;
use super::types::Mode;
use super::writer::{write_instance, write_script};

/// Recursively processes an instance and its children, writing to the output buffer.
pub(crate) fn process_instance(
    dom: &WeakDom,
    output: &mut String,
    mode: Mode,
    referent: rbx_dom_weak::types::Ref,
    full_path: &str,
    parent_path_quoted: &str,
    darklua_config: Option<&str>,
) -> Result<()> {
    let instance = dom
        .get_by_ref(referent)
        .context("Referent missing from DOM tree")?;

    let current_path_quoted = to_luau_string(full_path);

    match instance.class.as_str() {
        "LocalScript" | "ModuleScript" => write_script(
            output,
            instance,
            &instance.class,
            &current_path_quoted,
            parent_path_quoted,
            mode,
            darklua_config,
        )?,
        _ => write_instance(
            output,
            instance,
            &instance.class,
            &current_path_quoted,
            parent_path_quoted,
        )?,
    }

    // Reuse a buffer for child paths to avoid per-child allocations
    let mut child_path_buf = String::with_capacity(full_path.len() + 64);
    
    for child_ref in instance.children() {
        let child = dom
            .get_by_ref(*child_ref)
            .context("Child reference missing")?;

        // Build child path by reusing buffer
        child_path_buf.clear();
        child_path_buf.push_str(full_path);
        child_path_buf.push('.');
        child_path_buf.push_str(&child.name);
        
        process_instance(
            dom,
            output,
            mode,
            *child_ref,
            &child_path_buf,
            &current_path_quoted,
            darklua_config,
        )?;
    }

    Ok(())
}
