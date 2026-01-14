//! Template generation functions for project files.
//!
//! Provides functions to create tsconfig.json, package.json, .gitignore,
//! and TypeScript source files with configurable options.
const TSCONFIG_TEMPLATE: &str = include_str!("../templates/tsconfig.json");
const PACKAGE_TEMPLATE: &str = include_str!("../templates/package.json");
const GITIGNORE_TEMPLATE: &str = include_str!("../templates/gitignore");
const TS_MAIN_TEMPLATE: &str = include_str!("../templates/main.ts");

use crate::Args;
use dialoguer::Confirm;
use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

/// Initializes the package.json by setting the project name.
///
/// # Arguments
///
/// * `package_json` - A mutable reference to the package.json JSON value
/// * `args` - The command-line arguments containing the project name
fn init_package_json(package_json: &mut Value, args: &Args) {
    package_json["name"] = json!(args.project_name);
}

/// Prompts the user to enable TypeScript strict mode.
///
/// If enabled, adds the following options to `compilerOptions`:
/// - `strict: true`
/// - `noUncheckedIndexedAccess: true`
/// - `noImplicitOverride: true`
///
/// # Arguments
///
/// * `tsconfig` - A mutable reference to the tsconfig JSON value
fn check_strict(tsconfig: &mut Value) {
    let use_strict = Confirm::new()
        .with_prompt("Use strict TypeScript mode?")
        .interact()
        .unwrap();

    if use_strict && let Some(compiler_options) = tsconfig.get_mut("compilerOptions") {
        compiler_options["strict"] = json!(true);
        compiler_options["noUncheckedIndexedAccess"] = json!(true);
        compiler_options["noImplicitOverride"] = json!(true);
    }
}

/// Checks if the user wants to initialize a git repository.
///
/// If the project folder already has a `.git` directory, this step is skipped.
/// When the user confirms, executes `git init` and creates a `.gitignore` file.
///
/// # Arguments
///
/// * `args` - The command-line arguments containing the project name
pub fn check_git(args: &Args) {
    if Path::new(&format!("{}/.git", args.project_name)).exists() {
        return;
    }

    let init_git = Confirm::new()
        .with_prompt("Initialize git repository?")
        .default(true)
        .interact()
        .unwrap();

    if init_git {
        Command::new("git")
            .arg("init")
            .current_dir(&args.project_name)
            .output()
            .expect("Failed to execute git init");

        let gitignore_path = format!("{}/.gitignore", args.project_name);
        fs::write(&gitignore_path, GITIGNORE_TEMPLATE).expect("Failed to write gitignore");
    }
}

/// Create the `tsconfig.json` file on the project folder with the correspondant values.
///
/// # Arguments
///
/// * `args` - A reference to the command-line arguments containing the project name
pub fn create_tsconfig_file(args: &Args) {
    let mut tsconfig: Value =
        serde_json::from_str(TSCONFIG_TEMPLATE).expect("Failed to parse tsconfig template");

    check_strict(&mut tsconfig);

    let tsconfig_path = format!("{}/tsconfig.json", args.project_name);

    let tsconfig_str =
        serde_json::to_string_pretty(&tsconfig).expect("Failed to serialize tsconfig.json");

    fs::write(&tsconfig_path, tsconfig_str).expect("Failed to write tsconfig.json");
}

/// Create the `package.json` file on the project folder with the correspondant values.
///
/// # Arguments
///
/// * `args` - A reference to the command-line arguments containing the project name
pub fn create_package_file(args: &Args) {
    let mut package_json: Value =
        serde_json::from_str(PACKAGE_TEMPLATE).expect("Failed to parse package template");

    let package_json_path = format!("{}/package.json", args.project_name);

    init_package_json(&mut package_json, args);

    let package_json_str =
        serde_json::to_string_pretty(&package_json).expect("Failed to serialize package.json");

    fs::write(&package_json_path, package_json_str).expect("Failed to write package.json");
}

/// Create the `main.ts` file on the source folder inside the project.
/// In case that the folder already exists, we'll skip this step
///
/// # Arguments
///
/// * `args` - A reference to the command-line arguments containing the project name
pub fn create_source_file(args: &Args) {
    let src_path = format!("{}/src", args.project_name);
    if !Path::new(&src_path).exists() {
        fs::create_dir(src_path).expect("Failed to create source directory");
        let main_path = format!("{}/src/main.ts", args.project_name);
        fs::write(&main_path, TS_MAIN_TEMPLATE).expect("Failed to write main.ts");
    }
}
