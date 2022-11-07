//! Useful path validation functions.

use crate::EXIT_FAILURE;
use std::{ops::Not, path::PathBuf, process::exit};

/// Validate that a path points to an existing directory.
///
/// # Examples
///
/// ```no_run
/// let path = PathBuf::from("target");
/// validate_path_to_directory(&path, "Path to build artefacts");
/// ```
pub fn validate_path_to_directory(path: &PathBuf, description: &str) {
    if path.exists().not() {
        println!(
            "cargo:warning={} does not exists. Stopping build.",
            description
        );
        exit(EXIT_FAILURE);
    }
    if path.is_dir().not() {
        println!(
            "cargo:warning={} does not point to a valid directory. Stopping build.",
            description
        );
        exit(EXIT_FAILURE);
    }
}

/// Validate that a path points to an existing file.
///
/// # Examples
///
/// ```no_run
/// let path = PathBuf::from("Cargo.toml");
/// validate_path_to_file(&path, "Path to Cargo.toml");
/// ```
pub fn validate_path_to_file(path: &PathBuf, description: &str) {
    if path.exists().not() {
        println!(
            "cargo:warning={} does not exists. Stopping build.",
            description
        );
        exit(EXIT_FAILURE);
    }
    if path.is_file().not() {
        println!(
            "cargo:warning={} does not point to a valid file. Stopping build.",
            description
        );
        exit(EXIT_FAILURE);
    }
}
