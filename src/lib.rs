//! # rbxts-bundler
//!
//! A library for bundling Roblox TypeScript projects into single Lua files.
//!
//! ## Library Usage
//!
//! The bundler can be used as a library by importing the `bundler` module:
//!
//! ```no_run
//! use std::path::PathBuf;
//! use rbxts_bundler::{build, BuildConfig, Target};
//!
//! // Create a build configuration
//! let config = BuildConfig::new(
//!     PathBuf::from("input.rbxm"),
//!     PathBuf::from("dist"),
//! )
//! .with_targets(vec![Target::Dev, Target::Rel]);
//!
//! // Run the build
//! let result = build(&config).expect("Build failed");
//!
//! // Check the results
//! if result.is_success() {
//!     println!("Build completed in {:?}", result.duration);
//!     for target_result in &result.target_results {
//!         println!("  {} -> {}", target_result.target, target_result.output_file.display());
//!     }
//! }
//! ```
//!
//! ## Available Targets
//!
//! - `Target::Dev` - Development build (debug mode, no minification)
//! - `Target::DevCompat` - Development build with compatibility shims
//! - `Target::Rel` - Release build (production mode, minified)
//! - `Target::RelCompat` - Release build with compatibility shims

pub mod assets;
pub mod bundler;

// CLI and logging are internal to the binary
#[doc(hidden)]
pub mod cli;
#[doc(hidden)]
pub mod logging;

// Re-export commonly used types at the crate root for convenience
pub use bundler::{build, BuildConfig, BuildResult, Mode, Target, TargetResult, PKG_NAME, PKG_VERSION};
