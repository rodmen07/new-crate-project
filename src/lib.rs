use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Text,
    Json,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum EffortLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize)]
pub struct RunOutput {
    pub command: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct CheckinInput {
    pub mood: u8,
    pub energy: u8,
    pub friction: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PlanInput {
    pub priorities: Vec<String>,
    pub stop: Option<String>,
    pub effort: EffortLevel,
    pub focus: Option<String>,
}

#[derive(Debug, Parser)]
#[command(name = "new-crate-project", version, about = "A small starter CLI")]
pub struct Cli {
    /// Select output format
    #[arg(long, value_enum, default_value_t = OutputFormat::Text, global = true)]
    pub format: OutputFormat,

    /// Write output to this file for downstream tooling (for example calm-daily-coach)
    #[arg(long, value_name = "FILE", global = true, conflicts_with = "out_dir")]
    pub out: Option<PathBuf>,

    /// Write output artifacts to this directory (timestamped + latest)
    #[arg(long, value_name = "DIR", global = true, conflicts_with = "out")]
    pub out_dir: Option<PathBuf>,

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
    /// Compile a practical day plan from your priorities
    Plan {
        /// Priority item (repeat up to 3)
        #[arg(long = "priority")]
        priorities: Vec<String>,
        /// Optional stop time goal (for example 17:30)
        #[arg(long)]
        stop: Option<String>,
        /// Intended effort for the day
        #[arg(long, value_enum, default_value_t = EffortLevel::Medium)]
        effort: EffortLevel,
        /// Optional focus note for today's intent
        #[arg(long)]
        focus: Option<String>,
    },
}

pub fn build_day_plan(input: &PlanInput) -> String {
    let mut normalized: Vec<String> = input
        .priorities
        .iter()
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .take(3)
        .map(str::to_string)
        .collect();

    if normalized.is_empty() {
        normalized.push("Pick one meaningful task and finish it.".to_string());
    }

    let effort_label = match input.effort {
        EffortLevel::Low => "low",
        EffortLevel::Medium => "medium",
        EffortLevel::High => "high",
    };

    let mut lines = vec!["Plan ready:".to_string()];
    for (idx, item) in normalized.iter().enumerate() {
        lines.push(format!("{}. {item}", idx + 1));
    }
    lines.push(format!("Effort: {effort_label}"));

    if let Some(stop_time) = input
        .stop
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        lines.push(format!("Stop target: {stop_time}"));
    }

    if let Some(focus_note) = input
        .focus
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        lines.push(format!("Focus: {focus_note}"));
    }

    lines.join("\n")
}

pub fn checkin_suggestion(input: &CheckinInput) -> String {
    if input.energy <= 2 {
        return "Keep it light: choose one 15-minute task and complete it first.".to_string();
    }

    if let Some(raw) = input.friction.as_deref() {
        let trimmed = raw.trim();
        if !trimmed.is_empty() {
            return format!("Start with a 10-minute unblock step on: {trimmed}.");
        }
    }

    if input.mood <= 2 {
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
        }) => {
            let input = CheckinInput {
                mood,
                energy,
                friction,
            };
            (
                "checkin".to_string(),
                format!(
                    "Check-in complete (mood {}/5, energy {}/5). {}",
                    input.mood,
                    input.energy,
                    checkin_suggestion(&input)
                ),
            )
        }
        Some(Commands::Plan {
            priorities,
            stop,
            effort,
            focus,
        }) => {
            let input = PlanInput {
                priorities,
                stop,
                effort,
                focus,
            };
            ("plan".to_string(), build_day_plan(&input))
        }
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

    fn cli(command: Option<Commands>) -> Cli {
        Cli {
            format: OutputFormat::Text,
            out: None,
            out_dir: None,
            command,
        }
    }

    #[test]
    fn greet_command_formats_message() {
        let out = run(cli(Some(Commands::Greet {
            name: "Rod".to_string(),
            uppercase: false,
        })))
        .unwrap();
        assert_eq!(out.message, "Hello, Rod!");
        assert_eq!(out.command, "greet");
    }

    #[test]
    fn greet_command_uppercase_formats_message() {
        let out = run(cli(Some(Commands::Greet {
            name: "Rod".to_string(),
            uppercase: true,
        })))
        .unwrap();
        assert_eq!(out.message, "HELLO, ROD!");
    }

    #[test]
    fn default_message_without_subcommand() {
        let out = run(cli(None)).unwrap();
        assert!(out.message.contains("ready"));
    }

    #[test]
    fn sum_command_adds_values() {
        let out = run(cli(Some(Commands::Sum {
            values: vec![2, 3, 5],
        })))
        .unwrap();
        assert_eq!(out.message, "10");
    }

    #[test]
    fn sum_command_with_no_values_returns_zero() {
        let out = run(cli(Some(Commands::Sum { values: vec![] }))).unwrap();
        assert_eq!(out.message, "0");
    }

    #[test]
    fn version_command_prints_package_version() {
        let out = run(cli(Some(Commands::Version))).unwrap();
        assert_eq!(out.command, "version");
        assert_eq!(out.message, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn checkin_prefers_low_energy_guidance() {
        let suggestion = checkin_suggestion(&CheckinInput {
            mood: 4,
            energy: 2,
            friction: Some("email backlog".to_string()),
        });
        assert!(suggestion.contains("Keep it light"));
    }

    #[test]
    fn checkin_uses_friction_when_energy_is_ok() {
        let suggestion = checkin_suggestion(&CheckinInput {
            mood: 3,
            energy: 4,
            friction: Some("context switching".to_string()),
        });
        assert!(suggestion.contains("context switching"));
    }

    #[test]
    fn checkin_uses_small_win_when_mood_is_low() {
        let suggestion = checkin_suggestion(&CheckinInput {
            mood: 1,
            energy: 4,
            friction: None,
        });
        assert!(suggestion.contains("small win"));
    }

    #[test]
    fn checkin_defaults_to_focus_block_when_stable() {
        let suggestion = checkin_suggestion(&CheckinInput {
            mood: 4,
            energy: 4,
            friction: None,
        });
        assert!(suggestion.contains("focused 25-minute block"));
    }

    #[test]
    fn plan_defaults_when_no_priority_provided() {
        let message = build_day_plan(&PlanInput {
            priorities: vec![],
            stop: None,
            effort: EffortLevel::Medium,
            focus: None,
        });
        assert!(message.contains("Plan ready:"));
        assert!(message.contains("Pick one meaningful task"));
    }

    #[test]
    fn plan_keeps_only_top_three_priorities() {
        let message = build_day_plan(&PlanInput {
            priorities: vec![
                "A".to_string(),
                "B".to_string(),
                "C".to_string(),
                "D".to_string(),
            ],
            stop: Some("17:30".to_string()),
            effort: EffortLevel::High,
            focus: Some("Finish what matters".to_string()),
        });
        assert!(message.contains("1. A"));
        assert!(message.contains("2. B"));
        assert!(message.contains("3. C"));
        assert!(!message.contains("D"));
        assert!(message.contains("Effort: high"));
        assert!(message.contains("Stop target: 17:30"));
        assert!(message.contains("Focus: Finish what matters"));
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
