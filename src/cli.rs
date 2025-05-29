use anyhow::Ok;
use clap::{Parser, Subcommand};

use crate::{
    add_handler, delete_handler, done_handler, due_handler, list_handler, modify_handler,
    priority_handler, todo,
};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "add")]
    Add {
        todo: String,
        project: Option<String>,
        context: Option<String>,
    },
    #[command(name = "pri")]
    Priority { id: String, priority: String },
    #[command(name = "due")]
    Due {
        id: String,
        due_date: Option<String>, // 期限は設定してないけど、いずれやらないといけないtodoを表現できるようにOptionにする
    },
    #[command(name = "mod")]
    Modify {
        id: String,
        title: Option<String>,
        project: Option<String>,
        context: Option<String>,
    },
    #[command(name = "done")]
    Done { id: String },
    #[command(name = "ls")]
    List {
        #[arg(short = 'p', long = "project")]
        project: Option<String>,
        #[arg(short = 'c', long = "context")]
        context: Option<String>,
    },
    #[command(name = "del")]
    Delete { id: String },
}

pub fn dispatch(cli: Cli) -> Result<(), anyhow::Error> {
    match cli.command {
        Commands::Add {
            todo,
            project,
            context,
        } => {
            let todo = add_handler(todo, project, context)?;
            println!("Add todo {}: {}", todo.id, todo.title);
            Ok(())
        }
        Commands::Priority { id, priority } => {
            let todo = priority_handler(&id, priority)?;
            if let Some(exist) = todo {
                println!("{exist}");
                let priority = exist.priority;
                if let Some(pri) = priority {
                    println!("TODO: {} prioritized ({}).", exist.id, pri);
                }
            }
            Ok(())
        }
        Commands::Due { id, due_date } => {
            let todo = due_handler(&id, due_date)?;
            if let Some(exist) = todo {
                println!("{exist}");
                let due_date = exist.due_date;
                if let Some(due) = due_date {
                    let due_date = due.format("%Y-%m-%d").to_string();
                    println!("TODO: {} due date set to {}.", exist.id, due_date);
                }
            }
            Ok(())
        }
        Commands::Modify {
            id,
            title,
            project,
            context,
        } => {
            let todo = modify_handler(&id, title, project, context)?;
            if let Some(before) = todo.0 {
                println!("{before}");
            }
            if let Some(updated) = todo.1 {
                println!("TODO: {} is modified with:", updated.id);
                println!("{updated}");
            }
            Ok(())
        }
        Commands::Done { id } => {
            let todo = done_handler(&id)?;
            if let Some(exist) = todo {
                println!("{exist}");
                println!("TODO: {} is done.", exist.id);
            }
            Ok(())
        }
        Commands::List { project, context } => {
            let todos = list_handler(project, context)?;
            todos.0.iter().for_each(|todo| print!("{todo}"));
            println!("\n---");
            println!("TODO: {} of {} tasks shown", todos.0.len(), todos.1);
            Ok(())
        }
        Commands::Delete { id } => {
            if let Some(deleted) = delete_handler(&id)? {
                println!("{} {}", deleted.id, deleted.title);
                println!("TODO: {} deleted.", deleted.id);
            }
            Ok(())
        }
    }
}
