use crate::util::load_todos;
use crate::{TODO_PATH, model::Todo};
use anyhow::{Context, Result};
use std::fs;

pub fn write_to_toto_txt(todo: &Todo) -> Result<()> {
    let mut todos = load_todos()?;

    // 2. 新しいTodoを追加
    todos.push(todo.clone());

    // 3. Vec<Todo> をJSON文字列に変換
    let json = serde_json::to_string_pretty(&todos).context("Failed to serialize todos")?;

    // 4. ファイルに書き込む（全体を書き直し）
    fs::write(TODO_PATH, json).context("Failed to write todo.json")?;

    Ok(())
}

pub fn save_todo_txt(todos: Vec<Todo>) -> Result<()> {
    let json = serde_json::to_string_pretty(&todos).context("Failed to serialize todos")?;

    // 4. ファイルに書き込む（全体を書き直し）
    fs::write(TODO_PATH, json).context("Failed to write todo.json")?;

    Ok(())
}
