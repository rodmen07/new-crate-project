use clap::Parser;
use new_crate_project::{Cli, run};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let output = run(cli)?;
    println!("{output}");
    Ok(())
}
