//! Build UI and display utilities for the CLI.

use std::time::Duration;

use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};

use crate::bundler::{BuildResult, PKG_NAME, PKG_VERSION};
use crate::cli::Verbosity;

/// A spinner-based UI for displaying build progress.
pub struct BuildUI {
    spinner: Option<ProgressBar>,
    verbosity: Verbosity,
}

impl BuildUI {
    pub fn new(verbosity: Verbosity) -> Self {
        let spinner = if verbosity == Verbosity::Normal {
            let style = ProgressStyle::default_spinner()
                .template("{spinner:.cyan} {msg}")
                .unwrap();
            let pb = ProgressBar::new_spinner();
            pb.set_style(style);
            pb.enable_steady_tick(Duration::from_millis(80));
            Some(pb)
        } else {
            None
        };
        
        Self { spinner, verbosity }
    }
    
    /// Print the header with version info.
    pub fn print_header(&self) {
        if self.verbosity != Verbosity::Normal {
            return;
        }
        eprintln!("{} v{}", PKG_NAME.cyan().bold(), PKG_VERSION);
    }
    
    /// Print the input file info.
    pub fn print_input(&self, input_path: &std::path::Path) {
        if self.verbosity != Verbosity::Normal {
            return;
        }
        eprintln!("  {} Input: {}", "→".dimmed(), input_path.display());
    }
    
    /// Set the spinner message during processing.
    pub fn set_status(&self, msg: &str) {
        if let Some(spinner) = &self.spinner {
            spinner.set_message(msg.to_string());
        }
    }
    
    /// Finish the spinner (called before printing results).
    pub fn finish_spinner(&self) {
        if let Some(spinner) = &self.spinner {
            spinner.finish_and_clear();
        }
    }
    
    /// Display the final build result.
    pub fn display_result(&self, result: &BuildResult) {
        if self.verbosity != Verbosity::Normal {
            return;
        }
        
        // Print each target result
        for target_result in &result.target_results {
            let filename = target_result.output_file
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown");
            
            if target_result.success {
                eprintln!(
                    "  {} {} {} {}",
                    "✔".green().bold(),
                    target_result.target.to_string().dimmed(),
                    "→".dimmed(),
                    filename
                );
            } else {
                let err_msg = target_result.error_message.as_deref().unwrap_or("unknown error");
                eprintln!(
                    "  {} {} {} {}",
                    "✘".red().bold(),
                    target_result.target.to_string().dimmed(),
                    "→".dimmed(),
                    err_msg.red()
                );
            }
        }
        
        // Print summary line
        let elapsed = result.duration;
        let target_count = result.target_results.len();
        let success_count = result.success_count();
        
        if result.is_success() {
            eprintln!(
                "{} Built {} {} in {:.2?}",
                "✔".green().bold(),
                target_count,
                if target_count == 1 { "target" } else { "targets" },
                elapsed
            );
        } else {
            eprintln!(
                "{} Built {}/{} {} in {:.2?}",
                "✘".red().bold(),
                success_count,
                target_count,
                if target_count == 1 { "target" } else { "targets" },
                elapsed
            );
        }
    }
    
    /// Display an early error (before build starts).
    /// Shown in Normal and Quiet modes, suppressed only in Silent mode.
    pub fn display_error(&self, msg: &str) {
        self.finish_spinner();
        if self.verbosity != Verbosity::Silent {
            eprintln!("{} {}", "error:".red().bold(), msg);
        }
    }
}
