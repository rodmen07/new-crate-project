use clap::Parser;
use new_crate_project::{Cli, render_output, run};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let format = cli.format;
    let output = run(cli)?;
    let output = render_output(&output, format)?;
    println!("{output}");
    Ok(())
}
