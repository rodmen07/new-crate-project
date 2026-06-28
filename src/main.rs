use clap::Parser;
use new_crate_project::{Cli, render_output, run};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let format = cli.format;
    let out_path = cli.out.clone();
    let output = run(cli)?;
    let output = render_output(&output, format)?;

    if let Some(path) = out_path {
        std::fs::write(&path, &output)?;
        eprintln!("wrote output to {}", path.display());
    }

    println!("{output}");
    Ok(())
}
