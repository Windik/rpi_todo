use std::io;
use std::io::{BufReader, Write};
use std::fs::File;

use rpi_todo::tasks;
use tasks::TodoList;
use clap::{Parser, Subcommand};

const FILE_PATH: &str = "tasks.json";

fn save_tasks(list: &TodoList) -> io::Result<()> {
	let file = File::create(FILE_PATH)?;
	serde_json::to_writer_pretty(file, &list.tasks)?;
	Ok(())
}

fn load_tasks() -> TodoList {
	let file = File::open(FILE_PATH);

	match file {
		Ok(f) => {
			let reader = BufReader::new(f);

			let tasks_vec  = serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new());

			TodoList { tasks: tasks_vec }
		},
		Err(_) => TodoList::new(),
	}
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
	/// Add a new task
	Add { title: String },
	/// List all tasks
	List,
	/// Mark a task as done
	Done { id: u32 },
	/// Delete a task
	Delete { id: u32 },
}

fn handle_command(command: Commands, list: &mut TodoList) -> bool {
	match command {
		Commands::Add { title } => {
			list.add_task(title);
			println!("Task added!");
			true
		},
		Commands::List => {
			if list.tasks.is_empty() {
				println!("Your list is empty.");
			} else  {
				for t in  &list.tasks {
					let status = if t.completed { "✅" } else { "⏳" };
            		println!("{}. [{}] {}", t.id, status, t.title);
				}
			}
			false
		},
		Commands::Delete { id } => {
			if list.delete_task(id) {
				println!("Task #{} deleted.", id);
				true
			} else {
				println!("Task not found.");
				false
			}
		},
		Commands::Done { id } => {
			if list.complete_task(id) {
				println!("Task #{} completed.", id);
				true
			} else {
				println!("Task not found.");
				false
			}
		},
	}
}

fn print_help() {
	println!("Available commands:");
    println!("  add <text>    - Add a new task");
    println!("  list          - Show all tasks");
    println!("  delete <id>   - Remove task by ID");
    println!("  done <id>     - Mark task as completed");
    println!("  help          - Show this help");
}

fn active_mod(list: &mut TodoList) {
	println!("Interactive mode. Type 'exit' to quit or 'help' for commands.");
	
	loop {
        print!("> "); // Красивый пригласительный знак
        io::stdout().flush().unwrap(); // Чтобы '>' отобразился сразу

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let input = input.trim(); // Убираем лишние пробелы и символ новой строки

        if input.is_empty() { continue; }
        if input == "exit" { break; }
        
        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        let cmd_part = parts[0];
        let arg_part = parts.get(1).copied();

        let command_to_run = match cmd_part {
        	"add" => arg_part.map(|t| Commands::Add { title: t.to_string() }),
        	"list" => Some(Commands::List),
        	"done" => arg_part.and_then(|id| id.parse().ok().map(|id| Commands::Done { id })),
        	"delete" => arg_part.and_then(|id| id.parse().ok().map(|id| Commands::Delete { id })),
        	"help" => { print_help(); None },
        	_ => { println!("Unknown command:{}. Type 'help' for info.", cmd_part); None }
        };

		if let Some(cmd) = command_to_run {
			if handle_command(cmd, list) {
				if let Err(e) = save_tasks(list) {
					eprintln!("Error saving tasks: {}", e);
				}
			}
		} else if cmd_part != "help" {
			println!("Invalid arguments for '{}'.", cmd_part);
		}
    }
}

fn main() -> io::Result<()> {
	let cli = Cli::parse();
	let mut list = load_tasks();

	match cli.command {
		Some(cmd) => {
			if handle_command(cmd, &mut list) {
				save_tasks(&list)?;
			}
		}

		None => active_mod(&mut list),
	}

	Ok(())
}
