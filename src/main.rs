use clap::Parser;
use coachkit::{Cli, OutputFormat, render_output, run};

fn write_to_dir(dir: &std::path::Path, output: &str, format: OutputFormat) -> anyhow::Result<()> {
    use std::time::{SystemTime, UNIX_EPOCH};

    std::fs::create_dir_all(dir)?;
    let ext = match format {
        OutputFormat::Text => "txt",
        OutputFormat::Json => "json",
    };
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let stamped = dir.join(format!("artifact-{now}.{ext}"));
    let latest = dir.join(format!("latest.{ext}"));

    std::fs::write(&stamped, output)?;
    std::fs::write(&latest, output)?;

    eprintln!("wrote output to {}", stamped.display());
    eprintln!("updated latest artifact {}", latest.display());
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let format = cli.format;
    let out_path = cli.out.clone();
    let out_dir = cli.out_dir.clone();
    let output = run(cli)?;
    let output = render_output(&output, format)?;

    if let Some(path) = out_path {
        std::fs::write(&path, &output)?;
        eprintln!("wrote output to {}", path.display());
    }

    if let Some(dir) = out_dir {
        write_to_dir(&dir, &output, format)?;
    }

    println!("{output}");
    Ok(())
}
