//! CLI entry point for ts-scaffold.
//!
//! Parses command-line arguments and initiates the scaffolding process.
use clap::Parser;

mod scaffold;
mod templates;

#[derive(Parser)]
#[command(name = "ts-scaffold")]
#[command(about = "A TypeScript project scaffolding tool")]
pub struct Args {
    pub project_name: String,
}

fn main() {
    let args = Args::parse();

    scaffold::run(&args);
}
