use crate::{CONTEXT_PREFIX, PROJECT_PREFIX};
use crate::{Todo, load_todos, save_todo_txt, validated_keyword};
use anyhow::{Context, Result};
use std::str::FromStr;

pub fn modify_handler(
    id: &str,
    title: Option<String>,
    project: Option<String>,
    context: Option<String>,
) -> Result<(Option<Todo>, Option<Todo>)> {
    let mut todos = load_todos()?;
    let id = u32::from_str(id).context("Failed to parse ID")?;

    let mut before_todo: Option<Todo> = None;

    let mut updated_todo: Option<Todo> = None;

    for todo in &mut todos {
        if todo.id == id {
            before_todo = Some(todo.clone());
            let mut ti = String::new();
            let mut pj = String::new();
            let mut ctx = String::new();

            if let Some(t) = title {
                ti = t;
            }
            if let Some(p) = project {
                pj = p;
            }
            if let Some(c) = context {
                ctx = c;
            }
            todo.title = ti;
            let validated_projects = validated_keyword(pj, PROJECT_PREFIX)?;
            todo.projects = validated_projects;
            let validated_contexts = validated_keyword(ctx, CONTEXT_PREFIX)?;
            todo.contexts = validated_contexts;
            updated_todo = Some(todo.clone()); // コピーして返却用に保存
            break;
        }
    }

    save_todo_txt(todos)?;

    if updated_todo.is_none() {
        println!("No TODO found matching ID {}.", id);
    }

    let result = (before_todo, updated_todo);

    Ok(result)
}
