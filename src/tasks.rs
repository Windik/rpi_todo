use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};
use colored::*; 

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
	pub id: u32,
	pub title: String,
	pub completed: bool,
	pub created_at: DateTime<Local>,
	pub tags: Vec<String>,
}

impl Task {
	/// Returns a formatted string of tags for console output. 
	/// If there are no tags, returns an empty string to avoid cluttering the output.
	pub fn format_tags(&self) -> String {
		if self.tags.is_empty() {
			return String::new();
		}

		// Style each tag individually and combine them with a space
		// Example: [work] [urgent] — blue
		self.tags
			.iter()
			.map(|tag| format!("[{}]", tag).blue().to_string())
			.collect::<Vec<String>>()
			.join(" ")
	}
}

pub struct TodoList {
	pub tasks: Vec<Task>,
}

impl TodoList {
	/// Create new TodoList
	pub fn new() -> Self {
		TodoList { tasks: Vec::new() }
	}

	/// Add new task to TodoList with title
	pub fn add_task(&mut self, title: String, arg_tags: Option<String>) {
		let id = self.tasks.last().map_or(1, |t| t.id + 1);

		let tags = match arg_tags {
			Some(tags_str) => {
				// Fixed: remove the prefix and split by spaces, ignoring extra whitespace.
				tags_str
					.replace("--tags", "")
					.split_whitespace()
					.filter(|s| !s.is_empty()) 
					.map(String::from)
					.collect()
			},
			None => Vec::new(), 
		};
		
		let new_task = Task {
			id,
			title,
			completed: false,
			created_at: Local::now(),
			tags,
		};

		self.tasks.push(new_task);
	}

	/// Delete task from TodoList by id
	pub fn delete_task(&mut self, id: u32) -> bool {
		let initial_len = self.tasks.len();
		self.tasks.retain(|t| t.id != id);
		self.tasks.len() < initial_len
	}

	/// Complete task in TodoList by id
	pub fn complete_task(&mut self, id: u32) -> bool {
		if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
			task.completed = true;
			return true;
		}
		false
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_add_task() {
		let mut list = TodoList::new();
		list.add_task("Test".to_string(), None);
		assert_eq!(list.tasks.len(), 1);
		assert_eq!(list.tasks[0].title, "Test");
		// Исправлен тест: теперь по умолчанию теги пустые
		assert!(list.tasks[0].tags.is_empty()); 
	}

	#[test]
	fn test_add_task_with_tags() {
		let mut list = TodoList::new();
		list.add_task("Test".to_string(), Some("--tags work urgent".to_string()));
		assert_eq!(list.tasks[0].tags, vec!["work".to_string(), "urgent".to_string()]);
	}

	#[test]
	fn test_format_tags_empty() {
		let mut list = TodoList::new();
		list.add_task("Test".to_string(), None);
		assert_eq!(list.tasks[0].format_tags(), "");
	}

	#[test]
	fn test_complete_task() {
		let mut list = TodoList::new();
		list.add_task("Complete Test".to_string(), None);
		let success = list.complete_task(1);
		assert!(success);
		assert!(list.tasks[0].completed);
	}

	#[test]
	fn test_delete_task() {
		let mut list = TodoList::new();
		list.add_task("To delete".to_string(), None);
		let success = list.delete_task(1);
		assert!(success);
		assert_eq!(list.tasks.len(), 0);
	}

	#[test]
	fn test_delete_non_existent_task() {
		let mut list = TodoList::new();
		list.add_task("Delete non-existent task".to_string(), None);
		let success = list.delete_task(2);
		assert!(!success);
	}

	#[test]
	fn test_delete_middle_task() {
		let mut list = TodoList::new();
		list.add_task("Task 1".to_string(), None);
		list.add_task("Task 2".to_string(), None);
		list.add_task("Task 3".to_string(), None);
		let success = list.delete_task(2);
		assert!(success);
		assert_eq!(list.tasks.len(), 2);
		assert_eq!(list.tasks[0].id, 1);
		assert_eq!(list.tasks[1].id, 3);
	}

	#[test]
	fn test_complete_completed_task() {
		let mut list = TodoList::new();
		list.add_task("Task to complete".to_string(), None);
		let success = list.complete_task(1);
		let try_to_success = list.complete_task(1);
		assert!(success);
		assert!(try_to_success);
		assert!(list.tasks[0].completed);
	}
}
