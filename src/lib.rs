use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "new-crate-project", version, about = "A small starter CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Print a greeting
    Greet {
        /// Name to greet
        #[arg(short, long, default_value = "world")]
        name: String,
    },
}

pub fn run(cli: Cli) -> Result<String> {
    let output = match cli.command {
        Some(Commands::Greet { name }) => format!("Hello, {name}!"),
        None => "new-crate-project is ready. Run with --help for usage.".to_string(),
    };
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_command_formats_message() {
        let out = run(Cli {
            command: Some(Commands::Greet {
                name: "Rod".to_string(),
            }),
        })
        .unwrap();
        assert_eq!(out, "Hello, Rod!");
    }

    #[test]
    fn default_message_without_subcommand() {
        let out = run(Cli { command: None }).unwrap();
        assert!(out.contains("ready"));
    }
}