use rpi_todo::tasks::TodoList;

#[test]
fn test_full_todo_cycle() {
	let mut list = TodoList::new();
	let task_text = "Integration Test";

	list.add_task(task_text.to_string());

	assert_eq!(list.tasks.len(), 1);
	assert_eq!(list.tasks[0].title, task_text);

	let success = list.complete_task(1);

	assert!(success);

	assert!(list.tasks[0].completed);
}
