use clap::Parser;
use new_crate_project::{run, Cli};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let output = run(cli)?;
    println!("{output}");
    Ok(())
}
