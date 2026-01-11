// const TSCONFIG_TEMPLATE: &str = include_str!("../templates/tsconfig.json");

use clap::Parser;
use dialoguer::Confirm;

#[derive(Parser)]
#[command(name = "ts-scaffold")]
#[command(about = "A TypeScript project scaffolding tool")]
struct Args {
    project_name: String,
}

fn run_ts_scaffold(args: Args) {
    let should_install = Confirm::new()
        .with_prompt("Use strict TypeScript mode?")
        .interact()
        .unwrap();

    println!(
        "Create ts files for {}, strict mode: {}",
        args.project_name, should_install
    );
}

fn main() {
    let args = Args::parse();

    run_ts_scaffold(args);
}
