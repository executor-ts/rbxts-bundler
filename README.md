# rbxts-bundler

A blazing fast, Rust-based bundler that compiles Roblox Instance models (`.rbxm`) into a single, portable Luau script.

Designed primarily for `rbxts` users who need to run their projects in environments that require a single file entry point (such as executors or specific testing environments), `rbxts-bundler` preserves the instance hierarchy and provides a virtualized runtime for `require` calls.

## Features

* **Single File Output:** Collapses a complex tree of `ModuleScript`s and `LocalScript`s into one standalone `.lua` file.
* **Virtual Filesystem:** Reconstructs the instance tree in memory, ensuring `script.Parent` and hierarchy-based logic work as expected.
* **Circular Dependency Detection:** The runtime shim detects and reports circular dependencies between modules.
* **Built-in Minification:** Integrated [Darklua](https://darklua.com/) support for release builds to minify and optimize output.
* **Customizable:** Support for custom file headers and Darklua configurations.
* **Parallel Builds:** Multi-target builds run in parallel for faster compilation.
* **Library Support:** Can be used as a Rust library/crate in addition to CLI usage.

## Installation

`rbxts-bundler` is available via standard tool managers.

### Aftman

```toml
# aftman.toml
[tools]
rbxts-bundler = "executor-ts/rbxts-bundler@0.2.0" # Replace with latest version
```

### Rokit

```bash
rokit add executor-ts/rbxts-bundler
```

### From Source (Cargo)

```bash
cargo install --git https://github.com/executor-ts/rbxts-bundler
```

### As a Library Dependency

Add to your `Cargo.toml`:

```toml
[dependencies]
rbxts-bundler = { git = "https://github.com/executor-ts/rbxts-bundler" }
```

## CLI Usage

The basic usage requires an input model file (`.rbxm`) and an output destination (`.lua`).

```bash
# Build development target (default)
rbxts-bundler build model.rbxm -o dist

# Build multiple targets
rbxts-bundler build model.rbxm -t dev -t rel -t rel-compat -o dist

# With custom header
rbxts-bundler build model.rbxm -t rel -o dist --header ./license_header.txt
```

**Available targets:**
- `dev` - Development (unminified, uses `loadstring`)
- `dev-compat` - Development with compatibility mode
- `rel` - Release (minified, optimized)
- `rel-compat` - Release with compatibility mode

**Note:** Compat targets make the generated Luau more likely to run in outdated environments and potentially even Lua 5.3 by avoiding newer language features and providing polyfills.

Output files are named `<input-stem>.<target>.lua` in the specified output directory.

## Library Usage

The bundler can be used programmatically as a Rust library:

```rust
use std::path::PathBuf;
use rbxts_bundler::{build, BuildConfig, Target};

fn main() -> anyhow::Result<()> {
    // Create a build configuration
    let config = BuildConfig::new(
        PathBuf::from("input.rbxm"),
        PathBuf::from("dist"),
    )
    .with_targets(vec![Target::Dev, Target::Rel]);

    // Run the build
    let result = build(&config)?;

    // Check the results
    if result.is_success() {
        println!("Build completed in {:?}", result.duration);
        for target_result in &result.target_results {
            println!("  {} -> {}", target_result.target, target_result.output_file.display());
        }
    } else {
        for target_result in &result.target_results {
            if let Some(err) = &target_result.error_message {
                eprintln!("Failed {}: {}", target_result.target, err);
            }
        }
    }

    Ok(())
}
```

### Library API

The main types exposed by the library:

- **`BuildConfig`** - Configuration for a build operation
- **`BuildResult`** - Result of a build operation with per-target results and duration
- **`TargetResult`** - Individual target result with success status and error message
- **`Target`** - Build target variants (`Dev`, `DevCompat`, `Rel`, `RelCompat`)
- **`Mode`** - Build mode (`Development`, `Production`)
- **`build(config)`** - Main entry point to run a build

## CLI Options

| Flag | Short | Description |
| --- | --- | --- |
| `build <INPUT>` |  | Path to the input model file (`.rbxm`). |
| `--target <TARGET>` | `-t` | Build target(s): `dev`, `dev-compat`, `rel`, `rel-compat` (can be specified multiple times, default: `dev`). |
| `--out-dir <DIR>` | `-o` | Output directory for generated bundles. |
| `--header <PATH>` |  | Path to a custom header file to prepend to the output. |
| `--quiet` | `-q` | Suppress progress output, show only errors. |
| `--silent` | `-s` | Suppress all output including errors. |

## How it Works

1. **Parsing:** The tool reads the binary Roblox model (`.rbxm`).
2. **Virtualization:** It wraps every script in a closure and registers it into a virtual DOM table.
3. **Shim Generation:** A lightweight runtime shim is prepended to the file. This shim handles:
   * Virtual instance creation.
   * `require()` logic (resolving modules within the virtual tree).
   * Thread-safe loading of modules.
4. **Minification:** If running in release mode, the final assembled Lua string is passed through `darklua` to reduce file size and obfuscate variable names.

## License

This project is licensed under the MIT License.