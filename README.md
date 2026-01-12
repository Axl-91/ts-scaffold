# TS-scaffold CLI

A command-line tool for scaffolding TypeScript projects written in Rust. Creates new TypeScript projects with customizable configuration including tsconfig.json, project structure, and optional strict mode settings.

Inspired by website: https://tsconfig.guide/

## Installation

```bash
cargo install --path .
```

## Usage

```bash
ts-scaffold <project_name>
```

Creates a new TypeScript project with `tsconfig.json` and basic structure. You will be prompted to choose strict TypeScript mode.

## Strict Mode

When enabled, adds the following to `tsconfig.json`:
- `"strict": true`
- `"noUncheckedIndexedAccess": true`
- `"noImplicitOverride": true`

## Development

```bash
cargo build
cargo run -- my-test-project
```
