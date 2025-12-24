//! Progress logging with a spinner and progress bar.

use std::time::Duration;

use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};

pub struct Logger {
    pb: Option<ProgressBar>,
}

impl Logger {
    pub fn new(total_steps: u64, silent: bool) -> Self {
        if silent {
            return Self { pb: None };
        }

        let style = ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("#>-");

        let pb = ProgressBar::new(total_steps);
        pb.set_style(style);
        pb.enable_steady_tick(Duration::from_millis(100));

        Self { pb: Some(pb) }
    }

    pub fn step(&self, msg: &str) {
        if let Some(pb) = &self.pb {
            pb.set_message(msg.to_string());
            pb.inc(1);
        }
    }

    pub fn finish(self, msg: &str) {
        if let Some(pb) = &self.pb {
            let elapsed = pb.elapsed();
            pb.finish_and_clear();
            eprintln!("{} {} ({:.2?})", "✔".green().bold(), msg, elapsed);
        }
    }

    pub fn fail(&self, msg: &str) {
        if let Some(pb) = &self.pb {
            pb.finish_and_clear();
        }
        eprintln!("{} {}", "ERROR:".red().bold(), msg);
    }
}