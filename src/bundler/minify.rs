use std::path::Path;

use anyhow::{anyhow, bail, Context, Result};
use darklua_core::{Configuration, Options, Resources};

/// Minifies Luau source code using darklua with the provided configuration.
pub fn minify(text: &str, config_content: &str) -> Result<String> {
    let config: Configuration =
        serde_json::from_str(config_content).context("Failed to parse darklua configuration")?;

    let resources = Resources::from_memory();
    let temp_file = "temp.lua";

    resources
        .write(temp_file, text)
        .map_err(|e| anyhow!("Failed to write to darklua resources: {:?}", e))?;

    let options = Options::new(Path::new(temp_file)).with_configuration(config);

    match darklua_core::process(&resources, options) {
        Ok(_) => {
            let result = resources
                .get(temp_file)
                .map_err(|e| anyhow!("Failed to retrieve minified content: {:?}", e));
            
            // Clean up temp file to prevent memory accumulation
            let _ = resources.remove(temp_file);
            
            result
        }
        Err(e) => bail!("Darklua minification failed: {}", e),
    }
}
