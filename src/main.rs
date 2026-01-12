// Templates that we are going to use for the scaffolding
const TSCONFIG_TEMPLATE: &str = include_str!("../templates/tsconfig.json");
const PACKAGE_TEMPLATE: &str = include_str!("../templates/package.json");
const GITIGNORE_TEMPLATE: &str = include_str!("../templates/gitignore");
const TS_MAIN_TEMPLATE: &str = include_str!("../templates/main.ts");

use clap::Parser;
use dialoguer::Confirm;
use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[derive(Parser)]
#[command(name = "ts-scaffold")]
#[command(about = "A TypeScript project scaffolding tool")]
struct Args {
    project_name: String,
}

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
fn check_git(args: &Args) {
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

/// Runs the TypeScript scaffolding process.
///
/// Creates a new TypeScript project with the specified configuration,
/// including tsconfig.json, package.json, source files, and optional
/// git initialization.
///
/// # Arguments
///
/// * `args` - The command-line arguments containing the project name
fn run_ts_scaffold(args: Args) {
    // Parse files that might need changes
    let mut tsconfig: Value =
        serde_json::from_str(TSCONFIG_TEMPLATE).expect("Failed to parse tsconfig template");
    let mut package_json: Value =
        serde_json::from_str(PACKAGE_TEMPLATE).expect("Failed to parse package template");

    init_package_json(&mut package_json, &args);
    check_git(&args);
    check_strict(&mut tsconfig);

    // Paths for files to create
    let tsconfig_path = format!("{}/tsconfig.json", args.project_name);
    let package_json_path = format!("{}/package.json", args.project_name);
    let main_path = format!("{}/main.ts", args.project_name);

    // serialize files that are in JSON
    let tsconfig_str =
        serde_json::to_string_pretty(&tsconfig).expect("Failed to serialize tsconfig.json");
    let package_json_str =
        serde_json::to_string_pretty(&package_json).expect("Failed to serialize package.json");

    // Write all the files on the project folder
    fs::write(&tsconfig_path, tsconfig_str).expect("Failed to write tsconfig.json");
    fs::write(&package_json_path, package_json_str).expect("Failed to write package.json");
    fs::write(&main_path, TS_MAIN_TEMPLATE).expect("Failed to write main.ts");
}

fn main() {
    let args = Args::parse();

    if !Path::new(&args.project_name).exists() {
        fs::create_dir(&args.project_name).expect("Failed to create project directory");
    }

    run_ts_scaffold(args);
}
