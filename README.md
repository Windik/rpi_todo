# Rust Todo CLI

A lightweight Command Line Interface (CLI) application for managing tasks, built as a learning project on Raspberry Pi.

## Features
- **Dual Mode:** Works both with direct CLI arguments and in an interactive shell mode.
- **Persistence:** Automatically saves and loads tasks from a `tasks.json` file using `serde`.
- **Status Tracking:** Mark tasks as completed with visual feedback (✅/⏳).

## Installation
Ensure you have [Rust and Cargo](https://rustup.rs) installed.

1. Clone the repository:
   ```bash
   git clone https://github.com
   cd rpi_todo
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

## Usage
CLI Arguments Mode
- Add a task: ```cargo run -- add "Buy milk"```
- List tasks: ```cargo run -- list```
- Complete a task: ```cargo run -- done 1```

Interactive Mode
Just run the application without arguments: ```cargo run```

## Technical Stack
- **Language:** Rust
- **Serialization:** Serde & Serde-JSON
- **Environment:** Linux