use std::fmt;

#[derive(Debug)]
pub enum TodoError {
	// Standart Error
	Io(std::io::Error),
	// Serialization Error
	Json(serde_json::Error),
	// Own 'not found' Error for local
	LocaleNotFound(String),
	// Confy Error
	Config(confy::ConfyError), 
	// Task not found Error
	TaskNotFound(u32),
}

// Display realization
impl fmt::Display for TodoError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			TodoError::Io(err) => write!(f, "IO Error: {}", err),
			TodoError::Json(err) => write!(f, "JSON Error: {}", err),
			TodoError::LocaleNotFound(lang) => write!(f, "Language locale not found: {}", lang),
			TodoError::Config(err) => write!(f, "Config Error: {}", err), 
			TodoError::TaskNotFound(id) => write!(f, "Task with ID {} not found", id),
		}
	}
}

impl std::error::Error for TodoError{}

impl From<std::io::Error> for TodoError {
	fn from(err: std::io::Error) -> Self {
		TodoError::Io(err)
	}
}

impl From<serde_json::Error> for TodoError {
	fn from(err: serde_json::Error) -> Self {
		TodoError::Json(err)
	}
}

impl From<confy::ConfyError> for TodoError {
	fn from(err: confy::ConfyError) -> Self {
		TodoError::Config(err)
	}
}
