use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Todo {
    pub id: Option<String>,
    pub title: String,
    pub description: String,
    pub completed: Option<bool>,
    pub create_time: Option<DateTime<Utc>>
}

#[derive(Deserialize)]
pub struct QueryOptions {
    pub completed: bool
}

pub struct AppState {
    pub todos: Arc<Mutex<Vec<Todo>>>
}

impl AppState {
    pub fn new() -> AppState {
        Self {
            todos: Arc::new(Mutex::new(Vec::new()))
        }
    }
}
