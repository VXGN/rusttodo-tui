use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub priority: Priority,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {
    pub fn as_str(&self) -> &str {
        match self {
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High",
        }
    }

    pub fn color(&self) -> ratatui::style::Color {
        match self {
            Priority::Low => ratatui::style::Color::Green,
            Priority::Medium => ratatui::style::Color::Yellow,
            Priority::High => ratatui::style::Color::Red,
        }
    }
}

impl TodoItem {
    pub fn new(id: usize, title: String, description: String, priority: Priority) -> Self {
        Self {
            id,
            title,
            description,
            completed: false,
            created_at: Utc::now(),
            priority,
        }
    }

    pub fn toggle(&mut self) {
        self.completed = !self.completed;
    }
}