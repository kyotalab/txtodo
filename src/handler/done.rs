use crate::{Todo, load_todos, save_todo_txt};
use anyhow::{Context, Result};
use chrono::{Local, NaiveDate};
use std::str::FromStr;

pub fn done_handler(id: &str) -> Result<Option<Todo>> {
    let mut todos = load_todos()?;
    let id = u32::from_str(id).context("Failed to parse ID")?;

    let mut updated_todo: Option<Todo> = None;

    for todo in &mut todos {
        if todo.id == id {
            if !todo.is_done {
                todo.is_done = true;
                todo.end_date = Some(Local::now().date_naive());
                updated_todo = Some(todo.clone());

                break;
            } else {
                println!("TODO: {} is already done.", id);
                return Ok(None);
            }
        }
    }

    save_todo_txt(todos)?;
    if updated_todo.is_none() {
        println!("No TODO found matching ID {}.", id);
    }

    Ok(updated_todo)
}
