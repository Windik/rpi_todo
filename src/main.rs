use std::env; // environment module
use std::io;
use std::io::{BufReader, Write};
use std::fs::File;

use rpi_todo::tasks;
use tasks::TodoList;


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

fn handle_command(command: &str, arg: Option<&str>, list: &mut TodoList) -> bool {
	match command {
		"add" => {
			if let Some(title) = arg {
				list.add_task(title.to_string());
				println!("Task added!");
				
				true
			} else {
				println!("Error: 'add' requires a task title.");
				
				false
			}
		},
		"list" => {
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
		"delete" => {
			if let Some(id_str) = arg {
				if let Ok(id) = id_str.parse::<u32>() {
					if list.delete_task(id){
						println!("Task #{} deleted.", id);
						return true;
					} else {
						println!("Task not found.");
					}
				}
			}

			false
		},
		"done" => {
			if let Some(id_str) = arg {
				if let Ok(id) = id_str.parse::<u32>() {
					if list.complete_task(id) {
						println!("Task #{} completed.", id);
						return true;
					} else {
						println!("Task not found.");
					}
				}
			}

			false
		},
		"help" => {
			print_help();

			false
		},
		_ => {
			println!("Unknown command. Try 'help' for info.");

			false
		}	
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

fn args_mod(args: &[String], list: &mut TodoList) {
	let command = &args[1];
	let arg = args.get(2).map(|s| s.as_str());

	if handle_command(command, arg, list) {
		let _ = save_tasks(list);
	}
}

fn active_mod(list: &mut TodoList) {
	println!("Interactive mode. Type 'exit' to quit.");
	
	loop {
        print!("> "); // Красивый пригласительный знак
        io::stdout().flush().unwrap(); // Чтобы '>' отобразился сразу

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let input = input.trim(); // Убираем лишние пробелы и символ новой строки

        if input.is_empty() { continue; }
        
        // Разделяем ввод на команду и аргументы
        if input == "exit" { break; }
        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        let command = parts[0];
        let arg = parts.get(1).copied();

        if handle_command(command, arg, list) {
        	let _ = save_tasks(list);
        }
    }
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let mut list = load_tasks();

	if args.len() > 1 {
		args_mod(&args, &mut list);
	} 
	else {
		active_mod(&mut list);
	}
}
