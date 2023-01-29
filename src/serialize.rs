//! Provides functions for reading and writing todo lists to the disk.

use std::fs;
use std::io;

use thiserror::Error;

use crate::models::*;

/// An error type combining all potential issues while loading
/// or dumping the todo list.
#[derive(Error, Debug)]
pub enum SerdeError {
    #[error("failed to read or write given file")]
    IO(#[from] io::Error),
    #[error("failed to parse file contents")]
    Parse(#[from] toml::de::Error),
    #[error("failed to serialize todo list")]
    Format(#[from] toml::ser::Error),
}

/// Loads a `TodoList` instance in TOML format from the given `filepath`.
///
/// # Errors
///
/// A `SerdeError::IO` or `SerdeError::Parse` error may occur if either
/// operation fails during this process.
pub fn load_todo_list(filepath: &str) -> Result<TodoList, SerdeError> {
    let contents = fs::read_to_string(filepath)?;
    Ok(toml::from_str::<TodoList>(&contents)?)
}

/// Dumps the given `TodoList` instance to `filepath` in TOML format.
///
/// # Errors
///
/// A `SerdeError::IO` or `SerdeError::Format` error may occur if either
/// operation fails during this process.
pub fn dump_todo_list(todo_list: &TodoList, filepath: &str) -> Result<(), SerdeError> {
    let contents = toml::ser::to_string(todo_list)?;
    fs::write(filepath, contents)?;
    Ok(())
}
