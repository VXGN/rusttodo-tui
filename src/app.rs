use crate::todo::{Priority, TodoItem};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub enum InputMode {
    Normal,
    Adding(AddingField),
    Editing,
    ViewingDetails,
}

#[derive(Debug, PartialEq)]
pub enum AddingField {
    Title,
    Description,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Tab {
    All,
    Active,
    Completed,
}

impl Tab {
    fn next(self) -> Self {
        match self {
            Tab::All => Tab::Active,
            Tab::Active => Tab::Completed,
            Tab::Completed => Tab::All,
        }
    }
}

pub struct App {
    pub todos: Vec<TodoItem>,
    pub selected: usize,
    pub tab: Tab,
    pub input_mode: InputMode,
    pub input_buffer: String,
    pub description_buffer: String,
    pub priority_index: usize,
    pub should_quit: bool,
    next_id: usize,
    storage_path: PathBuf,
}

impl App {
    const PRIORITIES: [Priority; 3] = [Priority::Low, Priority::Medium, Priority::High];
    const DEFAULT_PRIORITY_INDEX: usize = 1;

    pub fn new() -> Self {
        let storage_path = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".todo-tui.json");

        let mut app = Self {
            todos: Vec::new(),
            selected: 0,
            tab: Tab::All,
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            description_buffer: String::new(),
            priority_index: Self::DEFAULT_PRIORITY_INDEX,
            should_quit: false,
            next_id: 0,
            storage_path,
        };

        app.load();
        app
    }

    pub fn filtered_todos(&self) -> Vec<&TodoItem> {
        self.todos.iter()
            .filter(|t| match self.tab {
                Tab::All => true,
                Tab::Active => !t.completed,
                Tab::Completed => t.completed,
            })
            .collect()
    }

    pub fn add_todo(&mut self) {
        if self.input_buffer.is_empty() {
            return;
        }

        let priority = Self::PRIORITIES[self.priority_index.min(2)];
        let todo = TodoItem::new(
            self.next_id,
            std::mem::take(&mut self.input_buffer),
            std::mem::take(&mut self.description_buffer),
            priority,
        );
        
        self.todos.push(todo);
        self.next_id += 1;
        self.priority_index = Self::DEFAULT_PRIORITY_INDEX;
        self.save();
    }

    pub fn delete_selected(&mut self) {
        if let Some(id) = self.selected_todo_id() {
            self.todos.retain(|t| t.id != id);
            self.clamp_selection();
            self.save();
        }
    }

    pub fn toggle_selected(&mut self) {
        if let Some(id) = self.selected_todo_id() {
            if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
                todo.toggle();
                self.save();
            }
        }
    }

    pub fn next_item(&mut self) {
        self.move_selection(1);
    }

    pub fn previous_item(&mut self) {
        self.move_selection(-1);
    }

    pub fn next_tab(&mut self) {
        self.tab = self.tab.next();
        self.selected = 0;
    }

    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self.todos) {
            let _ = fs::write(&self.storage_path, json);
        }
    }

    pub fn load(&mut self) {
        if let Ok(content) = fs::read_to_string(&self.storage_path) {
            if let Ok(todos) = serde_json::from_str(&content) {
                self.todos = todos;
                self.next_id = self.todos.iter()
                    .map(|t| t.id)
                    .max()
                    .map_or(0, |max| max + 1);
            }
        }
    }

    // Helper methods
    fn selected_todo_id(&self) -> Option<usize> {
        self.filtered_todos().get(self.selected).map(|t| t.id)
    }

    fn move_selection(&mut self, delta: isize) {
        let len = self.filtered_todos().len();
        if len == 0 {
            return;
        }
        
        self.selected = if delta > 0 {
            (self.selected + 1) % len
        } else {
            self.selected.checked_sub(1).unwrap_or(len - 1)
        };
    }

    fn clamp_selection(&mut self) {
        let len = self.filtered_todos().len();
        if self.selected >= len && len > 0 {
            self.selected = len - 1;
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
