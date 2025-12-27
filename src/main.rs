use std::process::ExitCode;

use clap::Parser;

use rbxts_bundler::bundler;
use rbxts_bundler::cli::{Cli, Commands};
use rbxts_bundler::logging::BuildUI;

fn main() -> ExitCode {
    let Cli { command } = Cli::parse();
    match command {
        Commands::Build(args) => {
            let ui = BuildUI::new(args.verbosity());
            
            ui.print_header();
            ui.print_input(&args.input);
            ui.set_status("Building targets...");
            
            // Convert CLI args to library BuildConfig
            let config = match args.to_build_config() {
                Ok(config) => config,
                Err(e) => {
                    ui.display_error(&format!("{e:#}"));
                    return ExitCode::FAILURE;
                }
            };
            
            match bundler::build(&config) {
                Ok(result) => {
                    ui.finish_spinner();
                    ui.display_result(&result);
                    
                    if result.is_success() {
                        ExitCode::SUCCESS
                    } else {
                        ExitCode::FAILURE
                    }
                }
                Err(e) => {
                    ui.display_error(&format!("{e:#}"));
                    ExitCode::FAILURE
                }
            }
        }
    }
}
