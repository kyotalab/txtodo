use clap::Parser;
use txtodo::{Cli, cli};

fn main() -> Result<(), anyhow::Error> {
    // let projects = vec!["+todo.txt"];
    // let contexts = vec!["@self-improvement"];

    // let project_prefix = Regex::new(r"^\+").unwrap();
    // let context_prefix = Regex::new(r"^@").unwrap();

    // let validated_projects = validated_keyword(projects, project_prefix)?;
    // let validated_contexts = validated_keyword(contexts, context_prefix)?;

    // let todo = Todo {
    //     id: "t-001".into(),
    //     title: "test todo".into(),
    //     priority: Some(Priority::B),
    //     projects: validated_projects,
    //     contexts: validated_contexts,
    //     due_date: Local::now().naive_local(),
    //     created_at: Some(Local::now().naive_local()),
    //     end_date: None,
    //     is_done: false,
    // };

    // println!("{todo}");

    let cli = Cli::parse();
    cli::dispatch(cli);
    Ok(())
}
