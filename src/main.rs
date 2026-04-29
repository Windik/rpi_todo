mod error;

use std::io;
use std::io::{BufReader, Write};
use std::fs;
use std::fs::File;

use rpi_todo::tasks;
use tasks::{TodoList, Task};
use clap::{Parser, Subcommand};
use colored::*;
use fluent_templates::static_loader;
use serde::{Serialize, Deserialize};

use crate::error::TodoError;

const FILE_PATH: &str = "tasks.json";

// ==== Config block ====
#[derive(Serialize, Deserialize, Debug)]
struct AppConfig {
	version: u8,
	language: String,
}

impl Default for AppConfig {
	fn default() -> Self {
		Self { version: 1, language:"en-US".into() }
	}
}

// ==== l18n block ==== 
static_loader! {
    static LOCALES = {
        locales: "locales",
        fallback_language: "en-US",
    };
}

// ==== Translator ====
pub struct Translator {
    pub lang: unic_langid::LanguageIdentifier,
}

impl Translator {
    pub fn new(lang_code: &str) -> Self {
        let lang = lang_code.parse().expect("Parsing language ID failed");
        Self { lang }
    }

    pub fn tr(&self, text_id: &str, args: &[(&str, &str)]) -> String {
        use fluent_templates::Loader;
        use std::collections::HashMap;

        if args.is_empty() {
            return LOCALES.lookup(&self.lang, text_id)
                .unwrap_or_else(|| text_id.to_string());
        }

        let mut fluent_args = HashMap::new();
        for (key, value) in args {
            fluent_args.insert(key.to_string(), (*value).into());
        }

        LOCALES.lookup_with_args(&self.lang, text_id, &fluent_args)
            .unwrap_or_else(|| text_id.to_string())
    }
}

fn locale_exists(lang: &str) -> bool {
	let path = format!("locales/{}", lang);

	fs::metadata(path).is_ok()
}


// ==== Save tasks data block ====
fn save_tasks(list: &TodoList) -> Result<(), TodoError> { 
    let file = File::create(FILE_PATH)?;
    serde_json::to_writer_pretty(file, &list.tasks)?;
    Ok(())
}

fn load_tasks() -> Result<Vec<Task>, TodoError> {
    let file = File::open(FILE_PATH)?;
    let tasks = serde_json::from_reader(file)?;
    Ok(tasks)
}

#[derive(Parser)]
#[command(name = "rpi_todo")]
#[command(about = "A simple Todo List CLI in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Clone)]
enum Commands {
    Add { title: String },
    List,
    Done { id: u32 },
    Delete { id: u32 },
    Config { lang: String },
}

// Add Translator
fn handle_command(command: Commands, list: &mut TodoList, t: &Translator) -> bool {
    match command {
        Commands::Add { title } => {
            list.add_task(title.clone());
            println!("{}", t.tr("task-added", &[("title", &title)]));
            true
        },
        Commands::List => {
            if list.tasks.is_empty() {
                println!("{}", t.tr("list-empty", &[]));
            } else {
                for task in &list.tasks {
                    let status_text = if task.completed { t.tr("status-done", &[]) } else { t.tr("status-pending", &[]) };
                    let status = if task.completed { status_text.green() } else { status_text.yellow() };
                    let date = task.created_at.format("%Y-%m-%d %H:%M").to_string().dimmed();
                    println!("{}. {} - [{}] {}", task.id, date, status, task.title.bold());
                }
            }
            false
        },
        Commands::Delete { id } => {
            let id_str = id.to_string();
            if list.delete_task(id) {
                println!("{}", t.tr("task-deleted", &[("id", &id_str)]));
                true
            } else {
                println!("{}", t.tr("error-not-found", &[]).red());
                false
            }
        },
        Commands::Done { id } => {
            let id_str = id.to_string();
            if list.complete_task(id) {
                println!("{}", t.tr("task-completed", &[("id", &id_str)]));
                true
            } else {
                println!("{}", t.tr("error-not-found", &[]).red());
                false
            }
        },
        Commands::Config { lang } => {
            if locale_exists(&lang) {
                let mut new_cfg = AppConfig::default();
                new_cfg.language = lang.clone();
                if confy::store("rpi_todo", None, new_cfg).is_ok() {
                    println!("{}", t.tr("lang-changed", &[("lang", &lang)]));
                    println!("{}", t.tr("restart-hint", &[]));
                    return true;
                }
            } else {
                println!("{}", t.tr("error-lang-not-found", &[("lang", &lang)]).red());
            }
            false
        },
    }
}

fn active_mod(list: &mut TodoList, t: &Translator) {
    println!("{}", t.tr("active-mod-greeting", &[]));
    
    loop {
        print!("> ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() { continue; }
        if input == "exit" { break; }
        
        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        let cmd_part = parts[0];
        let arg_part = parts.get(1).copied();

        let command_to_run = match cmd_part {
            "add" => arg_part.map(|title| Commands::Add { title: title.to_string() }),
            "list" => Some(Commands::List),
            "done" => arg_part.and_then(|id| id.parse().ok().map(|id| Commands::Done { id })),
            "delete" => arg_part.and_then(|id| id.parse().ok().map(|id| Commands::Delete { id })),
            "config" => arg_part.map(|l| Commands::Config { lang: l.to_string() }),
            "help" => { 
                println!("{}", t.tr("help-text", &[])); 
                None 
            },
            _ => { 
                println!("{}", t.tr("error-unknown-cmd", &[("cmd", cmd_part)]).red()); 
                None 
            }
        };

        if let Some(cmd) = command_to_run {
            if handle_command(cmd, list, t) {
                if let Err(e) = save_tasks(list) {
                    eprintln!("Error saving tasks: {}", e);
                }
            }
        }
    }
}

fn main() -> Result<(), TodoError> {

	let cfg: AppConfig = confy::load("rpi_todo", None)?; 
    
    let cli = Cli::parse();

	let tasks_vec = load_tasks().unwrap_or_else(|_| Vec::new());
    let mut list = TodoList { tasks: tasks_vec };

    let translator = Translator::new(&cfg.language);

    match cli.command {
        Some(cmd) => {
            if handle_command(cmd, &mut list, &translator) {
                save_tasks(&list)?;
            }
        }
        None => active_mod(&mut list, &translator),
    }

    Ok(())
}
