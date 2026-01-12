# ts-scaffold

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

This will prompt you for:
1. Whether to enable TypeScript strict mode
2. Whether to initialize a git repository

### What Gets Generated

The tool creates a complete TypeScript project structure:

```
my-project/
├── src/
│   └── main.ts          # Hello world entry point
├── tsconfig.json        # TypeScript configuration
├── package.json         # Project metadata and dependencies
└── .gitignore           # Git ignore rules
```

If git initialization is enabled, a `.git` directory is also created.

## Features

### Strict Mode

When enabled, adds the following options to `tsconfig.json`:
- `"strict": true`
- `"noUncheckedIndexedAccess": true`
- `"noImplicitOverride": true`

### Git Integration

Optionally initializes a git repository in the project directory. If a `.git` folder already exists, this step is skipped. 

Creates a `.gitignore` file with sensible defaults for Node.js/TypeScript projects.

### Package.json

Generates a `package.json` with:
- Project name matching your input
- TypeScript as a dev dependency
- Build and run scripts configured
