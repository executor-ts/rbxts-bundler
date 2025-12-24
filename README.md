# rbxts-bundler

A blazing fast, Rust-based bundler that compiles Roblox Instance models (`.rbxm`) into a single, portable Luau script.

Designed primarily for `rbxts` users who need to run their projects in environments that require a single file entry point (such as executors or specific testing environments), `rbxts-bundler` preserves the instance hierarchy and provides a virtualized runtime for `require` calls.

## Features

* **Single File Output:** Collapses a complex tree of `ModuleScript`s and `LocalScript`s into one standalone `.lua` file.
* **Virtual Filesystem:** Reconstructs the instance tree in memory, ensuring `script.Parent` and hierarchy-based logic work as expected.
* **Circular Dependency Detection:** The runtime shim detects and reports circular dependencies between modules.
* **Built-in Minification:** Integrated [Darklua](https://darklua.com/) support for release builds to minify and optimize output.
* **Customizable:** Support for custom file headers and Darklua configurations.

## Installation

`rbxts-bundler` is available via standard tool managers.

### Aftman

```toml
# aftman.toml
[tools]
rbxts-bundler = "executor-ts/rbxts-bundler@0.1.0" # Replace with latest version
```

### Rokit

```bash
rokit add executor-ts/rbxts-bundler
```

### From Source (Cargo)

```bash
cargo install --git https://github.com/executor-ts/rbxts-bundler
```

## Usage

The basic usage requires an input model file (`.rbxm`) and an output destination (`.lua`).

```bash
rbxts-bundler --input model.rbxm --output bundle.lua
```

### Release Build

To generate a production-ready bundle, use the `--release` (or `-r`) flag. This enables Darklua minification and optimization.

```bash
rbxts-bundler -i model.rbxm -o bundle.lua --release
```

### Custom Configuration

You can provide a custom Darklua configuration file or a custom header (copyright/license comment) for the output file.

```bash
rbxts-bundler \
  -i model.rbxm \
  -o bundle.lua \
  -r \
  --darklua-config ./darklua.json \
  --header ./license_header.txt
```

## CLI Options

| Flag | Short | Description |
| --- | --- | --- |
| `--input <PATH>` | `-i` | Path to the input model file (`.rbxm`). |
| `--output <PATH>` | `-o` | Path to the output file (`.lua`). |
| `--release` | `-r` | Enable release mode (minification, optimization). |
| `--header <PATH>` |  | Path to a custom header file to prepend to the output. |
| `--darklua-config <PATH>` |  | Path to a custom Darklua configuration file. |
| `--silent` | `-s` | Suppress standard output logs. |

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