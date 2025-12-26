use anyhow::{Context, Result};
use rbx_dom_weak::WeakDom;

use crate::cli::Mode;

use super::escape::to_luau_string;
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

    for child_ref in instance.children() {
        let child = dom
            .get_by_ref(*child_ref)
            .context("Child reference missing")?;

        let child_path = format!("{}.{}", full_path, child.name);
        process_instance(
            dom,
            output,
            mode,
            *child_ref,
            &child_path,
            &current_path_quoted,
            darklua_config,
        )?;
    }

    Ok(())
}
