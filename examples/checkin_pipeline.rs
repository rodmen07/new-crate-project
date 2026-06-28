use anyhow::Result;
use coachkit::{CheckinInput, CheckinStrategy, checkin_advice};

fn main() -> Result<()> {
    let input = CheckinInput {
        mood: 3,
        energy: 4,
        friction: Some("task switching".to_string()),
    };

    let advice = checkin_advice(&input);
    let ui_hint = match advice.strategy {
        CheckinStrategy::LowEnergy => "show_tiny_task_prompt",
        CheckinStrategy::FrictionUnblock => "show_unblock_step_prompt",
        CheckinStrategy::LowMood => "show_small_win_prompt",
        CheckinStrategy::FocusBlock => "show_focus_block_prompt",
    };

    let payload = serde_json::json!({
        "input": input,
        "advice": advice,
        "ui_hint": ui_hint,
    });

    println!("{}", serde_json::to_string_pretty(&payload)?);
    Ok(())
}
