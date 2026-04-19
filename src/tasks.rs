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
	/// Create new TodoList
	///
	/// # Examples
	/// ```
	/// use rpi_todo::tasks::TodoList;
	/// let list = TodoList::new();
	/// assert_eq!(list.tasks.len(), 0);
	/// ```
	pub fn new() -> Self {
		TodoList { tasks: Vec::new() }
	}

	/// Add new task to TodoList with title
	///
	/// # Examples
	/// ```
	/// use rpi_todo::tasks::TodoList;
	/// let mut list = TodoList::new();
	/// list.add_task("Example".to_string());
	/// assert_eq!(list.tasks.len(), 1);
	/// assert_eq!(list.tasks[0].title, "Example");
	/// ```
	pub fn add_task(&mut self, title: String) {
		let id = self.tasks.last().map_or(1, |t| t.id + 1);

		self.tasks.push(Task { id, title, completed: false });
	}

	/// Delete task from TodoList by id
	///
	/// # Examples
	/// ```
	/// use rpi_todo::tasks::TodoList;
	/// let mut list = TodoList::new();
	/// list.add_task("Delete example".to_string());
	/// assert_eq!(list.tasks.len(), 1);
	/// list.delete_task(1);
	/// assert_eq!(list.tasks.len(), 0);
	/// ```
	pub fn delete_task(&mut self, id: u32) -> bool {
		let initial_len = self.tasks.len();

		self.tasks.retain(|t| t.id != id);
		self.tasks.len() < initial_len
	}

	/// Complete task in TodoList by id
	///
	/// # Example
	/// ```
	/// use rpi_todo::tasks::TodoList;
	/// let mut list = TodoList::new();
	/// list.add_task("Need to complete".to_string());
	/// let success = list.complete_task(1);
	/// assert_eq!(list.tasks[0].completed, success);
	/// ```
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

	#[test]
	fn test_delete_non_existent_task() {
		let mut list = TodoList::new();

		list.add_task("Delete non-existent task".to_string());

		let success = list.delete_task(2);

		assert!(success == false);
	}

	#[test]
	fn test_delete_middle_task() {
		let mut list = TodoList::new();

		list.add_task("Task 1".to_string());
		list.add_task("Task 2".to_string());
		list.add_task("Task 3".to_string());

		let success = list.delete_task(2);

		assert!(success);
		assert_eq!(list.tasks.len(), 2);
		assert_eq!(list.tasks[0].id, 1);
		assert_eq!(list.tasks[1].id, 3);
	}

	#[test]
	fn test_complete_completed_task() {
		let mut list = TodoList::new();

		list.add_task("Task to complete".to_string());

		let success = list.complete_task(1);
		let try_to_success = list.complete_task(1);

		assert!(success);
		assert!(try_to_success);
		assert!(list.tasks[0].completed);
	}
}
