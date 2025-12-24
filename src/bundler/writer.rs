use anyhow::Result;
use rbx_dom_weak::{types::Variant, Instance};

use crate::cli::Mode;

use super::escape::append_luau_string;

/// Writes a non-script instance registration.
pub(crate) fn write_instance(
    output: &mut String,
    instance: &Instance,
    class_name: &str,
    full_path_quoted: &str,
    parent_path_quoted: &str,
) -> Result<()> {
    output.push_str("__rbx(");
    append_luau_string(&instance.name, output);
    output.push_str(", ");
    append_luau_string(class_name, output);
    output.push_str(", ");
    output.push_str(full_path_quoted);
    output.push_str(", ");
    output.push_str(parent_path_quoted);
    output.push_str(")\n");
    Ok(())
}

/// Writes a script registration (LocalScript or ModuleScript).
pub(crate) fn write_script(
    output: &mut String,
    instance: &Instance,
    class_name: &str,
    full_path_quoted: &str,
    parent_path_quoted: &str,
    mode: Mode,
) -> Result<()> {
    let source_code = instance
        .properties
        .iter()
        .find(|(k, _)| k.as_str() == "Source")
        .map(|(_, v)| match v {
            Variant::String(s) => s.as_str(),
            Variant::BinaryString(b) => std::str::from_utf8(b.as_ref()).unwrap_or(""),
            _ => "",
        })
        .unwrap_or("");

    output.push_str("__lua(");
    append_luau_string(&instance.name, output);
    output.push_str(", ");
    append_luau_string(class_name, output);
    output.push_str(", ");
    output.push_str(full_path_quoted);
    output.push_str(", ");
    output.push_str(parent_path_quoted);
    output.push_str(", function()\n");

    if mode == Mode::Production {
        output.push_str("\tlocal _=__env(");
        output.push_str(full_path_quoted);
        output.push_str(")\n\tlocal script,require=_.script,_.require\n\t");
        output.push_str(source_code);
        output.push('\n');
    } else {
        let wrapped_code = format!(
            "local _=(...)( {} ) local script,require=_.script,_.require\n{}",
            full_path_quoted, source_code
        );
        output.push_str("\treturn assert(loadstring(");
        append_luau_string(&wrapped_code, output);
        output.push_str(", ");
        output.push_str(full_path_quoted);
        output.push_str("))(__env)\n");
    }

    output.push_str("end)\n");
    Ok(())
}
