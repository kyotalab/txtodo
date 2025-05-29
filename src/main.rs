use clap::Parser;
use txtodo::{Cli, cli};

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();
    cli::dispatch(cli)?;
    Ok(())
}
