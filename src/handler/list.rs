use crate::{Todo, load_todos};

pub fn list_handler(
    project: Option<String>,
    context: Option<String>,
) -> Result<Vec<Todo>, anyhow::Error> {
    let todos = load_todos()?;
    if let Some(pj) = project {
        let filtered: Vec<_> = todos
            .iter()
            .filter(|todo| todo.projects.contains(&pj))
            .cloned()
            .collect();

        if filtered.is_empty() {
            println!(
                "⚠️ There are no matching todos for the specified project '{}'. ",
                pj
            );
        }
        // for todo in todos {
        //     for proj in &todo.projects {
        //         if *proj == pj {
        //             filtered_todos.push(todo.clone());
        //         }
        //     }
        // }
        return Ok(filtered);
    }

    if let Some(ctx) = context {
        let filtered: Vec<_> = todos
            .iter()
            .filter(|todo| todo.contexts.contains(&ctx))
            .cloned()
            .collect();

        if filtered.is_empty() {
            println!(
                "⚠️ There are no matching todos for the specified context '{}'. ",
                ctx
            );
        }
        // for todo in todos {
        //     for ctx in &todo.contexts {
        //         if *ctx == cx {
        //             filtered_todos.push(todo.clone());
        //         }
        //     }
        // }
        return Ok(filtered);
    }

    // let filtered: Vec<_> = todos
    // .into_iter()
    // .filter(|todo| {
    //     project.as_ref().map_or(true, |pj| todo.projects.contains(pj)) &&
    //     context.as_ref().map_or(true, |ctx| todo.contexts.contains(ctx))
    // })
    // .collect();

    Ok(todos)
}
