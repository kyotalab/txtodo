use crate::util::{validated_keyword, write_to_toto_txt};
use crate::{Todo, read_lines};
use chrono::Local;

const PROJECT_PREFIX: &'static str = "@";
const CONTEXT_PREFIX: &'static str = "+";

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
    };

    println!("{todo}");

    write_to_toto_txt(&todo)?;

    Ok(todo)
}

pub fn generate_todo_id() -> Result<u32, anyhow::Error> {
    let lines = read_lines("todo.txt")?;
    let mut counter = 1;
    lines.for_each(|_| counter += 1);
    Ok(counter)
}
