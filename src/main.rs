const TSCONFIG_TEMPLATE: &str = include_str!("../templates/tsconfig.json");

use clap::Parser;
use dialoguer::Confirm;
use serde_json::{Value, json};
use std::{fs, path::Path};

#[derive(Parser)]
#[command(name = "ts-scaffold")]
#[command(about = "A TypeScript project scaffolding tool")]
struct Args {
    project_name: String,
}

fn run_ts_scaffold(args: Args) {
    let mut tsconfig: Value =
        serde_json::from_str(TSCONFIG_TEMPLATE).expect("Failed to parse tsconfig template");

    let use_strict = Confirm::new()
        .with_prompt("Use strict TypeScript mode?")
        .interact()
        .unwrap();

    if use_strict && let Some(compiler_options) = tsconfig.get_mut("compilerOptions") {
        compiler_options["strict"] = json!(true);
        compiler_options["noUncheckedIndexedAccess"] = json!(true);
        compiler_options["noImplicitOverride"] = json!(true);
    }

    let tsconfig_path = format!("{}/tsconfig.json", args.project_name);
    let tsconfig_str =
        serde_json::to_string_pretty(&tsconfig).expect("Failed to serialize tsconfig.json");

    fs::write(&tsconfig_path, tsconfig_str).expect("Failed to write tsconfig.json");
}

fn main() {
    let args = Args::parse();

    if !Path::new(&args.project_name).exists() {
        fs::create_dir(&args.project_name).expect("Failed to create project directory");
    }

    run_ts_scaffold(args);
}
