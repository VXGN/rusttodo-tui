use crate::todo::{Priority, TodoItem};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub enum InputMode {
    Normal,
    Adding,
    Editing,
    ViewingDetails,
}

#[derive(Debug, PartialEq)]
pub enum Tab {
    All,
    Active,
    Completed,
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
    pub next_id: usize,
    storage_path: PathBuf,
}

impl App {
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
            priority_index: 1,
            should_quit: false,
            next_id: 0,
            storage_path,
        };

        app.load();
        app
    }

    pub fn filtered_todos(&self) -> Vec<&TodoItem> {
        match self.tab {
            Tab::All => self.todos.iter().collect(),
            Tab::Active => self.todos.iter().filter(|t| !t.completed).collect(),
            Tab::Completed => self.todos.iter().filter(|t| t.completed).collect(),
        }
    }

    pub fn add_todo(&mut self) {
        if !self.input_buffer.is_empty() {
            let priority = match self.priority_index {
                0 => Priority::Low,
                1 => Priority::Medium,
                _ => Priority::High,
            };

            let todo = TodoItem::new(
                self.next_id,
                self.input_buffer.clone(),
                self.description_buffer.clone(),
                priority,
            );
            self.todos.push(todo);
            self.next_id += 1;
            self.input_buffer.clear();
            self.description_buffer.clear();
            self.priority_index = 1;
            self.save();
        }
    }

    pub fn delete_selected(&mut self) {
        let filtered = self.filtered_todos();
        if let Some(todo) = filtered.get(self.selected) {
            let id = todo.id;
            self.todos.retain(|t| t.id != id);
            if self.selected > 0 && self.selected >= self.filtered_todos().len() {
                self.selected -= 1;
            }
            self.save();
        }
    }

    pub fn toggle_selected(&mut self) {
        let filtered = self.filtered_todos();
        if let Some(todo) = filtered.get(self.selected) {
            let id = todo.id;
            if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
                todo.toggle();
                self.save();
            }
        }
    }

    pub fn next_item(&mut self) {
        let len = self.filtered_todos().len();
        if len > 0 {
            self.selected = (self.selected + 1) % len;
        }
    }

    pub fn previous_item(&mut self) {
        let len = self.filtered_todos().len();
        if len > 0 {
            self.selected = if self.selected == 0 {
                len - 1
            } else {
                self.selected - 1
            };
        }
    }

    pub fn next_tab(&mut self) {
        self.tab = match self.tab {
            Tab::All => Tab::Active,
            Tab::Active => Tab::Completed,
            Tab::Completed => Tab::All,
        };
        self.selected = 0;
    }

    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self.todos) {
            let _ = fs::write(&self.storage_path, json);
        }
    }

    pub fn load(&mut self) {
        if let Ok(content) = fs::read_to_string(&self.storage_path) {
            if let Ok(todos) = serde_json::from_str::<Vec<TodoItem>>(&content) {
                self.todos = todos;
                self.next_id = self.todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;
            }
        }
    }
}