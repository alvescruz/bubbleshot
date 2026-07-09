#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::similar_names
)]

mod capture;
mod ui;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = args.get(1).map_or("interactive", String::as_str);

    // If the mode is "screen", we use interactive(false)
    // This captures the entire screen without opening the selection dialog
    let interactive = !matches!(mode, "screen");

    println!("[theoshot] Mode: {mode} (Interactive: {interactive})");

    match capture::capture_frame(interactive).await {
        Ok(frame) => {
            ui::run_ui(Some(frame.data), frame.width, frame.height);
        }
        Err(e) => {
            eprintln!("[theoshot] Fatal error during capture: {e}");
        }
    }
}
