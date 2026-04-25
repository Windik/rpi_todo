# 🦀 rpi_todo: Your Personal Rust-Powered Task Manager

Welcome! **rpi_todo** is a lightweight Command Line Interface (CLI) application for managing your daily tasks. What started as a learning project on a Raspberry Pi has evolved into a robust tool featuring multi-language support, persistent storage, and a sleek interactive mode.

## ✨ Features

- **Dual Operation Modes:** 
  - ⚡️ **One-shot CLI:** Execute commands directly from your terminal.
  - 💬 **Interactive Shell:** Launch the app without arguments to enter a dedicated command loop.
- **Global by Design (i18n):** Native support for multiple languages including English, Russian, and German (more coming soon!).
- **Smart Persistence:** Tasks are automatically saved to and loaded from `tasks.json` using the powerful `serde` framework.
- **Visual Feedback:** Track your progress with clear status indicators (✅/⏳) and terminal colors.
- **Persistent Configuration:** Automatically manages your preferences (like chosen language) in a dedicated config file.

## 🚀 Quick Start

Ensure you have [Rust and Cargo](https://rustup.rs) installed.

1. **Clone the repository:**
   ```bash
   git clone https://github.com
   cd rpi_todo
   ```

2. **Build the project:**
   ```bash
   cargo build --release
   ```

## 🛠 Usage

### CLI Argument Mode
- **Add a task:** `cargo run -- add "Order more Raspberry Pis"`
- **List all tasks:** `cargo run -- list`
- **Complete a task:** `cargo run -- done 1`
- **Delete a task:** `cargo run -- delete 1`

### Interactive Mode
Simply run the application without any arguments:
```bash
cargo run
```
Inside the interactive shell, you can use `add`, `list`, `done`, `delete`, and the `config` command.

## 🌍 Localization & Config

The application remembers your settings across sessions.

- **Change Language:** 
  In interactive mode, type: `config ru`, `config en-US`, or `config de`.
- **Configuration Path:**
  On Linux, your settings are typically stored at: `~/.config/rpi_todo/default-config.toml`.

## 📦 Technical Stack

Built with the best tools in the Rust ecosystem:
- **Core:** Rust 🦀
- **CLI Parsing:** `clap`
- **Localization:** `fluent` (by Mozilla)
- **Config Management:** `confy`
- **Terminal Styling:** `colored`

---
*Developed with a passion for clean code and the Raspberry Pi community.* 🍓
