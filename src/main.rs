use std::env; // environment module
use std::io;
use std::io::{BufReader, Write};
use std::fs::File;

use rpi_todo::tasks;
use tasks::TodoList;


const FILE_PATH: &str = "tasks.json";

fn save_tasks(list: &TodoList){
	let file = File::create(FILE_PATH).expect("Failed to create file");
	serde_json::to_writer_pretty(file, &list.tasks).expect("Failed to write JSON");
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

fn args_mod(args: &[String], list: &mut TodoList) {
	if args.len() < 2 { return; }
	let command = &args[1];
	
	match command.as_str() {
		"add" => {
			if let Some(title) = args.get(2) {
				list.add_task(title.to_string());
				println!("Task added!");
			}
		},
		"delete" => {
			if let Some(id_str) = args.get(2) {
				if let Ok(id) = id_str.parse::<u32>() {
					if list.delete_task(id) {
						println!("Task #{} deleted.", id);
					} else {
						println!("Task not found.");
					}
				}
			}
		},
		"list" => {
			for t in &list.tasks {
				let status = if t.completed { "✅" } else { "⏳" };

				println!("{}. [{}] {}", t.id, status, t.title);
			}
		},
		"done" => {
			if let Some(id_str) = args.get(2) {
				if let Ok(id) = id_str.parse::<u32>() {
					if list.complete_task(id) {
						println!("Task #{} completed.", id);
					} else {
						println!("Task not found.");
					}
				}
			}
		},
		"help" => {
			println!("-- add 'task text'- добавление новой задачи с заголовком 'task text'");
			println!("-- list - добавление новой задачи с заголовком 'task text'");
			println!("-- delete 1 - удаление задачи по номеру");
			println!("-- done 1  - установки отметки 'выполнено' для задачи с номером  1");
			println!("-- help - отображение справки по работе утилиты");
		},
		_ => {
			println!("Unknown command ");
		}
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
        
        // Разделяем ввод на команду и аргументы
        if input == "exit" { break; }
        let parts: Vec<&str> = input.splitn(2, ' ').collect();

        match parts[0] {
            "add" => {
                if let Some(title) = parts.get(1) {
                	list.add_task(title.to_string());
                	save_tasks(list);
                	println!("Added and saved.");
                }
            },
            "list" => {
                for t in &list.tasks {
                    println!("{}. {}", t.id, t.title);
                }
            },
            "delete" => {
				if let Some(id_str) = parts.get(1) {
					if let Ok(id) = id_str.parse::<u32>() {
						if list.delete_task(id) {
							save_tasks(list);
							println!("Deleted.");
						}
					}
				}
            },
            "done" => {
            	if let Some(id_str) = parts.get(1) {
            		if let Ok(id) = id_str.parse::<u32>() {
            			if list.complete_task(id) {
            				save_tasks(list);
            				println!("Task #{} completed.", id);
            			} else {
            				println!("Task not found.");
            			}
            		}
            	}	
            },
            "help" => {
            	println!("add 'task text'- добавление новой задачи с заголовком 'task text'");
            	println!("list - добавление новой задачи с заголовком 'task text'");
            	println!("delete 1 - удаление задачи по номеру");
            	println!("done 1  - установки отметки 'выполнено' для задачи с номером  1");
            	println!("help - отображение справки по работе утилиты");
            },
            "exit" => break, // Выход из цикла loop
            _ => println!("Unknown command. Try to use 'add', 'list' or 'exit'.")
        }
    }
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let mut list = load_tasks();

	if args.len() > 1 {
		args_mod(&args, &mut list);
		save_tasks(&list);
	} 
	else {
		active_mod(&mut list);
	}

	
}
