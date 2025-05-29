use clap::{Parser, Subcommand};

use crate::{add_handler, list_handler, priority_handler};

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
    // #[command(name = "pri")]
    // Priority {
    //     id: String,
    //     priority: Option<String>, // 優先度はOption
    // },
    // #[command(name = "due")]
    // Due {
    //     id: String,
    //     due_date: Option<String>, // 期限は設定してないけど、いずれやらないといけないtodoを表現できるようにOptionにする
    // },
    // #[command(name = "mod")]
    // Modify {
    //     id: String,
    //     todo: Option<String>,
    //     project: Option<String>,
    //     context: Option<String>,
    // },
    // #[command(name = "start")]
    // Start { id: String },
    // #[command(name = "done")]
    // Done { id: String },
    #[command(name = "ls")]
    List {
        #[arg(short = 'p', long = "project")]
        project: Option<String>,
        #[arg(short = 'c', long = "context")]
        context: Option<String>,
    },
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
        Commands::List { project, context } => {
            let todos = list_handler(project, context)?;
            todos.iter().for_each(|todo| println!("{todo}"));
            Ok(())
        }
    }
}
