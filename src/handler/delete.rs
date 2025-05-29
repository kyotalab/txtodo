use crate::{Todo, load_todos, save_todo_txt};
use anyhow::{Context, Result};
use std::io::{self, Write};
use std::str::FromStr;

pub fn delete_handler(id: &str) -> Result<Option<Todo>> {
    let mut todos = load_todos()?;
    let id = u32::from_str(id).context("Failed to parse ID")?;

    if let Some(pos) = todos.iter().position(|todo| todo.id == id) {
        let todo = &mut todos[pos];

        if todo.deleted {
            println!("TODO: {} is already deleted.", id);
            return Ok(None);
        }

        // 確認プロンプトの表示
        print!("Delete '{}'?  (y/n): ", todo.title);
        io::stdout().flush()?; // 出力を即座に表示

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();

        if input != "y" {
            println!("Canceled.");
            return Ok(None);
        }

        // 削除処理
        todo.deleted = true;
        let deleted = todo.clone();

        save_todo_txt(todos)?; // 保存

        return Ok(Some(deleted));
    }

    println!("No TODO found matching ID {}.", id);
    Ok(None)
}
