use anyhow::{Result, bail};
use coachkit::{
    CheckinInput, EffortLevel, PlanInput, build_day_plan_data, checkin_advice, render_day_plan,
};
use serde::Deserialize;
use serde_json::json;
use std::io::Read;

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum StdinRequest {
    Checkin {
        mood: u8,
        energy: u8,
        friction: Option<String>,
    },
    Plan {
        priorities: Vec<String>,
        stop: Option<String>,
        effort: Option<EffortLevel>,
        focus: Option<String>,
    },
}

fn main() -> Result<()> {
    let mut input_raw = String::new();
    std::io::stdin().read_to_string(&mut input_raw)?;

    if input_raw.trim().is_empty() {
        bail!("expected JSON payload on stdin");
    }

    let req: StdinRequest = serde_json::from_str(&input_raw)?;

    let payload = match req {
        StdinRequest::Checkin {
            mood,
            energy,
            friction,
        } => {
            if !(1..=5).contains(&mood) || !(1..=5).contains(&energy) {
                bail!("checkin mood and energy must be in range 1..=5");
            }

            let input = CheckinInput {
                mood,
                energy,
                friction,
            };
            let advice = checkin_advice(&input);

            json!({
                "request_type": "checkin",
                "input": input,
                "advice": advice,
            })
        }
        StdinRequest::Plan {
            priorities,
            stop,
            effort,
            focus,
        } => {
            let input = PlanInput {
                priorities,
                stop,
                effort: effort.unwrap_or(EffortLevel::Medium),
                focus,
            };
            let plan = build_day_plan_data(&input);
            let text_brief = render_day_plan(&plan);

            json!({
                "request_type": "plan",
                "input": input,
                "plan": plan,
                "text_brief": text_brief,
            })
        }
    };

    println!("{}", serde_json::to_string_pretty(&payload)?);
    Ok(())
}
