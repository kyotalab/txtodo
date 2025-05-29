use crate::{Todo, load_todos, parse_priority_to_enum, save_todo_txt};
use anyhow::{Context, Result};
use std::str::FromStr;

pub fn priority_handler(id: &str, priority: String) -> Result<Option<Todo>> {
    let mut todos = load_todos()?;
    let id = u32::from_str(id).context("Failed to parse ID")?;

    let mut updated_todo: Option<Todo> = None;

    for todo in &mut todos {
        if todo.id == id {
            let pri = parse_priority_to_enum(&priority)?;
            todo.priority = Some(pri);
            updated_todo = Some(todo.clone()); // コピーして返却用に保存
            break;
        }
    }

    save_todo_txt(todos)?;

    if updated_todo.is_none() {
        println!("No TODO found matching ID {}.", id);
    }

    Ok(updated_todo)
}
