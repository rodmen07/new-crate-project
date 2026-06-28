use anyhow::Result;
use coachkit::{EffortLevel, PlanInput, build_day_plan_data, render_day_plan};

fn main() -> Result<()> {
    let input = PlanInput {
        priorities: vec![
            "Ship one focused feature".to_string(),
            "Write reflection".to_string(),
            "Trim backlog".to_string(),
        ],
        stop: Some("17:30".to_string()),
        effort: EffortLevel::Medium,
        focus: Some("Keep scope tight".to_string()),
    };

    let plan = build_day_plan_data(&input);

    // App side: keep typed data for UI state and render text where needed.
    let text_brief = render_day_plan(&plan);

    let payload = serde_json::json!({
        "plan": plan,
        "text_brief": text_brief,
    });

    println!("{}", serde_json::to_string_pretty(&payload)?);
    Ok(())
}
