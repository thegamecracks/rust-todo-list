use std::fs;
use std::io;

use thiserror::Error;

use crate::models::*;

#[derive(Error, Debug)]
pub enum SerdeError {
    #[error("failed to read or write given file")]
    IO(#[from] io::Error),
    #[error("failed to parse file contents")]
    Parse(#[from] toml::de::Error),
    #[error("failed to serialize todo list")]
    Format(#[from] toml::ser::Error),
}

pub fn load_todo_list(filepath: &str) -> Result<TodoList, SerdeError> {
    let contents = fs::read_to_string(filepath)?;
    Ok(toml::from_str::<TodoList>(&contents)?)
}

pub fn dump_todo_list(todo_list: &TodoList, filepath: &str) -> Result<(), SerdeError> {
    let contents = toml::ser::to_string(todo_list)?;
    fs::write(filepath, contents)?;
    Ok(())
}
