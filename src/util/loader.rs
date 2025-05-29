use crate::model::Todo;
use anyhow::{Context, Error, Result};
use std::fs;
use std::path::Path;

pub fn load_todos() -> Result<Vec<Todo>, Error> {
    let path = Path::new("todo.json");
    let todos: Vec<Todo> = if path.exists() {
        let json_data = fs::read_to_string(path).context("Failed to read todo.json")?;
        if json_data.trim().is_empty() {
            vec![]
        } else {
            serde_json::from_str(&json_data).context("Failed to parse todo.json")?
        }
    } else {
        vec![]
    };

    Ok(todos)
}
