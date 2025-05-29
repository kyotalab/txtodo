use crate::{Todo, load_todos, save_todo_txt};
use anyhow::{Context, Result};
use chrono::NaiveDate;
use std::str::FromStr;

pub fn due_handler(id: &str, due_date: Option<String>) -> Result<Option<Todo>> {
    let mut todos = load_todos()?;
    let id = u32::from_str(id).context("Failed to parse ID")?;

    let mut updated_todo: Option<Todo> = None;

    for todo in &mut todos {
        if todo.id == id {
            if let Some(d) = due_date {
                let due = NaiveDate::parse_from_str(&d, "%Y-%m-%d")?;
                todo.due_date = Some(due);
                updated_todo = Some(todo.clone());
            }
            break;
        }
    }

    save_todo_txt(todos)?;
    if updated_todo.is_none() {
        println!("No ToDo found matching ID {}.", id);
    }

    Ok(updated_todo)
}
