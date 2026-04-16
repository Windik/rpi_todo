use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
	pub id: u32,
	pub title: String,
	pub completed: bool,
}

pub struct TodoList {
	pub tasks: Vec<Task>,
}

impl TodoList {
	pub fn new() -> Self {
		TodoList { tasks: Vec::new() }
	}

	pub fn add_task(&mut self, title: String) {
		let id = self.tasks.last().map_or(1, |t| t.id + 1);

		self.tasks.push(Task { id, title, completed: false });
	}

	pub fn delete_task(&mut self, id: u32) -> bool {
		let initial_len = self.tasks.len();

		self.tasks.retain(|t| t.id != id);
		self.tasks.len() < initial_len
	}

	pub fn complete_task(&mut self, id: u32) -> bool {
		for i in self.tasks.iter_mut() {
			if i.id == id {
				i.completed = true;
				return true;
			}
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

		list.add_task("Test".to_string());
		assert_eq!(list.tasks.len(), 1);
		assert_eq!(list.tasks[0].title, "Test");
	}

	#[test]
	fn test_complete_task() {
		let mut list = TodoList::new();

		list.add_task("Complete Test".to_string());

		let success = list.complete_task(1);

		assert!(success);
		assert!(list.tasks[0].completed);
	}

	#[test]
	fn test_delete_task() {
		let mut list = TodoList::new();

		list.add_task("To delete".to_string());

		let success = list.delete_task(1);

		assert!(success);
		assert_eq!(list.tasks.len(), 0);
	}
}
