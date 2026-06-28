use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use serde::Serialize;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Text,
    Json,
}

#[derive(Debug, Clone, Serialize)]
pub struct RunOutput {
    pub command: String,
    pub message: String,
}

#[derive(Debug, Parser)]
#[command(name = "new-crate-project", version, about = "A small starter CLI")]
pub struct Cli {
    /// Select output format
    #[arg(long, value_enum, default_value_t = OutputFormat::Text, global = true)]
    pub format: OutputFormat,

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
        /// Print the greeting in uppercase
        #[arg(long)]
        uppercase: bool,
    },
    /// Add integer values and print the total
    Sum {
        /// Values to add together
        #[arg(allow_hyphen_values = true)]
        values: Vec<i64>,
    },
    /// Print the crate version
    Version,
    /// Record a quick check-in and get a suggested next step
    Checkin {
        /// Mood score from 1 (low) to 5 (high)
        #[arg(long, value_parser = clap::value_parser!(u8).range(1..=5))]
        mood: u8,
        /// Energy score from 1 (low) to 5 (high)
        #[arg(long, value_parser = clap::value_parser!(u8).range(1..=5))]
        energy: u8,
        /// Optional friction note describing a blocker
        #[arg(long)]
        friction: Option<String>,
    },
}

fn next_step(mood: u8, energy: u8, friction: Option<&str>) -> String {
    if energy <= 2 {
        return "Keep it light: choose one 15-minute task and complete it first.".to_string();
    }

    if let Some(raw) = friction {
        let trimmed = raw.trim();
        if !trimmed.is_empty() {
            return format!("Start with a 10-minute unblock step on: {trimmed}.");
        }
    }

    if mood <= 2 {
        return "Aim for a small win first, then reassess your plan.".to_string();
    }

    "Pick your top priority and run one focused 25-minute block.".to_string()
}

pub fn run(cli: Cli) -> Result<RunOutput> {
    let (command, message) = match cli.command {
        Some(Commands::Greet { name, uppercase }) => {
            let greeting = format!("Hello, {name}!");
            (
                "greet".to_string(),
                if uppercase {
                    greeting.to_uppercase()
                } else {
                    greeting
                },
            )
        }
        Some(Commands::Sum { values }) => {
            let total: i64 = values.iter().sum();
            ("sum".to_string(), total.to_string())
        }
        Some(Commands::Version) => ("version".to_string(), env!("CARGO_PKG_VERSION").to_string()),
        Some(Commands::Checkin {
            mood,
            energy,
            friction,
        }) => (
            "checkin".to_string(),
            format!(
                "Check-in complete (mood {mood}/5, energy {energy}/5). {}",
                next_step(mood, energy, friction.as_deref())
            ),
        ),
        None => (
            "default".to_string(),
            "new-crate-project is ready. Run with --help for usage.".to_string(),
        ),
    };
    Ok(RunOutput { command, message })
}

pub fn render_output(out: &RunOutput, format: OutputFormat) -> Result<String> {
    let rendered = match format {
        OutputFormat::Text => out.message.clone(),
        OutputFormat::Json => serde_json::to_string_pretty(out)?,
    };
    Ok(rendered)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_command_formats_message() {
        let out = run(Cli {
            format: OutputFormat::Text,
            command: Some(Commands::Greet {
                name: "Rod".to_string(),
                uppercase: false,
            }),
        })
        .unwrap();
        assert_eq!(out.message, "Hello, Rod!");
        assert_eq!(out.command, "greet");
    }

    #[test]
    fn greet_command_uppercase_formats_message() {
        let out = run(Cli {
            format: OutputFormat::Text,
            command: Some(Commands::Greet {
                name: "Rod".to_string(),
                uppercase: true,
            }),
        })
        .unwrap();
        assert_eq!(out.message, "HELLO, ROD!");
    }

    #[test]
    fn default_message_without_subcommand() {
        let out = run(Cli {
            format: OutputFormat::Text,
            command: None,
        })
        .unwrap();
        assert!(out.message.contains("ready"));
    }

    #[test]
    fn sum_command_adds_values() {
        let out = run(Cli {
            format: OutputFormat::Text,
            command: Some(Commands::Sum {
                values: vec![2, 3, 5],
            }),
        })
        .unwrap();
        assert_eq!(out.message, "10");
    }

    #[test]
    fn sum_command_with_no_values_returns_zero() {
        let out = run(Cli {
            format: OutputFormat::Text,
            command: Some(Commands::Sum { values: vec![] }),
        })
        .unwrap();
        assert_eq!(out.message, "0");
    }

    #[test]
    fn version_command_prints_package_version() {
        let out = run(Cli {
            format: OutputFormat::Text,
            command: Some(Commands::Version),
        })
        .unwrap();
        assert_eq!(out.command, "version");
        assert_eq!(out.message, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn checkin_prefers_low_energy_guidance() {
        let out = run(Cli {
            format: OutputFormat::Text,
            command: Some(Commands::Checkin {
                mood: 4,
                energy: 2,
                friction: Some("email backlog".to_string()),
            }),
        })
        .unwrap();
        assert_eq!(out.command, "checkin");
        assert!(out.message.contains("Keep it light"));
    }

    #[test]
    fn checkin_uses_friction_when_energy_is_ok() {
        let out = run(Cli {
            format: OutputFormat::Text,
            command: Some(Commands::Checkin {
                mood: 3,
                energy: 4,
                friction: Some("context switching".to_string()),
            }),
        })
        .unwrap();
        assert!(out.message.contains("context switching"));
    }

    #[test]
    fn checkin_uses_small_win_when_mood_is_low() {
        let out = run(Cli {
            format: OutputFormat::Text,
            command: Some(Commands::Checkin {
                mood: 1,
                energy: 4,
                friction: None,
            }),
        })
        .unwrap();
        assert!(out.message.contains("small win"));
    }

    #[test]
    fn checkin_defaults_to_focus_block_when_stable() {
        let out = run(Cli {
            format: OutputFormat::Text,
            command: Some(Commands::Checkin {
                mood: 4,
                energy: 4,
                friction: None,
            }),
        })
        .unwrap();
        assert!(out.message.contains("focused 25-minute block"));
    }

    #[test]
    fn json_render_includes_command_and_message() {
        let out = RunOutput {
            command: "greet".to_string(),
            message: "Hello, Rod!".to_string(),
        };
        let rendered = render_output(&out, OutputFormat::Json).unwrap();
        assert!(rendered.contains("\"command\": \"greet\""));
        assert!(rendered.contains("\"message\": \"Hello, Rod!\""));
    }
}
