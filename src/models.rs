//! Provides the structures to model the data of a todo list.

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

/// A single item in a todo list.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TodoItem {
    pub description: String,
    pub completed: bool,
}

/// A list of todo items.
#[derive(Debug, Deserialize, Serialize)]
pub struct TodoList {
    items: Vec<TodoItem>,
    pub last_updated: DateTime<Utc>,
}

impl TodoList {
    // Item access

    /// Returns the number of `TodoItem`s contained in this list.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns `true` if this list contains no `TodoItem`s.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Returns an iterator over all `TodoItem`s in this list.
    pub fn iter(&self) -> std::slice::Iter<'_, TodoItem> {
        self.items.iter()
    }

    // Item manipulation

    /// Appends the given `TodoItem` to the end of this list.
    pub fn add_item(&mut self, item: TodoItem) {
        self.items.push(item);
        self.set_last_updated();
    }

    /// Moves the `TodoItem` at `ix_old` to `ix_new`.
    ///
    /// # Panics
    ///
    /// Panics if `ix_old` or `ix_new` is out of bounds.
    pub fn move_item(&mut self, ix_old: usize, ix_new: usize) {
        let item = self.items.remove(ix_old);
        self.items.insert(ix_new, item);
        self.set_last_updated();
    }

    /// Removes the `TodoItem` at `index`.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    pub fn remove_item(&mut self, index: usize) {
        self.items.remove(index);
        self.set_last_updated();
    }

    /// Toggles the `TodoItem` at the given `index` between completed and
    /// incomplete, returning its new status.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    pub fn toggle_completion(&mut self, index: usize) -> bool {
        let item = &mut self.items[index];

        let updated = !item.completed;
        item.completed = updated;

        self.set_last_updated();

        updated
    }

    /// Updates the last updated date time to the current time.
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
