use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TodoItem {
    pub description: String,
    pub completed: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TodoList {
    pub items: Vec<TodoItem>,
    pub last_updated: DateTime<Utc>,
}

impl TodoList {
    pub fn add_item(&mut self, item: TodoItem) {
        self.items.push(item);
        self.set_last_updated();
    }

    pub fn move_item(&mut self, ix_old: usize, ix_new: usize) {
        let item = self.items.remove(ix_old);
        self.items.insert(ix_new, item);
        self.set_last_updated();
    }

    pub fn remove_item(&mut self, index: usize) {
        self.items.remove(index);
        self.set_last_updated();
    }

    pub fn toggle_completion(&mut self, index: usize) -> bool {
        let item = &mut self.items[index];

        let updated = !item.completed;
        item.completed = updated;

        self.set_last_updated();

        updated
    }

    fn set_last_updated(&mut self) {
        self.last_updated = chrono::Utc::now();
    }
}

impl Default for TodoList {
    fn default() -> Self {
        TodoList {
            items: Default::default(),
            last_updated: Utc::now(),
        }
    }
}
