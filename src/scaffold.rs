//! Core scaffolding logic for creating TypeScript projects.
//!
//! Handles project directory creation, file generation, and git initialization.
use crate::Args;
use crate::templates::*;
use std::{fs, path::Path};

/// Runs the TypeScript scaffolding process.
///
/// Creates a new TypeScript project with the specified configuration,
/// including tsconfig.json, package.json, source files, and optional
/// git initialization.
///
/// # Arguments
///
/// * `args` - The command-line arguments containing the project name
pub fn run(args: &Args) {
    if !Path::new(&args.project_name).exists() {
        fs::create_dir(&args.project_name).expect("Failed to create project directory");
    }

    check_git(args);
    create_tsconfig_file(args);
    create_package_file(args);
    create_source_file(args);
}
