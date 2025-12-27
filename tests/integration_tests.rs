//! Integration tests for the bundler API and CLI.

use std::path::PathBuf;
use std::process::Command;

use rbxts_bundler::bundler::{build, BuildConfig, Target};

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

fn output_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/output")
}

fn test_rbxm() -> PathBuf {
    fixtures_dir().join("build.rbxm")
}

fn cli_binary() -> PathBuf {
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target");
    let debug = base.join("debug/rbxts-bundler");
    let release = base.join("release/rbxts-bundler");

    if debug.exists() {
        debug
    } else if release.exists() {
        release
    } else {
        panic!("CLI binary not found. Run `cargo build` first.");
    }
}

mod api {
    use super::*;

    mod single_target {
        use super::*;

        #[test]
        fn dev() {
            let config = BuildConfig::new(test_rbxm(), output_dir())
                .with_targets(vec![Target::Dev]);
            let result = build(&config).unwrap();

            assert!(result.is_success());
            assert_eq!(result.target_results.len(), 1);
            assert_eq!(result.target_results[0].target, Target::Dev);
            assert!(result.target_results[0].output_file.exists());
        }

        #[test]
        fn rel() {
            let config = BuildConfig::new(test_rbxm(), output_dir())
                .with_targets(vec![Target::Rel]);
            let result = build(&config).unwrap();

            assert!(result.is_success());
            assert_eq!(result.target_results[0].target, Target::Rel);
            assert!(result.target_results[0].output_file.exists());
        }
    }

    mod multiple_targets {
        use super::*;

        #[test]
        fn all_targets() {
            let targets = vec![Target::Dev, Target::Rel, Target::DevCompat, Target::RelCompat];
            let config = BuildConfig::new(test_rbxm(), output_dir())
                .with_targets(targets.clone());
            let result = build(&config).unwrap();

            assert!(result.is_success());
            assert_eq!(result.target_results.len(), targets.len());

            for r in &result.target_results {
                assert!(r.output_file.exists(), "{:?} missing", r.target);
            }
        }

        #[test]
        fn compat_extension() {
            let config = BuildConfig::new(test_rbxm(), output_dir())
                .with_targets(vec![Target::DevCompat, Target::RelCompat]);
            let result = build(&config).unwrap();

            for r in &result.target_results {
                let name = r.output_file.file_name().unwrap().to_str().unwrap();
                assert!(name.ends_with(".c.lua"), "expected .c.lua: {name}");
            }
        }
    }

    mod output {
        use super::*;

        #[test]
        fn has_duration() {
            let config = BuildConfig::new(test_rbxm(), output_dir())
                .with_targets(vec![Target::Dev]);
            let result = build(&config).unwrap();

            assert!(!result.duration.is_zero());
        }

        #[test]
        fn valid_lua() {
            let config = BuildConfig::new(test_rbxm(), output_dir())
                .with_targets(vec![Target::Dev]);
            let result = build(&config).unwrap();

            for r in &result.target_results {
                let content = std::fs::read_to_string(&r.output_file).unwrap();
                assert!(!content.is_empty());
                assert!(content.contains("return"));
            }
        }
    }

    mod errors {
        use super::*;

        #[test]
        fn nonexistent_input() {
            let config = BuildConfig::new(PathBuf::from("nonexistent.rbxm"), output_dir())
                .with_targets(vec![Target::Dev]);
            assert!(build(&config).is_err());
        }

        #[test]
        fn empty_targets() {
            let config = BuildConfig::new(test_rbxm(), output_dir())
                .with_targets(vec![]);
            assert!(build(&config).is_err());
        }
    }
}

mod cli {
    use super::*;

    mod info {
        use super::*;

        #[test]
        fn help() {
            let out = Command::new(cli_binary()).arg("--help").output().unwrap();
            assert!(out.status.success());
            assert!(String::from_utf8_lossy(&out.stdout).contains("rbxts-bundler"));
        }

        #[test]
        fn version() {
            let out = Command::new(cli_binary()).arg("--version").output().unwrap();
            assert!(out.status.success());
        }
    }

    mod build_cmd {
        use super::*;

        #[test]
        fn single_target() {
            let out = Command::new(cli_binary())
                .args([
                    "build", test_rbxm().to_str().unwrap(),
                    "--out-dir", output_dir().to_str().unwrap(),
                    "-t", "dev",
                ])
                .output()
                .unwrap();

            assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
        }

        #[test]
        fn multiple_targets() {
            let out = Command::new(cli_binary())
                .args([
                    "build", test_rbxm().to_str().unwrap(),
                    "--out-dir", output_dir().to_str().unwrap(),
                    "-t", "dev", "-t", "rel", "-t", "dev-compat", "-t", "rel-compat",
                ])
                .output()
                .unwrap();

            assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
        }

        #[test]
        fn silent_mode() {
            let out = Command::new(cli_binary())
                .args([
                    "build", test_rbxm().to_str().unwrap(),
                    "--out-dir", output_dir().to_str().unwrap(),
                    "-t", "dev", "--silent",
                ])
                .output()
                .unwrap();

            assert!(out.status.success());
            assert!(String::from_utf8_lossy(&out.stdout).lines().count() <= 2);
        }
    }

    mod errors {
        use super::*;

        #[test]
        fn nonexistent_file() {
            let out = Command::new(cli_binary())
                .args([
                    "build", "nonexistent.rbxm",
                    "--out-dir", output_dir().to_str().unwrap(),
                    "-t", "dev",
                ])
                .output()
                .unwrap();

            assert!(!out.status.success());
        }

        #[test]
        fn invalid_target() {
            let out = Command::new(cli_binary())
                .args([
                    "build", test_rbxm().to_str().unwrap(),
                    "--out-dir", output_dir().to_str().unwrap(),
                    "-t", "invalid-target",
                ])
                .output()
                .unwrap();

            assert!(!out.status.success());
        }
    }
}
