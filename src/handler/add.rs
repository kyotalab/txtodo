use crate::Todo;
use crate::util::{validated_keyword, write_to_toto_txt};
use crate::{CONTEXT_PREFIX, PROJECT_PREFIX};
use anyhow::{Context, Result};
use chrono::Local;
use std::fs;
use std::path::Path;

pub fn add_handler(
    todo: String,
    project: Option<String>,
    context: Option<String>,
) -> Result<Todo, anyhow::Error> {
    let todo_id = generate_todo_id()?;
    let mut pj = String::new();
    let mut ctx = String::new();

    if let Some(p) = project {
        pj = p;
    }
    if let Some(c) = context {
        ctx = c;
    }

    let validated_projects = validated_keyword(pj, PROJECT_PREFIX)?;
    let validated_contexts = validated_keyword(ctx, CONTEXT_PREFIX)?;

    let todo = Todo {
        id: todo_id,
        title: todo,
        priority: None,
        projects: validated_projects,
        contexts: validated_contexts,
        due_date: None,
        created_at: Local::now().naive_local(),
        end_date: None,
        is_done: false,
        deleted: false,
    };

    write_to_toto_txt(&todo)?;

    Ok(todo)
}

pub fn generate_todo_id() -> Result<u32> {
    let path = Path::new("todo.json");

    if !path.exists() {
        return Ok(1); // ファイルがなければ ID = 1
    }

    let json_data = fs::read_to_string(path).context("Failed to read todo.json")?;

    if json_data.trim().is_empty() {
        return Ok(1); // 空ファイルなら ID = 1
    }

    let deserialized_json: Vec<Todo> =
        serde_json::from_str(&json_data).context("Failed to parse todo.json as Vec<Todo>")?;

    let next_id = deserialized_json
        .into_iter()
        .map(|todo| todo.id)
        .max()
        .unwrap_or(0)
        + 1;

    Ok(next_id)
}
