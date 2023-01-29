mod models;
mod serialize;

use std::cmp::Ordering;
use std::io;
use std::io::Write;

use thiserror::Error;

use models::*;
use serialize::*;

const TODO_LIST_FILE_PATH: &str = "todo_list.toml";
const HELP_MESSAGE: &str = "\
1. Add a new item
2. Remove an existing item
3. Set an item as complete/incomplete
4. Move an item up or down the list
5. Show this help message
0. Quit\
";

#[derive(PartialEq, Eq)]
enum Command {
    Add(String),
    Remove(usize),
    ToggleCompletion(usize),
    Move(usize, usize),
    Help,
    Quit,
}

struct ProgramInterface {
    todo_list: TodoList,
}

impl ProgramInterface {
    pub fn run_loop(&mut self) {
        loop {
            let command = self.input_command();
            self.exec_command(&command);

            if command == Command::Quit {
                break;
            }

            println!();
            self.print_todo_list();
            println!();
        }
    }

    pub fn input_command(&self) -> Command {
        const DEFAULT_PROMPT: &str = "Select a number: ";
        let mut prompt = DEFAULT_PROMPT;

        loop {
            let choice = Self::input_integer(prompt);
            match self.input_command_arguments(choice) {
                Ok(command) => {
                    return command;
                }
                Err(error) => {
                    prompt = match error {
                        CommandError::InsufficientItems(n) => {
                            println!(
                                "Needs at least {n} {}",
                                if n == 1 { "item" } else { "items" }
                            );
                            DEFAULT_PROMPT
                        }
                        CommandError::UnknownChoice => "Unknown choice: ",
                    }
                }
            };
        }
    }

    pub fn exec_command(&mut self, command: &Command) {
        match command {
            Command::Add(description) => {
                self.todo_list.add_item(TodoItem {
                    description: description.to_string(),
                    ..Default::default()
                });
                println!("Added item #{}", self.todo_list.items.len());
            }
            Command::Remove(index) => {
                self.todo_list.remove_item(*index);
                println!("Removed item #{}", index + 1);
            }
            Command::ToggleCompletion(index) => {
                let completed = self.todo_list.toggle_completion(*index);
                let completed = if completed { "completed" } else { "incomplete" };
                println!("Marked item #{} as {}", index + 1, completed);
            }
            Command::Move(ix_old, ix_new) => {
                self.todo_list.move_item(*ix_old, *ix_new);
                println!("Moved #{} to #{}", ix_old + 1, ix_new + 1);
            }
            Command::Help => println!("{HELP_MESSAGE}"),
            Command::Quit => (),
        }
    }

    // Input

    fn input_command_arguments(&self, choice: usize) -> Result<Command, CommandError> {
        match choice {
            1 => {
                let description = Self::input_line("Describe your todo item: ");
                Ok(Command::Add(description))
            }
            2 => match self.todo_list.items.len().cmp(&1) {
                Ordering::Less => Err(CommandError::InsufficientItems(1)),
                Ordering::Equal => Ok(Command::Remove(0)),
                Ordering::Greater => {
                    let index = self.input_item_index("Index of an item to remove: ");
                    Ok(Command::Remove(index))
                }
            },
            3 => match self.todo_list.items.len().cmp(&1) {
                Ordering::Less => Err(CommandError::InsufficientItems(1)),
                Ordering::Equal => Ok(Command::ToggleCompletion(0)),
                Ordering::Greater => {
                    let index =
                        self.input_item_index("Index of an item to mark as (in)completed: ");
                    Ok(Command::ToggleCompletion(index))
                }
            },
            4 => match self.todo_list.items.len().cmp(&2) {
                Ordering::Less => Err(CommandError::InsufficientItems(2)),
                Ordering::Equal => Ok(Command::Move(0, 1)),
                Ordering::Greater => {
                    let ix_old = self.input_item_index("Index of the item to move: ");
                    let ix_new = self.input_item_index("Index to move to: ");
                    Ok(Command::Move(ix_old, ix_new))
                }
            },
            5 => Ok(Command::Help),
            0 => Ok(Command::Quit),
            _ => Err(CommandError::UnknownChoice),
        }
    }

    fn input_line(prompt: &str) -> String {
        let mut input = String::new();

        print!("{prompt}");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        // NOTE: there's a memory copy here but for simplification,
        //       I won't re-implement this in-place
        input.trim().to_string()
    }

    fn input_integer(mut prompt: &str) -> usize {
        loop {
            match Self::input_line(prompt).parse::<usize>() {
                Ok(value) => return value,
                Err(_) => {
                    prompt = "Invalid integer: ";
                }
            };
        }
    }

    fn input_item_index(&self, prompt: &str) -> usize {
        let length = self.todo_list.items.len();
        let invalid_prompt = format!("Must be within 1 and {length}: ");
        let mut current_prompt = prompt;

        loop {
            let n = Self::input_integer(current_prompt);
            if n < 1 || n > length {
                current_prompt = &invalid_prompt;
                continue;
            }

            return n - 1;
        }
    }

    // Output

    fn print_todo_list(&self) {
        for (i, item) in self.todo_list.items.iter().enumerate() {
            let i = i + 1;
            let checkmark = if item.completed { "[X]" } else { "[ ]" };
            println!("{checkmark} {i}. {}", item.description);
        }

        if self.todo_list.items.is_empty() {
            println!("No items to show");
        }

        let local = self.todo_list.last_updated.with_timezone(&chrono::Local);
        println!("Last updated at {}", local.format("%d/%m/%Y %H:%M:%S"));
    }
}

#[derive(Error, Debug)]
enum CommandError {
    #[error("{0} items are required")]
    InsufficientItems(usize),
    #[error("unknown choice provided")]
    UnknownChoice,
}

fn main() {
    let todo_list = load_todo_list(TODO_LIST_FILE_PATH).unwrap_or_else(|error| match error {
        SerdeError::IO(error) if error.kind() == io::ErrorKind::NotFound => Default::default(),
        _ => panic!("Unhandled error while loading todo list: {error:#?}"),
    });
    let mut interface = ProgramInterface { todo_list };

    interface.print_todo_list();
    println!();
    interface.exec_command(&Command::Help);
    println!();

    interface.run_loop();

    dump_todo_list(&interface.todo_list, TODO_LIST_FILE_PATH)
        .expect("An error occurred while saving");
}
