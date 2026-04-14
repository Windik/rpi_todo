use std::env; // environment module
use std::io;
use std::io::{BufReader, Write};
use std::fs::File;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
	id: u32,
	title: String,
	completed: bool,
}

const FILE_PATH: &str = "tasks.json";

fn save_tasks(tasks: &Vec<Task>){
	let file = File::create(FILE_PATH).expect("Не удалось создать файл!");
	serde_json::to_writer_pretty(file, tasks).expect("Ошибка при записи JSON");
}

fn load_tasks() -> Vec<Task> {
	let file = File::open(FILE_PATH);

	match file {
		Ok(f) => {
			let reader = BufReader::new(f);

			serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new())
		},
		Err(_) => Vec::new(),
	}
}

fn args_mod(args: &[String], tasks: &mut Vec<Task>) {
	let command = &args[1];
	
	match command.as_str() {
		"add" => {
			if args.len() < 3 {
				println!("Ошибка: введите текст задачи после 'add'");
			} else {
				let task_text = &args[2];
				println!("Добавляем задачу: {}", task_text);
				// Add save logic (to vec right now)
				let new_task: Task = Task {
					id: tasks.len() as u32,
					title: String::from(task_text),
					completed: false,
				};
				
				tasks.push(new_task);
			}
		}
		"list" => {
			println!("Список всех задач");
			// Add show logic later
			for i in tasks.iter() {
				println!("Задача #{} - {}, Статус - {}", i.id, i.title, i.completed);
			}
		},
		"done" => {
			if let Some(id_str) = args.get(2) {
				if let Ok(target_id) = id_str.parse::<u32>() {
				    // Используем iter_mut() для изменения элементов
				    for task in tasks.iter_mut() {
				        if task.id == target_id {
				            task.completed = true;
				        }
				    }
				}
			}
		},
		"help" => {
			println!("add 'task text'- добавление новой задачи с заголовком 'task text'");
			println!("list - добавление новой задачи с заголовком 'task text'");
			println!("--done 1  - установки отметки 'выполнено' для задачи с номером  1");
			println!("--help - отображение справки по работе утилиты");
		},
		_ => {
			println!("Неизвестная команда {}", command);
		}
	}
}

fn active_mod(tasks: &mut Vec<Task>) {
	loop {
        print!("> "); // Красивый пригласительный знак
        io::stdout().flush().unwrap(); // Чтобы '>' отобразился сразу

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Ошибка чтения");
        
        let input = input.trim(); // Убираем лишние пробелы и символ новой строки
        
        // Разделяем ввод на команду и аргументы
        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        let command = parts[0];

        match command {
            "add" => {
                if parts.len() < 2 {
                    println!("Ошибка: напишите текст задачи");
                } else {
                    let id = (tasks.len() + 1) as u32;
                    tasks.push(Task { id, title: parts[1].to_string(), completed: false });
                    println!("Задача добавлена!");
                    save_tasks(&tasks);
                    println!("Список задач сохранен!");
                }
            }
            "list" => {
                for t in tasks.iter() { // ИСПОЛЬЗУЕМ ССЫЛКУ &, чтобы не потерять список
                    let status = if t.completed { "✅" } else { "⏳" };
                    println!("{}. [{}] {}", t.id, status, t.title);
                }
            }
            "exit" => break, // Выход из цикла loop
            _ => println!("Неизвестная команда. Попробуйте 'add', 'list' или 'exit'."),
        }
    }
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let mut tasks: Vec<Task> = load_tasks();

	if args.len() > 1 {
		args_mod(&args, &mut tasks);
		save_tasks(&tasks);
	} 
	else {
		active_mod(&mut tasks);
	}

	
}
